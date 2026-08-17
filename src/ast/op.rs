use std::collections::BTreeMap;

use super::attr::Attribute;
use super::ty::Type;

/// A single SSA value id, without its `%` sigil.
pub type ValueId = String;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Region {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone)]
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
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericModule {
    pub type_aliases: BTreeMap<String, Type>,
    pub attr_aliases: BTreeMap<String, Attribute>,
    pub ops: Vec<Operation>,
}

impl GenericModule {
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
}
