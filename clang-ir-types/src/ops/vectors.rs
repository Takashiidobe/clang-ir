//! Vector operations.

/// `cir.vec.cmp`
/// Compare two vectors
///
/// The `cir.vec.cmp` operation does an element-wise comparison of two vectors
/// of the same type. The result is a vector of the same size as the operands
/// whose element type is the signed integral type that is the same size as the
/// element type of the operands. The values in the result are 0 or -1.
///
/// The optional `fenv` attribute describes constraints on the floating-point
/// handling of the operation. It is only valid for floating-point vector
/// comparisons.
///
/// ```
/// %eq = cir.vec.cmp(eq, %vec_a, %vec_b) : !cir.vector<4 x !s32i>, !cir.vector<4 x !s32i>
/// %lt = cir.vec.cmp(lt, %vec_a, %vec_b) : !cir.vector<4 x !s32i>, !cir.vector<4 x !s32i>
/// %gt = cir.vec.cmp(gt, %va, %vb) : !cir.vector<4 x !cir.float>, !cir.vector<4 x !s32i> fenv<>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VecCmp {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// compare operation kind
    pub kind: crate::enums::CmpOpKind,
    /// CIR vector type
    pub lhs: super::ValueId,
    /// CIR vector type
    pub rhs: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.vec.create`
/// Create a vector value
///
/// The `cir.vec.create` operation creates a vector value with the given element
/// values. The number of element arguments must match the number of elements
/// in the vector type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VecCreate {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// variadic of any cir boolean, integer, floating point
    pub elements: Vec<super::ValueId>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.vec.extract`
/// Extract one element from a vector object
///
/// The `cir.vec.extract` operation extracts the element at the given index
/// from a vector object.
///
/// ```
/// %tmp = cir.load %vec : !cir.ptr<!cir.vector<4 x !s32i>>, !cir.vector<4 x !s32i>
/// %idx = cir.const #cir.int<1> : !s32i
/// %element = cir.vec.extract %tmp[%idx : !s32i] : !cir.vector<4 x !s32i>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VecExtract {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// CIR vector type
    pub vec: super::ValueId,
    /// fundamental integer type
    pub index: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.vec.insert`
/// Insert one element into a vector object
///
/// The `cir.vec.insert` operation produces a new vector by replacing
/// the element of the input vector at `index` with `value`.
///
/// ```
/// %value = cir.const #cir.int<5> : !s32i
/// %index = cir.const #cir.int<2> : !s32i
/// %vec_tmp = cir.load %0 : !cir.ptr<!cir.vector<4 x !s32i>>, !cir.vector<4 x !s32i>
/// %new_vec = cir.vec.insert %value, %vec_tmp[%index : !s32i] : !cir.vector<4 x !s32i>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VecInsert {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// CIR vector type
    pub vec: super::ValueId,
    /// any cir boolean, integer, floating point
    pub value: super::ValueId,
    /// fundamental integer type
    pub index: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.vec.masked_load`
/// Masked vector load from memory
///
/// `cir.masked_load` conditionally loads elements from memory based on a mask.
/// Elements for which the mask is false are taken from `pass_thru`.
///
/// This operation corresponds to LLVM's masked load op (`llvm.intr.maskedload`)
/// and lower directly to it.
///
/// `alignment` can be provided to override the default alignment derived from
/// the pointee/element type data layout.
///
/// Example:
///
/// ```
/// %v = cir.masked_load align(16) %ptr, %mask, %passthru
///      : !cir.ptr<i32>, <4xi1>, <4xi32> -> <4xi32>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VecMaskedLoad {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// base address (points to element type)
    pub addr: super::ValueId,
    /// CIR vector type
    pub mask: super::ValueId,
    /// CIR vector type
    pub pass_thru: super::ValueId,
    /// 64-bit signless integer attribute whose value is positive and whose value is a power of two > 0
    pub alignment: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.vec.reduce.add`
/// Reduce a vector using add
///
/// Reduces the elements of a vector to a scalar of the vector element type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VecReduceAdd {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// vector of any of boolean type, integer type
    pub input: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.vec.reduce.and`
/// Reduce a vector using and
///
/// Reduces the elements of a vector to a scalar of the vector element type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VecReduceAnd {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// vector of any of boolean type, integer type
    pub input: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.vec.reduce.fadd`
/// Reduce a floating-point vector using fadd
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VecReduceFadd {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type
    pub accumulator: super::ValueId,
    /// vector of single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type
    pub input: super::ValueId,
    /// fast-math flags
    pub fastmath_flags: Option<crate::enums::FastMathFlags>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.vec.reduce.fmax`
/// Reduce a floating-point vector using fmax
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VecReduceFmax {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// vector of single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type
    pub input: super::ValueId,
    /// fast-math flags
    pub fastmath_flags: Option<crate::enums::FastMathFlags>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.vec.reduce.fmin`
/// Reduce a floating-point vector using fmin
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VecReduceFmin {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// vector of single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type
    pub input: super::ValueId,
    /// fast-math flags
    pub fastmath_flags: Option<crate::enums::FastMathFlags>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.vec.reduce.fmul`
/// Reduce a floating-point vector using fmul
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VecReduceFmul {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type
    pub accumulator: super::ValueId,
    /// vector of single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type
    pub input: super::ValueId,
    /// fast-math flags
    pub fastmath_flags: Option<crate::enums::FastMathFlags>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.vec.reduce.mul`
/// Reduce a vector using mul
///
/// Reduces the elements of a vector to a scalar of the vector element type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VecReduceMul {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// vector of any of boolean type, integer type
    pub input: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.vec.reduce.or`
/// Reduce a vector using or
///
/// Reduces the elements of a vector to a scalar of the vector element type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VecReduceOr {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// vector of any of boolean type, integer type
    pub input: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.vec.reduce.smax`
/// Reduce a vector using smax
///
/// Reduces the elements of a vector to a scalar of the vector element type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VecReduceSmax {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// vector of signed integer type
    pub input: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.vec.reduce.smin`
/// Reduce a vector using smin
///
/// Reduces the elements of a vector to a scalar of the vector element type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VecReduceSmin {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// vector of signed integer type
    pub input: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.vec.reduce.umax`
/// Reduce a vector using umax
///
/// Reduces the elements of a vector to a scalar of the vector element type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VecReduceUmax {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// vector of any of unsigned integer type, boolean type
    pub input: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.vec.reduce.umin`
/// Reduce a vector using umin
///
/// Reduces the elements of a vector to a scalar of the vector element type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VecReduceUmin {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// vector of any of unsigned integer type, boolean type
    pub input: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.vec.reduce.xor`
/// Reduce a vector using xor
///
/// Reduces the elements of a vector to a scalar of the vector element type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VecReduceXor {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// vector of any of boolean type, integer type
    pub input: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.vec.shuffle`
/// Combine two vectors using indices passed as constant integers
///
/// The `cir.vec.shuffle` operation implements the documented form of Clang's
/// `__builtin_shufflevector`, where the indices of the shuffled result are
/// integer constants.
///
/// The two input vectors, which must have the same type, are concatenated.
/// Each of the integer constant arguments is interpreted as an index into that
/// concatenated vector, with a value of -1 meaning that the result value
/// doesn't matter. The result vector, which must have the same element type as
/// the input vectors and the same number of elements as the list of integer
/// constant indices, is constructed by taking the elements at the given
/// indices from the concatenated vector. The size of the result vector does
/// not have to match the size of the individual input vectors or of the
/// concatenated vector.
///
/// ```
/// %new_vec = cir.vec.shuffle(%vec_1, %vec_2 : !cir.vector<2 x !s32i>)
///     [#cir.int<3> : !s64i, #cir.int<1> : !s64i] : !cir.vector<2 x !s32i>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VecShuffle {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// CIR vector type
    pub vec1: super::ValueId,
    /// CIR vector type
    pub vec2: super::ValueId,
    /// integer array attribute
    pub indices: crate::attrs::Attribute,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.vec.shuffle.dynamic`
/// Shuffle a vector using indices in another vector
///
/// The `cir.vec.shuffle.dynamic` operation implements the undocumented form of
/// Clang's __builtin_shufflevector, where the indices of the shuffled result
/// can be runtime values.
///
/// There are two input vectors, which must have the same number of elements.
/// The second input vector must have an integral element type. The elements of
/// the second vector are interpreted as indices into the first vector. The
/// result vector is constructed by taking the elements from the first input
/// vector from the indices indicated by the elements of the second vector.
///
/// ```
/// %new_vec = cir.vec.shuffle.dynamic %vec : !cir.vector<4 x !s32i>, %indices
///     : !cir.vector<4 x !s32i>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VecShuffleDynamic {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// CIR vector type
    pub vec: super::ValueId,
    /// vector of integer type
    pub indices: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.vec.splat`
/// Convert a scalar into a vector or matrix
///
/// The `cir.vec.splat` operation creates a vector or matrix value from a
/// scalar value. All elements of the result have the same value, that of the
/// given scalar.
///
/// It's a separate operation from `cir.vec.create` because more
/// efficient LLVM IR can be generated for it, and because some optimization and
/// analysis passes can benefit from knowing that all elements of the vector
/// have the same value.
///
/// ```
/// %value = cir.const #cir.int<3> : !s32i
/// %value_vec = cir.vec.splat %value : !s32i, !cir.vector<4 x !s32i>
/// %value_mat = cir.vec.splat %value : !s32i, !cir.matrix<2 x 2 x !s32i>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VecSplat {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// any cir boolean, integer, floating point
    pub value: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.vec.ternary`
/// The `cond ? a : b` ternary operator for vector types
///
/// The `cir.vec.ternary` operation represents the C/C++ ternary operator,
/// `?:`, for vector types, which does a `select` on individual elements of the
/// vectors. Unlike a regular `?:` operator, there is no short circuiting. All
/// three arguments are always evaluated.  Because there is no short
/// circuiting, there are no regions in this operation, unlike cir.ternary.
///
/// The first argument is a vector of integral type. The second and third
/// arguments are vectors of the same type and have the same number of elements
/// as the first argument.
///
/// The result is a vector of the same type as the second and third arguments.
/// Each element of the result is `(bool)a[n] ? b[n] : c[n]`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VecTernary {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// vector of any of boolean type, integer type
    pub cond: super::ValueId,
    /// CIR vector type
    pub lhs: super::ValueId,
    /// CIR vector type
    pub rhs: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}