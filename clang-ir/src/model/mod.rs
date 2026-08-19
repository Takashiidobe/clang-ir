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
pub use function::Function;
pub use global::Global;
pub use module::Module;
