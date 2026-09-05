//! C varargs operations.

/// `cir.va_arg`
/// Fetches next variadic element as a given type
///
/// The `cir.va_arg` operation models the C/C++ `va_arg` macro by reading the
/// next argument from an active variable argument list and producing it as a
/// value of a specified result type.
///
/// The operand must be a pointer to the target's `va_list` representation.
/// The operation advances the `va_list` state as a side effect and returns
/// the fetched value as the result, whose type is chosen by the user of the
/// operation.
///
/// A `cir.va_arg` must only be used on a `va_list` that has been initialized
/// with `cir.va.start` and not yet finalized by `cir.va.end`. The semantics
/// (including alignment and promotion rules) follow the platform ABI; the
/// frontend is responsible for providing a `va_list` pointer that matches the
/// target representation.
///
/// Example:
/// ```
/// // %args : !cir.ptr<!cir.array<!rec___va_list_tag x 1>>
/// %p = cir.cast array_to_ptrdecay %args
///         : !cir.ptr<!cir.array<!rec___va_list_tag x 1>>
///         -> !cir.ptr<!rec___va_list_tag>
/// cir.va.start %p : !cir.ptr<!rec___va_list_tag>
///
/// // Fetch an `int` from the vararg list.
/// %v = cir.va_arg %p : (!cir.ptr<!rec___va_list_tag>) -> !s32i
///
/// cir.va.end %p : !cir.ptr<!rec___va_list_tag>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VaArg {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// CIR pointer type
    pub arg_list: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.va_copy`
/// Copied a variable argument list
///
/// The `cir.copy` operation models the C/C++ va_copy macro.
/// The variable argument list passed as the `$src_list` is copied to an
/// unitialized `va_list` in the destination operand. The next argument that
/// can be extracted from the copied list is the same as the next argument in
/// the source list. The copied list must be destroyed with `va_end`.
///
/// Example:
///
/// ```
/// // %args : !cir.ptr<!cir.array<!rec___va_list_tag x 1>>
/// %p = cir.cast array_to_ptrdecay %args
///       : !cir.ptr<!cir.array<!rec___va_list_tag x 1>>
///       -> !cir.ptr<!rec___va_list_tag>
/// cir.va_copy %p to %dst
///       : (!cir.ptr<!rec___va_list_tag>, !cir.ptr<!rec___va_list_tag>)
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VaCopy {
    /// CIR pointer type
    pub dst_list: super::ValueId,
    /// CIR pointer type
    pub src_list: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.va_end`
/// Ends a variable argument list
///
/// The `cir.va_end` operation models the C/C++ va_end macro by finalizing
/// and cleaning up a variable argument list previously initialized with
/// `cir.va_start`.
///
/// The operand must be a pointer to the target's `va_list` representation.
/// This operation has no results and produces its effect by mutating the
/// storage referenced by the pointer operand.
///
/// `cir.va_end` must only be called after a matching `cir.va_start` on the
/// same `va_list` along all control-flow paths. After `cir.va_end`, the
/// `va_list` is invalid and must not be accessed unless reinitialized.
///
/// Lowering typically maps this to the LLVM intrinsic `llvm.va_end`,
/// passing the appropriately decayed pointer to the underlying `va_list`
/// storage.
///
/// Example:
/// ```
/// // %args : !cir.ptr<!cir.array<!rec___va_list_tag x 1>>
/// %p = cir.cast array_to_ptrdecay %args
///       : !cir.ptr<!cir.array<!rec___va_list_tag x 1>>
///       -> !cir.ptr<!rec___va_list_tag>
/// cir.va_end %p : !cir.ptr<!rec___va_list_tag>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VaEnd {
    /// CIR pointer type
    pub arg_list: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.va_start`
/// Starts a variable argument list
///
/// The cir.va_start operation models the C/C++ va_start macro by
/// initializing a variable argument list at the given va_list storage
/// location.
///
/// The operand must be a pointer to the target's `va_list` representation.
/// This operation has no results and produces its effect by mutating the
/// storage referenced by the pointer operand.
///
/// Each `cir.va_start` must be paired with a corresponding `cir.va_end`
/// on the same logical `va_list` object along all control-flow paths. After
/// `cir.va_end`, the `va_list` must not be accessed unless reinitialized
/// with another `cir.va_start`.
///
/// Lowering maps this to the LLVM intrinsic `llvm.va_start`, passing the
/// appropriately decayed pointer to the underlying `va_list` storage.
///
/// Example:
///
/// ```
/// // %args : !cir.ptr<!cir.array<!rec___va_list_tag x 1>>
/// %p = cir.cast array_to_ptrdecay %args
///       : !cir.ptr<!cir.array<!rec___va_list_tag x 1>>)
///       -> !cir.ptr<!rec___va_list_tag>
/// cir.va_start %p : !cir.ptr<!rec___va_list_tag>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VaStart {
    /// CIR pointer type
    pub arg_list: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}