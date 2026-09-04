use std::fmt;

use crate::ast::{Attribute, Operation, Type, ValueId};
use crate::enums::{CallingConv, GlobalLinkageKind, InlineKind, SideEffect, VisibilityKind};

/// One entry of a `"cir.target-features"` list, e.g. `+ssse3` or `-avx512f`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TargetFeature {
    pub name: String,
    pub enabled: bool,
}

fn parse_target_features(raw: &str) -> Vec<TargetFeature> {
    raw.split(',')
        .filter(|entry| !entry.is_empty())
        .filter_map(|entry| {
            let (enabled, name) = match entry.split_at_checked(1)? {
                ("+", name) => (true, name),
                ("-", name) => (false, name),
                _ => return None,
            };
            Some(TargetFeature {
                name: name.to_string(),
                enabled,
            })
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Function {
    pub name: String,
    pub params: Vec<(ValueId, Type)>,
    pub return_ty: Type,
    pub varargs: bool,
    pub linkage: GlobalLinkageKind,
    pub calling_conv: CallingConv,
    pub inline_kind: Option<InlineKind>,
    pub side_effect: Option<SideEffect>,
    pub visibility: Option<VisibilityKind>,
    pub aliasee: Option<String>,
    pub section: Option<String>,
    pub noreturn: bool,
    pub cold: bool,
    pub hot: bool,
    pub ctor_priority: Option<i64>,
    pub dtor_priority: Option<i64>,
    pub dso_local: bool,
    pub nothrow: bool,
    pub comdat: bool,
    pub target_cpu: Option<String>,
    pub target_features: Vec<TargetFeature>,
    /// Per-parameter dictionary attributes as written on the `cir.func` op,
    /// e.g. `llvm.byval` marking a large-struct-by-value parameter.
    pub arg_attrs: Option<Attribute>,
    /// `None` for a declaration (no defining region, e.g. an external
    /// library function like `printf`).
    pub body: Option<crate::ops::Region>,
}

impl Function {
    /// Builds a bare declaration with sensible defaults (external linkage,
    /// C calling convention, no body), for hand-constructing CIR rather than
    /// parsing it. Use the public fields (or struct-update syntax) to adjust
    /// anything else, e.g. `Function { body: Some(body), ..Function::new(..) }`.
    pub fn new(name: impl Into<String>, params: Vec<(ValueId, Type)>, return_ty: Type) -> Function {
        Function {
            name: name.into(),
            params,
            return_ty,
            varargs: false,
            linkage: GlobalLinkageKind::External,
            calling_conv: CallingConv::C,
            inline_kind: None,
            side_effect: None,
            visibility: None,
            aliasee: None,
            section: None,
            noreturn: false,
            cold: false,
            hot: false,
            ctor_priority: None,
            dtor_priority: None,
            dso_local: false,
            nothrow: false,
            comdat: false,
            target_cpu: None,
            target_features: Vec::new(),
            arg_attrs: None,
            body: None,
        }
    }

    pub fn from_op(op: &Operation) -> Result<Function, super::ModelError> {
        let name = op
            .attr("sym_name")
            .and_then(Attribute::as_str)
            .ok_or_else(|| super::invalid_attribute(op, "sym_name"))?
            .to_string();
        let function_type = op
            .attr("function_type")
            .and_then(Attribute::as_type)
            .ok_or_else(|| super::invalid_attribute(op, "function_type"))?;
        let (inputs, output, varargs) = match function_type {
            Type::Func {
                inputs,
                optional_return_type,
                var_arg,
            } => (
                inputs.clone(),
                optional_return_type
                    .as_deref()
                    .cloned()
                    .unwrap_or(Type::Void),
                *var_arg,
            ),
            _ => return Err(super::invalid_attribute(op, "function_type")),
        };

        let entry_args = op
            .regions
            .first()
            .and_then(|r| r.blocks.first())
            .map(|b| b.args.clone())
            .unwrap_or_default();
        let params = if entry_args.len() == inputs.len() {
            entry_args
                .into_iter()
                .zip(inputs.iter().cloned())
                .map(|((id, _), ty)| (id, ty))
                .collect()
        } else {
            // Declarations have no body/block args; fall back to the
            // function type's input list with synthetic names.
            inputs
                .iter()
                .enumerate()
                .map(|(i, ty)| (format!("arg{i}"), ty.clone()))
                .collect()
        };

        let body = op
            .regions
            .first()
            .filter(|r| !r.blocks.is_empty())
            .map(super::lower_region)
            .transpose()?;

        let linkage = op
            .attr("linkage")
            .and_then(|a| GlobalLinkageKind::try_from(a).ok())
            .unwrap_or(GlobalLinkageKind::External);
        let calling_conv = op
            .attr("calling_conv")
            .and_then(|a| CallingConv::try_from(a).ok())
            .unwrap_or(CallingConv::C);
        let inline_kind = op
            .attr("inline_kind")
            .and_then(|a| InlineKind::try_from(a).ok());
        let side_effect = op
            .attr("side_effect")
            .and_then(|a| SideEffect::try_from(a).ok());
        let visibility = op
            .attr("global_visibility")
            .and_then(|a| VisibilityKind::try_from(a).ok());
        let aliasee = super::symbol_ref_name(op.attr("aliasee"));
        let section = op
            .attr("section")
            .and_then(Attribute::as_str)
            .map(str::to_string);
        let noreturn = op.attr("noreturn").is_some();
        let cold = op.attr("cold").is_some();
        let hot = op.attr("hot").is_some();
        let ctor_priority = op
            .attr("global_ctor_priority")
            .and_then(Attribute::as_int)
            .and_then(|value| i64::try_from(value).ok());
        let dtor_priority = op
            .attr("global_dtor_priority")
            .and_then(Attribute::as_int)
            .and_then(|value| i64::try_from(value).ok());
        let dso_local = op.attr("dso_local").is_some();
        let nothrow = op.attr("nothrow").is_some();
        let comdat = op
            .attr("comdat")
            .is_some_and(|a| a.as_bool().unwrap_or(true));
        let target_cpu = op
            .attr("cir.target-cpu")
            .and_then(Attribute::as_str)
            .map(str::to_string);
        let target_features = op
            .attr("cir.target-features")
            .and_then(Attribute::as_str)
            .map(parse_target_features)
            .unwrap_or_default();
        let arg_attrs = op.attr("arg_attrs").cloned();

        Ok(Function {
            name,
            params,
            return_ty: output,
            varargs,
            linkage,
            calling_conv,
            inline_kind,
            side_effect,
            visibility,
            aliasee,
            section,
            noreturn,
            cold,
            hot,
            ctor_priority,
            dtor_priority,
            dso_local,
            nothrow,
            comdat,
            target_cpu,
            target_features,
            arg_attrs,
            body,
        })
    }

    pub fn is_declaration(&self) -> bool {
        self.body.is_none()
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fn {}(", self.name)?;
        for (i, (id, ty)) in self.params.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "%{id}: {ty}")?;
        }
        if self.varargs {
            if !self.params.is_empty() {
                write!(f, ", ")?;
            }
            write!(f, "...")?;
        }
        write!(f, ") -> {}", self.return_ty)?;

        let mut attrs = vec![self.linkage.to_string(), self.calling_conv.to_string()];
        if let Some(ik) = self.inline_kind {
            attrs.push(ik.to_string());
        }
        if let Some(se) = self.side_effect {
            attrs.push(se.to_string());
        }
        if self.dso_local {
            attrs.push("dso_local".to_string());
        }
        if self.nothrow {
            attrs.push("nothrow".to_string());
        }
        if self.comdat {
            attrs.push("comdat".to_string());
        }
        write!(f, " [{}]", attrs.join(", "))?;

        match &self.body {
            None => writeln!(f, ";"),
            Some(body) => {
                writeln!(f, " {{")?;
                crate::ops::write_region(f, body, 1)?;
                writeln!(f, "}}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Block, Region};

    #[test]
    fn parameter_types_come_from_function_signature() {
        let signature_ty = Type::Named("source_param".into());
        let body_ty = Type::Named("abi_param".into());
        let op = Operation {
            name: "cir.func".into(),
            results: Vec::new(),
            operands: Vec::new(),
            successors: Vec::new(),
            properties: vec![
                ("sym_name".into(), Attribute::Str("f".into())),
                (
                    "function_type".into(),
                    Attribute::Type(Type::Func {
                        inputs: vec![signature_ty.clone()],
                        optional_return_type: None,
                        var_arg: false,
                    }),
                ),
            ],
            regions: vec![Region {
                blocks: vec![Block {
                    label: None,
                    args: vec![("arg".into(), body_ty)],
                    ops: Vec::new(),
                }],
            }],
            attributes: Vec::new(),
            operand_types: Vec::new(),
            loc: None,
        };

        let function = Function::from_op(&op).unwrap();

        assert_eq!(function.params, vec![("arg".into(), signature_ty)]);
    }

    #[test]
    fn preserves_semantic_function_metadata() {
        let op = Operation {
            name: "cir.func".into(),
            results: Vec::new(),
            operands: Vec::new(),
            successors: Vec::new(),
            properties: vec![
                ("sym_name".into(), Attribute::Str("alias".into())),
                (
                    "function_type".into(),
                    Attribute::Type(Type::Func {
                        inputs: Vec::new(),
                        optional_return_type: None,
                        var_arg: false,
                    }),
                ),
                (
                    "global_visibility".into(),
                    Attribute::Int { ty: None, value: 2 },
                ),
                ("aliasee".into(), Attribute::SymbolRef("target".into())),
                ("section".into(), Attribute::Str("functions".into())),
                ("noreturn".into(), Attribute::Bool(true)),
                ("cold".into(), Attribute::Bool(true)),
                ("hot".into(), Attribute::Bool(true)),
                (
                    "global_ctor_priority".into(),
                    Attribute::Int {
                        ty: None,
                        value: 123,
                    },
                ),
                (
                    "global_dtor_priority".into(),
                    Attribute::Int {
                        ty: None,
                        value: 456,
                    },
                ),
            ],
            regions: Vec::new(),
            attributes: Vec::new(),
            operand_types: Vec::new(),
            loc: None,
        };

        let function = Function::from_op(&op).unwrap();

        assert_eq!(
            function.visibility,
            Some(crate::enums::VisibilityKind::Protected)
        );
        assert_eq!(function.aliasee.as_deref(), Some("target"));
        assert_eq!(function.section.as_deref(), Some("functions"));
        assert!(function.noreturn);
        assert!(function.cold);
        assert!(function.hot);
        assert_eq!(function.ctor_priority, Some(123));
        assert_eq!(function.dtor_priority, Some(456));
    }
}
