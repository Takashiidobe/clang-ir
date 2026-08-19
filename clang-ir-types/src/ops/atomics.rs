//! Atomic operations.

/// `cir.atomic.clear`
/// Atomic clear
///
/// C/C++ atomic clear operation. Implements the builtin function
/// `__atomic_clear`.
///
/// The operation takes as its only operand a pointer to an 8-bit signed
/// integer. The operation atomically sets the integer to zero.
///
/// Example:
/// ```
///   cir.atomic.clear seq_cst %ptr : !cir.ptr<!s8i>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AtomicClear {
    pub ptr: super::ValueId,
    /// memory order
    pub mem_order: crate::enums::MemOrder,
    /// 64-bit signless integer attribute
    pub alignment: Option<crate::attrs::Attribute>,
    /// unit attribute
    pub is_volatile: bool,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.atomic.cmpxchg`
/// Atomic compare and exchange
///
/// C/C++ atomic compare and exchange operation. Implements builtins like
/// `__atomic_compare_exchange_n` and `__atomic_compare_exchange`.
///
/// This operation takes three arguments: a pointer `ptr` and two values
/// `expected` and `desired`. This operation compares the value of the object
/// pointed-to by `ptr` with `expected`, and if they are equal, it sets the
/// value of the object to `desired`.
///
/// The `succ_order` attribute gives the memory order of this atomic operation
/// when the exchange takes place. The `fail_order` attribute gives the memory
/// order of this atomic operation when the exchange does not take place.
///
/// The `sync_scope` attribute specifies the synchronization scope for this
/// atomic operation.
///
/// The `weak` attribute is a boolean flag that indicates whether this is a
/// "weak" compare-and-exchange operation. A weak compare-and-exchange operation
/// allows "spurious failures", meaning that be treated as if the comparison
/// failed and not exchange values even if `*ptr` and `expected` indeed compare
/// equal.
///
/// The type of `expected` and `desired` must be the same. The pointee type of
/// `ptr` must be the same as the type of `expected` and `desired`.
///
/// This operation has two results. The first result `old` gives the old value
/// of the object pointed-to by `ptr`, regardless of whether the exchange
/// actually took place. The second result `success` is a boolean flag
/// indicating whether the exchange actually took place.
///
/// Example:
///
/// ```
/// %old, %success = cir.atomic.cmpxchg weak success(seq_cst) failure(acquire)
///     syncscope(system) %ptr, %expected, %desired
///     : (!cir.ptr<!u64i>, !u64i, !u64i) -> (!u64i, !cir.bool)
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AtomicCmpxchg {
    pub old: super::ValueId,
    pub old_ty: crate::types::Type,
    pub success: super::ValueId,
    pub success_ty: crate::types::Type,
    pub ptr: super::ValueId,
    /// CIR void type or CIR bool type or CIR array type or CIR vector type or Integer type with arbitrary precision up to a fixed limit or single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type or CIR pointer type or CIR function type or CIR struct/class type or CIR union type or CIR complex type or CIR type that is used for the vptr member of C++ objects or CIR type that represents a pointer-to-data-member in C++ or CIR type that represents C++ pointer-to-member-function type or CIR exception handling token type or CIR cleanup token type or CIR catch token type
    pub expected: super::ValueId,
    /// CIR void type or CIR bool type or CIR array type or CIR vector type or Integer type with arbitrary precision up to a fixed limit or single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type or CIR pointer type or CIR function type or CIR struct/class type or CIR union type or CIR complex type or CIR type that is used for the vptr member of C++ objects or CIR type that represents a pointer-to-data-member in C++ or CIR type that represents C++ pointer-to-member-function type or CIR exception handling token type or CIR cleanup token type or CIR catch token type
    pub desired: super::ValueId,
    /// success memory order
    pub succ_order: crate::enums::MemOrder,
    /// failure memory order
    pub fail_order: crate::enums::MemOrder,
    /// sync scope kind
    pub sync_scope: crate::enums::SyncScopeKind,
    /// 64-bit signless integer attribute
    pub alignment: Option<crate::attrs::Attribute>,
    /// unit attribute
    pub weak: bool,
    /// unit attribute
    pub is_volatile: bool,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.atomic.fence`
/// Atomic thread fence
///
/// C/C++ Atomic thread fence synchronization primitive. Implements the builtin
/// `__atomic_thread_fence` which enforces memory ordering constraints across
/// threads within the specified synchronization scope.
///
/// This handles all variations including:
///   - `__atomic_thread_fence`
///   - `__atomic_signal_fence`
///   - `__c11_atomic_thread_fence`
///   - `__c11_atomic_signal_fence`
///
/// Example:
/// ```
///   cir.atomic.fence syncscope(system) seq_cst
///   cir.atomic.fence syncscope(single_thread) seq_cst
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AtomicFence {
    /// memory order
    pub ordering: crate::enums::MemOrder,
    /// sync scope kind
    pub syncscope: Option<crate::enums::SyncScopeKind>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.atomic.fetch`
/// Atomic fetch-and-update operation
///
/// C/C++ atomic fetch-and-update operation. This operation implements the C/C++
/// builtin functions `__atomic_<binop>_fetch`, `__atomic_fetch_<binop>`, and
/// `__c11_atomic_fetch_<binop>`, where `<binop>` is one of the following binary
/// opcodes: `add`, `sub`, `and`, `xor`, `or`, `nand`, `max`, `min`,
/// `uinc_wrap`, `udec_wrap`, `maximum`, `minimum`, `maximum_num`, and
/// `minimum_num`.
///
/// This operation takes 2 arguments: a pointer `ptr` and a value `val`. The
/// type of `val` must match the pointee type of `ptr`. If the binary operation
/// is `add`, `sub`, `max`, or `min`, the type of `val` may either be an integer
/// type or a floating-point type. Otherwise, `val` must be an integer.
///
/// This operation atomically loads the value from `ptr`, performs the binary
/// operation as indicated by `binop` on the loaded value and `val`, and stores
/// the result back to `ptr`. If the `fetch_first` flag is present, the result
/// of this operation is the old value loaded from `ptr` before the binary
/// operation. Otherwise, the result of this operation is the result of the
/// binary operation.
///
/// The primary difference between `max`, `maximum`, and `maximum_num` is how
/// they handle floating-point inputs:
///
/// - `max` on floating-point inputs corresponds to the `atomicrmw fmax` LLVM
///   instruction.
/// - `maximum` corresponds to the `atomicrmw fmaximum` LLVM instruction.
/// - `maximum_num` corresponds to the `atomicrmw fmaximumnum` LLVM instruction.
///
/// Similar rules apply to `min`, `minimum`, and `minimum_num`. See the
/// reference for the LLVM instruction `atomicrmw` for more information.
///
/// The operation `maximum`, `minimum`, `maximum_num`, and `minimum_num` only
/// accept floating-point inputs.
///
/// Example:
/// %res = cir.atomic.fetch add seq_cst %ptr, %val
///     : (!cir.ptr<!s32i>, !s32i) -> !s32i
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AtomicFetch {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    pub ptr: super::ValueId,
    /// integer or floating point type
    pub val: super::ValueId,
    /// Binary opcode for atomic fetch-and-update operations
    pub binop: crate::enums::AtomicFetchKind,
    /// memory order
    pub mem_order: crate::enums::MemOrder,
    /// synchronization scope
    pub sync_scope: crate::enums::SyncScopeKind,
    /// unit attribute
    pub is_volatile: bool,
    /// unit attribute
    pub fetch_first: bool,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.atomic.test_and_set`
/// Atomic test and set
///
/// C/C++ atomic test and set operation. Implements the builtin function
/// `__atomic_test_and_set`.
///
/// The operation takes as its only operand a pointer to an 8-bit signed
/// integer. The operation atomically set the integer to an implementation-
/// defined non-zero "set" value. The result of the operation is a boolean value
/// indicating whether the previous value of the integer was the "set" value.
///
/// Example:
/// ```
///   %res = cir.atomic.test_and_set seq_cst %ptr : !cir.ptr<!s8i> -> !cir.bool
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AtomicTestAndSet {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    pub ptr: super::ValueId,
    /// memory order
    pub mem_order: crate::enums::MemOrder,
    /// 64-bit signless integer attribute
    pub alignment: Option<crate::attrs::Attribute>,
    /// unit attribute
    pub is_volatile: bool,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.atomic.xchg`
/// Atomic exchange
///
/// C/C++ atomic exchange operation. This operation implements the C/C++
/// builtin function `__atomic_exchange`, `__atomic_exchange_n`, and
/// `__c11_atomic_exchange`.
///
/// This operation takes two arguments: a pointer `ptr` and a value `val`. The
/// operation atomically replaces the value of the object pointed-to by `ptr`
/// with `val`, and returns the original value of the object.
///
/// Example:
///
/// ```
/// %res = cir.atomic.xchg seq_cst %ptr, %val : !cir.ptr<!u64i> -> !u64i
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AtomicXchg {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    pub ptr: super::ValueId,
    /// CIR void type or CIR bool type or CIR array type or CIR vector type or Integer type with arbitrary precision up to a fixed limit or single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type or CIR pointer type or CIR function type or CIR struct/class type or CIR union type or CIR complex type or CIR type that is used for the vptr member of C++ objects or CIR type that represents a pointer-to-data-member in C++ or CIR type that represents C++ pointer-to-member-function type or CIR exception handling token type or CIR cleanup token type or CIR catch token type
    pub val: super::ValueId,
    /// memory order
    pub mem_order: crate::enums::MemOrder,
    /// sync scope kind
    pub sync_scope: crate::enums::SyncScopeKind,
    /// unit attribute
    pub is_volatile: bool,
    pub loc: Option<crate::ast::SourceLocation>,
}