pub mod function;
pub mod global;
pub mod module;

pub use crate::enums;
pub use crate::enums::{
    CallingConv, CaseOpKind, CastKind, CmpOpKind, GlobalLinkageKind, InlineKind, MemOrder,
    SideEffect, SourceLanguage, SyncScopeKind, TlsModel, VisibilityKind,
};
pub use crate::ops as instruction;
pub use crate::ops::Op;
pub use function::{Function, TargetFeature};
pub use global::Global;
pub use module::Module;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ModelError {
    #[error("cannot convert `{operation}`: missing or invalid `{attribute}`")]
    InvalidAttribute {
        operation: String,
        attribute: &'static str,
    },
    #[error("cannot convert known CIR operation `{operation}`")]
    InvalidOperation { operation: String },
    #[error("generic module has no builtin.module operation")]
    MissingModule,
}

pub(crate) fn invalid_attribute(op: &crate::ast::Operation, attribute: &'static str) -> ModelError {
    ModelError::InvalidAttribute {
        operation: op.name.clone(),
        attribute,
    }
}

pub(crate) fn lower_region(region: &crate::ast::Region) -> Result<crate::ops::Region, ModelError> {
    validate_region(region)?;
    Ok(crate::ops::lower_region(region))
}

fn validate_region(region: &crate::ast::Region) -> Result<(), ModelError> {
    for block in &region.blocks {
        for op in &block.ops {
            validate_operation(op)?;
        }
    }
    Ok(())
}

fn validate_operation(op: &crate::ast::Operation) -> Result<(), ModelError> {
    if op.name.starts_with("cir.") && crate::ops::Op::from_operation(op).is_none() {
        return Err(ModelError::InvalidOperation {
            operation: op.name.clone(),
        });
    }
    for region in &op.regions {
        validate_region(region)?;
    }
    Ok(())
}

fn symbol_ref_name(attr: Option<&crate::ast::Attribute>) -> Option<String> {
    let crate::ast::Attribute::SymbolRef(value) = attr? else {
        return None;
    };
    Some(value.trim_start_matches('@').trim_matches('"').to_string())
}

#[cfg(test)]
mod tests {
    use crate::ast::{Attribute, Operation, Type};
    use crate::ops::Op;

    #[test]
    fn preserves_semantic_instruction_metadata() {
        let call = Operation {
            name: "cir.call".into(),
            operands: vec!["arg".into()],
            properties: vec![
                ("callee".into(), Attribute::SymbolRef("callee".into())),
                ("noreturn".into(), Attribute::Unit),
                ("side_effect".into(), Attribute::Int { value: 0, ty: None }),
            ],
            operand_types: vec![Type::Int {
                width: 32,
                is_signed: true,
                is_bit_int: false,
            }],
            results: Vec::new(),
            successors: Vec::new(),
            regions: Vec::new(),
            attributes: Vec::new(),
            loc: None,
        };
        let member = Operation {
            name: "cir.get_member".into(),
            results: vec![(
                "member".into(),
                Type::Pointer {
                    pointee: Box::new(Type::Int {
                        width: 32,
                        is_signed: true,
                        is_bit_int: false,
                    }),
                    addr_space: None,
                },
            )],
            operands: vec!["record".into()],
            properties: vec![
                ("name".into(), Attribute::Str("field".into())),
                ("index_attr".into(), Attribute::Int { value: 0, ty: None }),
            ],
            operand_types: vec![Type::Pointer {
                pointee: Box::new(Type::Struct {
                    members: None,
                    name: Some("struct.Record".into()),
                    incomplete: true,
                    packed: false,
                    member_kinds: Vec::new(),
                    is_class: false,
                }),
                addr_space: None,
            }],
            successors: Vec::new(),
            regions: Vec::new(),
            attributes: Vec::new(),
            loc: None,
        };

        let Some(Op::Call(call)) = Op::from_operation(&call) else {
            panic!("call did not lower")
        };
        assert!(call.noreturn);
        assert_eq!(call.arg_types.len(), 1);
        let Some(Op::GetMember(member)) = Op::from_operation(&member) else {
            panic!("get_member did not lower")
        };
        assert!(matches!(member.addr_ty, Type::Pointer { .. }));
    }
}
