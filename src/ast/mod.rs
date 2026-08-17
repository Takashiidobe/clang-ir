//! Dialect-agnostic representation of generic MLIR textual IR, specialized
//! with structural interpretations for the CIR dialect's types/attributes.
//!
//! This is the "generic" layer: it faithfully represents whatever `cir-opt
//! --mlir-print-op-generic` printed, including constructs we haven't
//! hand-modeled (see [`ty::Type::Dialect`] / [`attr::Attribute::Dialect`]).
//! The [`crate::model`] module builds the friendlier, CIR-semantic typed
//! representation on top of this.

pub mod attr;
pub mod op;
pub mod ty;

pub use attr::{Attribute, ConstArrayData};
pub use op::{Block, Module, Operation, Region, ValueId};
pub use ty::{FloatKind, RecordKind, RecordMemberKind, StructType, Type};
