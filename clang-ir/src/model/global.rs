use std::fmt;

use crate::ast::{Attribute, Operation, Type};
use crate::enums::{GlobalLinkageKind, TlsModel, VisibilityKind};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Global {
    pub name: String,
    pub ty: Type,
    pub linkage: GlobalLinkageKind,
    pub visibility: Option<VisibilityKind>,
    pub dso_local: bool,
    pub constant: bool,
    pub comdat: bool,
    pub alignment: Option<u64>,
    pub tls_model: Option<TlsModel>,
    /// `None` for an `extern` declaration with no initializer.
    pub initial_value: Option<Attribute>,
    /// The original operation, if this `Global` came from parsing rather
    /// than being hand-built — an escape hatch for any attribute not
    /// decoded above.
    pub raw: Option<Operation>,
}

impl Global {
    /// Builds a bare `extern` declaration with sensible defaults (external
    /// linkage, no initializer), for hand-constructing CIR rather than
    /// parsing it. Use the public fields (or struct-update syntax) to adjust
    /// anything else, e.g. `Global { constant: true, ..Global::new(..) }`.
    pub fn new(name: impl Into<String>, ty: Type) -> Global {
        Global {
            name: name.into(),
            ty,
            linkage: GlobalLinkageKind::External,
            visibility: None,
            dso_local: false,
            constant: false,
            comdat: false,
            alignment: None,
            tls_model: None,
            initial_value: None,
            raw: None,
        }
    }

    /// Builds a `Global` from a `cir.global` operation. Only fails if the
    /// op is missing its name or value type, which real CIR output never
    /// does — everything else defaults sensibly.
    pub fn from_op(op: &Operation) -> Option<Global> {
        let name = op.attr("sym_name").and_then(Attribute::as_str)?.to_string();
        let ty = op.attr("sym_type").and_then(Attribute::as_type)?.clone();

        let linkage = op
            .attr("linkage")
            .and_then(|a| GlobalLinkageKind::try_from(a).ok())
            .unwrap_or(GlobalLinkageKind::External);
        let visibility = op
            .attr("global_visibility")
            .and_then(|a| VisibilityKind::try_from(a).ok());
        let dso_local = op.attr("dso_local").is_some();
        let constant = op
            .attr("constant")
            .is_some_and(|a| a.as_bool().unwrap_or(true));
        let comdat = op
            .attr("comdat")
            .is_some_and(|a| a.as_bool().unwrap_or(true));
        let alignment = op
            .attr("alignment")
            .and_then(Attribute::as_int)
            .map(|v| v as u64);
        let tls_model = op
            .attr("tls_model")
            .and_then(|a| TlsModel::try_from(a).ok());
        let initial_value = op.attr("initial_value").cloned();

        Some(Global {
            name,
            ty,
            linkage,
            visibility,
            dso_local,
            constant,
            comdat,
            alignment,
            tls_model,
            initial_value,
            raw: Some(op.clone()),
        })
    }

    pub fn is_declaration(&self) -> bool {
        self.initial_value.is_none()
    }
}

impl fmt::Display for Global {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "global {} : {}", self.name, self.ty)?;
        if let Some(v) = &self.initial_value {
            write!(f, " = {v:?}")?;
        }

        let mut attrs = vec![self.linkage.to_string()];
        if let Some(vis) = self.visibility {
            attrs.push(vis.to_string());
        }
        if let Some(tm) = self.tls_model {
            attrs.push(tm.to_string());
        }
        if let Some(a) = self.alignment {
            attrs.push(format!("align {a}"));
        }
        if self.dso_local {
            attrs.push("dso_local".to_string());
        }
        if self.constant {
            attrs.push("constant".to_string());
        }
        if self.comdat {
            attrs.push("comdat".to_string());
        }
        writeln!(f, " [{}]", attrs.join(", "))
    }
}
