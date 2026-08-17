use std::fmt;

use crate::ast::attr::ConstArrayData;
use crate::ast::{Attribute, Block, Module as GenericModule, Operation, Region};
use crate::model::enums::SourceLanguage;
use crate::model::function::Function;
use crate::model::global::Global;

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Module {
    /// The module's `sym_name`: clang IR names the module after the source
    /// file path it was compiled from.
    pub name: Option<String>,
    pub source_language: Option<SourceLanguage>,
    pub triple: Option<String>,
    pub functions: Vec<Function>,
    pub globals: Vec<Global>,
    /// Top-level operations that were neither `cir.func` nor `cir.global`
    /// (nothing in practice today, but kept for forward-compatibility).
    pub other: Vec<Operation>,
    /// The full generic parse, for anything not surfaced above (module-level
    /// attributes like `dlti.dl_spec`, target datalayout, etc). Empty for a
    /// hand-built `Module` rather than one produced by parsing.
    pub generic: GenericModule,
}

/// Inlines `Attribute::Named` alias references (`bitfield_info = #bfi_a`,
/// `value = #true`, ...) throughout `op` and its nested regions, recursively.
/// The `ast` layer deliberately keeps these unresolved (faithful to the
/// source text, and `!`-type aliases can be self-referential in ways
/// attribute aliases aren't - see [`ast::Module::resolve_type`]), but the
/// `model` layer's [`crate::model::instruction::try_lower`] only ever sees a
/// bare `Operation` with no access to `generic.attr_aliases`, so aliases must
/// be inlined before lowering or every alias-referenced attribute silently
/// fails to structurally match and falls back to [`crate::model::Instruction::Other`].
fn resolve_op(op: &Operation, generic: &GenericModule) -> Operation {
    Operation {
        name: op.name.clone(),
        results: op.results.clone(),
        operands: op.operands.clone(),
        successors: op.successors.clone(),
        properties: op
            .properties
            .iter()
            .map(|(k, v)| (k.clone(), resolve_attribute(v, generic)))
            .collect(),
        regions: op
            .regions
            .iter()
            .map(|r| resolve_region(r, generic))
            .collect(),
        attributes: op
            .attributes
            .iter()
            .map(|(k, v)| (k.clone(), resolve_attribute(v, generic)))
            .collect(),
        operand_types: op.operand_types.clone(),
        loc: op.loc.clone(),
    }
}

fn resolve_region(region: &Region, generic: &GenericModule) -> Region {
    Region {
        blocks: region
            .blocks
            .iter()
            .map(|b| Block {
                label: b.label.clone(),
                args: b.args.clone(),
                ops: b.ops.iter().map(|op| resolve_op(op, generic)).collect(),
            })
            .collect(),
    }
}

fn resolve_attribute(attr: &Attribute, generic: &GenericModule) -> Attribute {
    if let Attribute::Named(name) = attr {
        // Aliases are defined before use in `cir-opt` output, but chase
        // alias-to-alias chains regardless; the `seen` guard mirrors
        // `ast::Module::resolve_type`'s cycle break (falls back to the
        // unresolved reference rather than looping forever).
        let mut current_name = name.clone();
        let mut seen = std::collections::HashSet::new();
        loop {
            if !seen.insert(current_name.clone()) {
                return attr.clone();
            }
            match generic.attr_aliases.get(&current_name) {
                Some(Attribute::Named(next)) => current_name = next.clone(),
                Some(resolved) => return resolve_attribute(resolved, generic),
                None => return attr.clone(),
            }
        }
    }
    match attr {
        Attribute::Array(items) => Attribute::Array(
            items
                .iter()
                .map(|a| resolve_attribute(a, generic))
                .collect(),
        ),
        Attribute::Dict(entries) => Attribute::Dict(
            entries
                .iter()
                .map(|(k, v)| (k.clone(), resolve_attribute(v, generic)))
                .collect(),
        ),
        Attribute::ConstArray {
            data: ConstArrayData::Elements(items),
            trailing_zeros,
            ty,
        } => Attribute::ConstArray {
            data: ConstArrayData::Elements(
                items
                    .iter()
                    .map(|a| resolve_attribute(a, generic))
                    .collect(),
            ),
            trailing_zeros: *trailing_zeros,
            ty: ty.clone(),
        },
        Attribute::ConstVector { elements, ty } => Attribute::ConstVector {
            elements: elements
                .iter()
                .map(|a| resolve_attribute(a, generic))
                .collect(),
            ty: ty.clone(),
        },
        Attribute::ConstRecord { elements, ty } => Attribute::ConstRecord {
            elements: elements
                .iter()
                .map(|a| resolve_attribute(a, generic))
                .collect(),
            ty: ty.clone(),
        },
        Attribute::ConstComplex { real, imag, ty } => Attribute::ConstComplex {
            real: Box::new(resolve_attribute(real, generic)),
            imag: Box::new(resolve_attribute(imag, generic)),
            ty: ty.clone(),
        },
        other => other.clone(),
    }
}

impl Module {
    /// An empty module, for hand-constructing CIR rather than parsing it.
    /// Add functions/globals directly via the public fields, e.g.
    /// `module.functions.push(Function::new(...))`.
    pub fn new(name: impl Into<String>) -> Module {
        Module {
            name: Some(name.into()),
            ..Module::default()
        }
    }

    pub fn from_generic(generic: GenericModule) -> Module {
        let module_op = generic.ops.iter().find(|op| op.name == "builtin.module");

        let name = module_op
            .and_then(|op| op.attr("sym_name"))
            .and_then(Attribute::as_str)
            .map(str::to_string);
        let source_language = module_op
            .and_then(|op| op.attr("cir.lang"))
            .and_then(|a| SourceLanguage::try_from(a).ok());
        let triple = module_op
            .and_then(|op| op.attr("cir.triple"))
            .and_then(Attribute::as_str)
            .map(str::to_string);

        let mut functions = Vec::new();
        let mut globals = Vec::new();
        let mut other = Vec::new();

        let body_ops = module_op
            .and_then(|op| op.regions.first())
            .and_then(|r| r.blocks.first())
            .map(|b| b.ops.as_slice());

        for op in body_ops.unwrap_or_default() {
            let op = resolve_op(op, &generic);
            match op.mnemonic() {
                "func" => match Function::from_op(&op) {
                    Some(f) => functions.push(f),
                    None => other.push(op),
                },
                "global" => match Global::from_op(&op) {
                    Some(g) => globals.push(g),
                    None => other.push(op),
                },
                _ => other.push(op),
            }
        }

        Module {
            name,
            source_language,
            triple,
            functions,
            globals,
            other,
            generic,
        }
    }

    pub fn function(&self, name: &str) -> Option<&Function> {
        self.functions.iter().find(|f| f.name == name)
    }

    pub fn global(&self, name: &str) -> Option<&Global> {
        self.globals.iter().find(|g| g.name == name)
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "module")?;
        if let Some(name) = &self.name {
            write!(f, " {name:?}")?;
        }
        let mut attrs = Vec::new();
        if let Some(lang) = self.source_language {
            attrs.push(lang.to_string());
        }
        if let Some(triple) = &self.triple {
            attrs.push(triple.clone());
        }
        if !attrs.is_empty() {
            write!(f, " ({})", attrs.join(", "))?;
        }
        writeln!(f)?;

        for g in &self.globals {
            write!(f, "{g}")?;
        }
        if !self.globals.is_empty() {
            writeln!(f)?;
        }
        for func in &self.functions {
            write!(f, "{func}")?;
        }
        if !self.other.is_empty() {
            writeln!(f, "; {} unmodeled top-level op(s)", self.other.len())?;
        }
        Ok(())
    }
}
