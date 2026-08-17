use std::fmt;

use crate::ast::{Attribute, GenericModule, Operation};
use crate::model::enums::SourceLanguage;
use crate::model::function::Function;
use crate::model::global::Global;

#[derive(Clone, Default)]
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
            match op.mnemonic() {
                "func" => match Function::from_op(op) {
                    Some(f) => functions.push(f),
                    None => other.push(op.clone()),
                },
                "global" => match Global::from_op(op) {
                    Some(g) => globals.push(g),
                    None => other.push(op.clone()),
                },
                _ => other.push(op.clone()),
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

impl fmt::Debug for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
