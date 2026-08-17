use std::fmt;

use crate::ast::{Attribute, Operation, Type, ValueId};
use crate::model::enums::{CallingConv, GlobalLinkageKind, InlineKind, SideEffect};
use crate::model::instruction::{Body, lower_region, write_body};

#[derive(Clone)]
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
    pub dso_local: bool,
    pub nothrow: bool,
    pub comdat: bool,
    /// `None` for a declaration (no defining region, e.g. an external
    /// library function like `printf`).
    pub body: Option<Body>,
    /// The original operation, if this `Function` came from parsing rather
    /// than being hand-built — an escape hatch for any attribute not
    /// decoded above.
    pub raw: Option<Operation>,
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
            dso_local: false,
            nothrow: false,
            comdat: false,
            body: None,
            raw: None,
        }
    }

    /// Builds a `Function` from a `cir.func` operation. Only fails (returns
    /// `None`) if the op is missing its name or function-type, which real
    /// CIR output never does — everything else defaults sensibly.
    pub fn from_op(op: &Operation) -> Option<Function> {
        let name = op.attr("sym_name").and_then(Attribute::as_str)?.to_string();
        let (inputs, output, varargs) = match op.attr("function_type")?.as_type()? {
            Type::CirFunc {
                inputs,
                output,
                varargs,
            } => (inputs.clone(), (**output).clone(), *varargs),
            _ => return None,
        };

        let entry_args = op
            .regions
            .first()
            .and_then(|r| r.blocks.first())
            .map(|b| b.args.clone())
            .unwrap_or_default();
        let params = if entry_args.len() == inputs.len() {
            entry_args
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
            .map(lower_region);

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
        let dso_local = op.attr("dso_local").is_some();
        let nothrow = op.attr("nothrow").is_some();
        let comdat = op
            .attr("comdat")
            .is_some_and(|a| a.as_bool().unwrap_or(true));

        Some(Function {
            name,
            params,
            return_ty: output,
            varargs,
            linkage,
            calling_conv,
            inline_kind,
            side_effect,
            dso_local,
            nothrow,
            comdat,
            body,
            raw: Some(op.clone()),
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
                write_body(body, f, 1)?;
                writeln!(f, "}}")
            }
        }
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
