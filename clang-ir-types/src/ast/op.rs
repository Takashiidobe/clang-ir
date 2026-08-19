use std::collections::BTreeMap;

use crate::attrs::Attribute;
use crate::types::Type;

/// A single SSA value id, without its `%` sigil.
pub type ValueId = String;

/// A source location attached to an operation.
///
/// MLIR locations have a grammar of their own (`fused`, `callsite`,
/// aliases, `unknown`). We structurally interpret the common
/// file-line-column form and keep every other location payload verbatim in
/// [`SourceLocation::Loc`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SourceLocation {
    File {
        file: String,
        line: u32,
        column: u32,
    },
    Fused(Vec<SourceLocation>),
    Loc(String),
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Operation {
    /// Full dialect-qualified mnemonic, e.g. `"cir.alloca"` or `"builtin.module"`.
    pub name: String,
    pub results: Vec<(ValueId, Type)>,
    pub operands: Vec<ValueId>,
    /// Block successors (`^bb1` labels, without the `^`), for terminator-like ops.
    pub successors: Vec<String>,
    /// The `<{...}>` inherent-attribute dict, if present.
    pub properties: Vec<(String, Attribute)>,
    pub regions: Vec<Region>,
    /// The trailing `{...}` discardable-attribute dict, if present.
    pub attributes: Vec<(String, Attribute)>,
    /// The operand types from the op's `: (operand-types) -> result-types`
    /// function-type signature, in operand order. Not derivable from
    /// `operands` alone: operand `ValueId`s carry no type of their own, only
    /// the producing op's `results` do, and for cross-block/region operands
    /// (e.g. a `cir.cast` fed by a block argument) that producer isn't always
    /// locally available.
    pub operand_types: Vec<Type>,
    /// Trailing `loc(...)`, structurally parsed where possible.
    pub loc: Option<SourceLocation>,
}

impl Operation {
    pub fn attr(&self, key: &str) -> Option<&Attribute> {
        self.properties
            .iter()
            .chain(self.attributes.iter())
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }

    pub fn dialect(&self) -> &str {
        self.name
            .split_once('.')
            .map(|(d, _)| d)
            .unwrap_or(&self.name)
    }

    pub fn mnemonic(&self) -> &str {
        self.name
            .split_once('.')
            .map(|(_, m)| m)
            .unwrap_or(&self.name)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Region {
    pub blocks: Vec<Block>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Block {
    /// `None` for an unlabeled entry block (only the first block in a region
    /// may omit its label).
    pub label: Option<String>,
    pub args: Vec<(ValueId, Type)>,
    pub ops: Vec<Operation>,
}

/// The result of parsing a whole `.mlir`/generic-CIR text file: top-level
/// type/attribute aliases plus the (usually singular) top-level operation(s).
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Module {
    pub type_aliases: BTreeMap<String, Type>,
    pub attr_aliases: BTreeMap<String, Attribute>,
    /// `#locN = loc(...)` alias definitions.
    pub loc_aliases: BTreeMap<String, SourceLocation>,
    pub ops: Vec<Operation>,
}

impl Module {
    /// Follows a `Type::Named` chain to its underlying definition. Returns
    /// `None` if the alias is undefined (never loops forever: CIR record
    /// aliases may be self-referential through a `Ptr`/`Array` indirection,
    /// but a bare `Named -> Named -> ...` cycle with no indirection would be
    /// a malformed input, not a valid recursive type).
    pub fn resolve_type<'a>(&'a self, ty: &'a Type) -> &'a Type {
        let mut current = ty;
        let mut seen = std::collections::HashSet::new();
        while let Type::Named(name) = current {
            if !seen.insert(name.clone()) {
                break;
            }
            match self.type_aliases.get(name) {
                Some(next) => current = next,
                None => break,
            }
        }
        current
    }

    /// Follows an `Attribute::Named` chain to its underlying definition.
    pub fn resolve_attr<'a>(&'a self, attr: &'a Attribute) -> &'a Attribute {
        let mut current = attr;
        let mut seen = std::collections::HashSet::new();
        while let Attribute::Named(name) = current {
            if !seen.insert(name.clone()) {
                break;
            }
            match self.attr_aliases.get(name) {
                Some(next) => current = next,
                None => break,
            }
        }
        current
    }

    /// Resolves every `Attribute::Named` reachable from `self.ops`
    /// against `self.attr_aliases`, replacing each reference in place its referenced value.
    pub fn resolve_named_attrs(&mut self) {
        let aliases = self.attr_aliases.clone();
        for op in &mut self.ops {
            resolve_op_attrs(op, &aliases);
        }
    }
}

fn resolve_op_attrs(op: &mut Operation, aliases: &BTreeMap<String, Attribute>) {
    for (_, attr) in op.properties.iter_mut().chain(op.attributes.iter_mut()) {
        resolve_attr_in_place(attr, aliases);
    }
    for region in &mut op.regions {
        for block in &mut region.blocks {
            for nested in &mut block.ops {
                resolve_op_attrs(nested, aliases);
            }
        }
    }
}

fn resolve_attr_in_place(attr: &mut Attribute, aliases: &BTreeMap<String, Attribute>) {
    if let Attribute::Named(name) = attr
        && let Some(resolved) = resolve_alias_chain(name, aliases)
    {
        *attr = resolved;
    }
    match attr {
        Attribute::Array(items) => {
            for item in items {
                resolve_attr_in_place(item, aliases);
            }
        }
        Attribute::Dict(entries) => {
            for (_, v) in entries {
                resolve_attr_in_place(v, aliases);
            }
        }
        Attribute::ConstVector { elts, .. } | Attribute::ConstRecord { members: elts, .. } => {
            resolve_attr_in_place(elts, aliases);
        }
        Attribute::ConstComplex { real, imag, .. } => {
            resolve_attr_in_place(real, aliases);
            resolve_attr_in_place(imag, aliases);
        }
        Attribute::ConstArray { elts, .. } => {
            resolve_attr_in_place(elts, aliases);
        }
        _ => {}
    }
}

fn resolve_alias_chain(name: &str, aliases: &BTreeMap<String, Attribute>) -> Option<Attribute> {
    let mut seen = std::collections::HashSet::new();
    let mut current = name.to_string();
    loop {
        if !seen.insert(current.clone()) {
            return None;
        }
        match aliases.get(&current) {
            Some(Attribute::Named(next)) => current = next.clone(),
            Some(other) => return Some(other.clone()),
            None => return None,
        }
    }
}
