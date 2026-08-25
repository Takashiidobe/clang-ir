use std::collections::{BTreeMap, HashSet};
use std::fmt;

use crate::ast::{Attribute, Module as GenericModule, Type};
use crate::enums::SourceLanguage;
use crate::model::function::Function;
use crate::model::global::Global;
use crate::ops::Op;

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Module {
    pub name: Option<String>,
    pub source_language: Option<SourceLanguage>,
    pub triple: Option<String>,
    pub module_asm: Vec<String>,
    pub functions: Vec<Function>,
    pub globals: Vec<Global>,
    pub other: Vec<Op>,
    pub type_aliases: BTreeMap<String, Type>,
    pub attr_aliases: BTreeMap<String, Attribute>,
}

impl Module {
    pub fn new(name: impl Into<String>) -> Module {
        Module {
            name: Some(name.into()),
            ..Module::default()
        }
    }

    pub fn from_generic(generic: GenericModule) -> Result<Module, super::ModelError> {
        let GenericModule {
            type_aliases,
            attr_aliases,
            ops,
            ..
        } = generic;
        let module_op = ops
            .iter()
            .find(|op| op.name == "builtin.module")
            .ok_or(super::ModelError::MissingModule)?;

        let name = module_op
            .attr("sym_name")
            .and_then(Attribute::as_str)
            .map(str::to_string);
        let source_language = module_op
            .attr("cir.lang")
            .and_then(|a| SourceLanguage::try_from(a).ok());
        let triple = module_op
            .attr("cir.triple")
            .and_then(Attribute::as_str)
            .map(str::to_string);
        let module_asm = module_op
            .attr("cir.module_asm")
            .and_then(|attr| match attr {
                Attribute::Array(values) => Some(values),
                _ => None,
            })
            .map(|values| {
                values
                    .iter()
                    .filter_map(Attribute::as_str)
                    .map(str::to_string)
                    .collect()
            })
            .unwrap_or_default();

        let mut functions = Vec::new();
        let mut globals = Vec::new();
        let mut other = Vec::new();

        let body_ops = module_op
            .regions
            .first()
            .and_then(|r| r.blocks.first())
            .map(|b| b.ops.as_slice());

        for op in body_ops.unwrap_or_default() {
            match op.mnemonic() {
                "func" => functions.push(Function::from_op(op)?),
                "global" => globals.push(Global::from_op(op)?),
                _ => match Op::from_operation(op) {
                    Some(op) => other.push(op),
                    None if op.name.starts_with("cir.") => {
                        return Err(super::ModelError::InvalidOperation {
                            operation: op.name.clone(),
                        });
                    }
                    None => other.push(Op::Other(op.clone())),
                },
            }
        }

        Ok(Module {
            name,
            source_language,
            triple,
            module_asm,
            functions,
            globals,
            other,
            type_aliases,
            attr_aliases,
        })
    }

    pub fn function(&self, name: &str) -> Option<&Function> {
        self.functions.iter().find(|f| f.name == name)
    }

    pub fn global(&self, name: &str) -> Option<&Global> {
        self.globals.iter().find(|g| g.name == name)
    }

    pub fn resolve_type<'a>(&'a self, ty: &'a Type) -> &'a Type {
        let mut current = ty;
        let mut seen = HashSet::new();
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

    pub fn resolve_attr<'a>(&'a self, attr: &'a Attribute) -> &'a Attribute {
        let mut current = attr;
        let mut seen = HashSet::new();
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
            for op in &self.other {
                writeln!(f, "{op}")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Block, Operation, Region, Type};

    fn empty_operation(name: &str) -> Operation {
        Operation {
            name: name.into(),
            results: Vec::new(),
            operands: Vec::new(),
            successors: Vec::new(),
            properties: Vec::new(),
            regions: Vec::new(),
            attributes: Vec::new(),
            operand_types: Vec::new(),
            loc: None,
        }
    }

    #[test]
    fn preserves_module_assembly() {
        let generic = GenericModule {
            ops: vec![Operation {
                name: "builtin.module".into(),
                results: Vec::new(),
                operands: Vec::new(),
                successors: Vec::new(),
                properties: vec![(
                    "cir.module_asm".into(),
                    Attribute::Array(vec![Attribute::Str(".globl symbol".into())]),
                )],
                regions: vec![Region {
                    blocks: vec![Block::default()],
                }],
                attributes: Vec::new(),
                operand_types: Vec::new(),
                loc: None,
            }],
            ..GenericModule::default()
        };

        let module = Module::from_generic(generic).unwrap();

        assert_eq!(module.module_asm, vec![".globl symbol"]);
    }

    #[test]
    fn preserves_and_resolves_normalized_aliases() {
        let mut generic = GenericModule::default();
        generic.type_aliases.insert("word".into(), Type::Bool);
        generic
            .attr_aliases
            .insert("enabled".into(), Attribute::Bool(true));
        generic.ops.push(Operation {
            name: "builtin.module".into(),
            results: Vec::new(),
            operands: Vec::new(),
            successors: Vec::new(),
            properties: Vec::new(),
            regions: Vec::new(),
            attributes: Vec::new(),
            operand_types: Vec::new(),
            loc: None,
        });

        let module = Module::from_generic(generic).unwrap();

        assert_eq!(module.type_aliases.get("word"), Some(&Type::Bool));
        assert_eq!(
            module.resolve_type(&Type::Named("word".into())),
            &Type::Bool
        );
        assert_eq!(
            module.resolve_attr(&Attribute::Named("enabled".into())),
            &Attribute::Bool(true)
        );
    }

    #[test]
    fn converted_operations_are_not_retained_as_other() {
        let function = Operation {
            name: "cir.func".into(),
            results: Vec::new(),
            operands: Vec::new(),
            successors: Vec::new(),
            properties: vec![
                ("sym_name".into(), Attribute::Str("f".into())),
                (
                    "function_type".into(),
                    Attribute::Type(Type::Func {
                        inputs: Vec::new(),
                        optional_return_type: None,
                        var_arg: false,
                    }),
                ),
            ],
            regions: Vec::new(),
            attributes: Vec::new(),
            operand_types: Vec::new(),
            loc: None,
        };
        let global = Operation {
            name: "cir.global".into(),
            results: Vec::new(),
            operands: Vec::new(),
            successors: Vec::new(),
            properties: vec![
                ("sym_name".into(), Attribute::Str("g".into())),
                ("sym_type".into(), Attribute::Type(Type::Bool)),
            ],
            regions: Vec::new(),
            attributes: Vec::new(),
            operand_types: Vec::new(),
            loc: None,
        };
        let generic = GenericModule {
            ops: vec![Operation {
                name: "builtin.module".into(),
                results: Vec::new(),
                operands: Vec::new(),
                successors: Vec::new(),
                properties: Vec::new(),
                regions: vec![Region {
                    blocks: vec![Block {
                        ops: vec![function, global],
                        ..Block::default()
                    }],
                }],
                attributes: Vec::new(),
                operand_types: Vec::new(),
                loc: None,
            }],
            ..GenericModule::default()
        };

        let module = Module::from_generic(generic).unwrap();

        assert_eq!(module.functions.len(), 1);
        assert_eq!(module.globals.len(), 1);
        assert!(module.other.is_empty());
    }

    #[test]
    fn rejects_malformed_known_definitions() {
        let generic = GenericModule {
            ops: vec![Operation {
                name: "builtin.module".into(),
                results: Vec::new(),
                operands: Vec::new(),
                successors: Vec::new(),
                properties: Vec::new(),
                regions: vec![Region {
                    blocks: vec![Block {
                        ops: vec![Operation {
                            name: "cir.func".into(),
                            results: Vec::new(),
                            operands: Vec::new(),
                            successors: Vec::new(),
                            properties: Vec::new(),
                            regions: Vec::new(),
                            attributes: Vec::new(),
                            operand_types: Vec::new(),
                            loc: None,
                        }],
                        ..Block::default()
                    }],
                }],
                attributes: Vec::new(),
                operand_types: Vec::new(),
                loc: None,
            }],
            ..GenericModule::default()
        };

        let error = Module::from_generic(generic).unwrap_err();

        assert!(matches!(
            error,
            super::super::ModelError::InvalidAttribute {
                operation,
                attribute: "sym_name"
            } if operation == "cir.func"
        ));
    }

    #[test]
    fn rejects_malformed_known_body_operations() {
        let mut function = empty_operation("cir.func");
        function.properties = vec![
            ("sym_name".into(), Attribute::Str("f".into())),
            (
                "function_type".into(),
                Attribute::Type(Type::Func {
                    inputs: Vec::new(),
                    optional_return_type: None,
                    var_arg: false,
                }),
            ),
        ];
        function.regions = vec![Region {
            blocks: vec![Block {
                ops: vec![empty_operation("cir.add")],
                ..Block::default()
            }],
        }];
        let mut module_op = empty_operation("builtin.module");
        module_op.regions = vec![Region {
            blocks: vec![Block {
                ops: vec![function],
                ..Block::default()
            }],
        }];

        let error = Module::from_generic(GenericModule {
            ops: vec![module_op],
            ..GenericModule::default()
        })
        .unwrap_err();

        assert_eq!(
            error,
            super::super::ModelError::InvalidOperation {
                operation: "cir.add".into()
            }
        );
    }

    #[test]
    fn preserves_unknown_dialect_operations() {
        let unknown = empty_operation("example.unknown");
        let mut module_op = empty_operation("builtin.module");
        module_op.regions = vec![Region {
            blocks: vec![Block {
                ops: vec![unknown],
                ..Block::default()
            }],
        }];

        let module = Module::from_generic(GenericModule {
            ops: vec![module_op],
            ..GenericModule::default()
        })
        .unwrap();

        assert!(matches!(
            module.other.as_slice(),
            [Op::Other(op)] if op.name == "example.unknown"
        ));
    }
}
