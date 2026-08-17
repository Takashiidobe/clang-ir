//! The CIR-semantic typed layer, built on top of [`crate::ast`]'s generic
//! operation tree.

pub mod enums;
pub mod function;
pub mod global;
pub mod instruction;
pub mod module;

pub use enums::{
    CallingConv, CaseOpKind, CastKind, CmpOpKind, GlobalLinkageKind, InlineKind, MemOrder,
    SideEffect, SourceLanguage, SyncScopeKind, TlsModel, VisibilityKind,
};
pub use function::Function;
pub use global::Global;
pub use instruction::{BinaryOp, Body, Callee, InstBlock, Instruction, SwitchCase, UnaryOp};
pub use module::Module;
