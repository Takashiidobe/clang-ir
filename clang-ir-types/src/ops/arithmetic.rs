//! Arithmetic, comparison, and bit-manipulation operations.

/// `cir.abs`
/// Computes the absolute value of a signed integer
///
/// `cir.abs` computes the absolute value of a signed integer or vector
/// of signed integers.
///
/// The `min_is_poison` attribute indicates whether the result value is a
/// poison value if the argument is statically or dynamically the minimum
/// value for the type.
///
/// Example:
///
/// ```
///   %0 = cir.const #cir.int<-42> : s32i
///   %1 = cir.abs %0 min_is_poison : s32i
///   %2 = cir.abs %3 : !cir.vector<!s32i x 4>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Abs {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// signed integer or vector of signed integer type
    pub src: super::ValueId,
    /// unit attribute
    pub min_is_poison: bool,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.acos`
/// Computes the arcus cosine of the specified value
///
/// `cir.acos`computes the arcus cosine of a given value and
/// returns a result of the same type.
///
/// Floating-point exceptions are ignored, and it does not set `errno`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Acos {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub src: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.add`
/// Integer addition
///
/// The `cir.add` operation performs addition on integer operands. Both
/// operands and the result must have the same integer or vector-of-integer
/// type.
///
/// The optional `nsw` (no signed wrap) and `nuw` (no unsigned wrap) unit
/// attributes indicate that the result is poison if signed or unsigned
/// overflow occurs, respectively. The optional `sat` (saturated) attribute
/// clamps the result to the type's representable range instead of wrapping.
/// The `nsw`/`nuw` flags and `sat` are mutually exclusive.
///
/// Example:
///
/// ```
/// %0 = cir.add %a, %b : !s32i
/// %1 = cir.add nsw %a, %b : !s32i
/// %2 = cir.add nuw %a, %b : !u32i
/// %3 = cir.add sat %a, %b : !s32i
/// %4 = cir.add %va, %vb : !cir.vector<4 x !s32i>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Add {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// integer or vector of integer type
    pub lhs: super::ValueId,
    /// integer or vector of integer type
    pub rhs: super::ValueId,
    /// unit property
    pub no_signed_wrap: bool,
    /// unit property
    pub no_unsigned_wrap: bool,
    /// unit property
    pub saturated: bool,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.and`
/// Bitwise AND
///
/// The `cir.and` operation performs a bitwise AND on integer operands.
/// Both operands and the result must have the same integer type.
///
/// Example:
///
/// ```
/// %0 = cir.and %a, %b : !s32i
/// %1 = cir.and %a, %b : !cir.bool
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct And {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// integer, boolean, or vector of bool or integer
    pub lhs: super::ValueId,
    /// integer, boolean, or vector of bool or integer
    pub rhs: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.asin`
/// Computes the arcus sine of the specified value
///
/// `cir.asin`computes the arcus sine of a given value and
/// returns a result of the same type.
///
/// Floating-point exceptions are ignored, and it does not set `errno`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Asin {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub src: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.atan`
/// Computes the floating-point arcus tangent value
///
/// `cir.atan` computes the arcus tangent of a floating-point operand
/// and returns a result of the same type.
///
/// Floating-point exceptions are ignored, and it does not set `errno`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Atan {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub src: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.atan2`
/// Computes the arc tangent of y/x
///
/// `cir.atan2` computes the arc tangent of the first operand divided by the
/// second operand, using the signs of both to determine the quadrant.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Atan2 {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub lhs: super::ValueId,
    /// floating point or vector of floating point type
    pub rhs: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.byte_swap`
/// Reverse the bytes in the object representation of the operand
///
/// The `cir.byte_swap` operation takes an integer as operand, reverse the bytes
/// in the object representation of the operand integer, and returns the result.
///
/// The operand integer must be an unsigned integer whose width is a multiple of
/// 16 bits (e.g. 16, 32, 64, 128, or a wider `_BitInt`).
///
/// Example:
///
/// ```
/// // %0 = 0x12345678
/// %0 = cir.const #cir.int<305419896> : !u32i
///
/// // %1 should be 0x78563412
/// %1 = cir.byte_swap %0 : !u32i
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ByteSwap {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// unsigned integer type with a width that is a multiple of 16 bits
    pub input: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.ceil`
/// Computes the ceiling of the specified value
///
/// `cir.ceil` computes the ceiling of a given value and returns a result
/// of the same type.
///
/// Floating-point exceptions are ignored, and it does not set `errno`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ceil {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub src: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.cmp`
/// Compare two values and produce a boolean result
///
/// The `cir.cmp` operation compares two operands of the same type and produces
/// a `!cir.bool` result. It supports integral, boolean, floating-point, and
/// pointer types.  Booleans (including enums with a boolean underlying type)
/// are compared as unsigned integers.
///
/// The following comparison predicates are available:
///
/// - `lt`: less than
/// - `le`: less than or equal
/// - `gt`: greater than
/// - `ge`: greater than or equal
/// - `eq`: equal
/// - `ne`: not equal
/// - `one`: ordered and not equal (floating-point only)
/// - `uno`: unordered (floating-point only, true if either operand is NaN)
///
/// For floating-point comparisons, the predicate follows C semantics (e.g.
/// NaN comparisons return false for all predicates except `ne`).
/// The `one` and `uno` predicates are floating-point specific: `one` is
/// ordered not-equal (false for NaN), `uno` tests if either operand is NaN.
///
/// The optional `fenv` attribute describes constraints on the floating-point
/// handling of the operation. It is only valid for floating-point
/// comparisons.
///
/// ```
/// %0 = cir.cmp gt %1, %2 : !s32i
/// %1 = cir.cmp eq %a, %b : !cir.ptr<!u8i>
/// %2 = cir.cmp lt %x, %y : !cir.float {fenv = #cir.fenv<>}
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cmp {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// compare operation kind
    pub kind: crate::enums::CmpOpKind,
    /// comparable type
    pub lhs: super::ValueId,
    /// comparable type
    pub rhs: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.cmp3way`
/// Compare two values with C++ three-way comparison semantics
///
/// The `cir.cmp3way` operation models the builtin `<=>` operator in C++20.
/// It takes two operands with the same type and produces a result indicating
/// the ordering between the two input operands.
///
/// The result of the operation is a signed integer that indicates the ordering
/// between the two input operands.
///
/// There are three kinds of ordering: strong, weak and partial ordering.
/// Comparing different types of values yields different kinds of orderings.
/// The `info` parameter gives the ordering kind and other necessary information
/// about the comparison.
///
/// Example:
///
/// ```
/// !s32i = !cir.int<s, 32>
///
/// #cmpinfo_partial_ltn1eq0gt1unn127 =
///   #cir.cmp3way_info<partial, lt = -1, eq = 0, gt = 1, unordered = -127>
/// #cmpinfo_strong_ltn1eq0gt1 =
///   #cir.cmp3way_info<strong, lt = -1, eq = 0, gt = 1>
///
/// %0 = cir.const #cir.int<0> : !s32i
/// %1 = cir.const #cir.int<1> : !s32i
/// %2 = cir.cmp3way #cmpinfo_strong_ltn1eq0gt1 %0, %1 : !s32i -> !s8i
///
/// %3 = cir.const #cir.fp<0.0> : !cir.float
/// %4 = cir.const #cir.fp<1.0> : !cir.float
/// %5 = cir.cmp3way #cmpinfo_partial_ltn1eq0gt1unn127 %3, %4 : !cir.float -> !s8i
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cmp3way {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// CIR void type or CIR bool type or CIR array type or CIR vector type or Integer type with arbitrary precision up to a fixed limit or single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type or CIR pointer type or CIR function type or CIR struct/class type or CIR union type or CIR complex type or CIR type that is used for the vptr member of C++ objects or CIR type that represents a pointer-to-data-member in C++ or CIR type that represents C++ pointer-to-member-function type or CIR exception handling token type or CIR cleanup token type or CIR catch token type
    pub lhs: super::ValueId,
    /// CIR void type or CIR bool type or CIR array type or CIR vector type or Integer type with arbitrary precision up to a fixed limit or single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type or CIR pointer type or CIR function type or CIR struct/class type or CIR union type or CIR complex type or CIR type that is used for the vptr member of C++ objects or CIR type that represents a pointer-to-data-member in C++ or CIR type that represents C++ pointer-to-member-function type or CIR exception handling token type or CIR cleanup token type or CIR catch token type
    pub rhs: super::ValueId,
    /// Holds information about a three-way comparison operation
    pub info: crate::attrs::Attribute,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.copysign`
/// Copies the sign of a floating-point value
///
/// `cir.copysign` returns a value with the magnitude of the first operand
/// and the sign of the second operand.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Copysign {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub lhs: super::ValueId,
    /// floating point or vector of floating point type
    pub rhs: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.cos`
/// Computes the floating-point cosine value
///
/// `cir.cos` computes the cosine of a floating-point operand and returns
/// a result of the same type.
///
/// Floating-point exceptions are ignored, and it does not set `errno`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cos {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub src: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.cosh`
/// Computes the floating-point hyperbolic cosine value
///
/// `cir.cosh` computes the hyperbolic cosine of a floating-point operand and
/// returns a result of the same type.
///
/// Floating-point exceptions are ignored, and it does not set `errno`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cosh {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub src: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.div`
/// Integer division
///
/// The `cir.div` operation performs division on integer operands. Both
/// operands and the result must have the same integer or vector-of-integer
/// type.
///
/// Example:
///
/// ```
/// %0 = cir.div %a, %b : !s32i
/// %1 = cir.div %a, %b : !u32i
/// %2 = cir.div %va, %vb : !cir.vector<4 x !s32i>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Div {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// integer or vector of integer type
    pub lhs: super::ValueId,
    /// integer or vector of integer type
    pub rhs: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.exp`
/// Computes the floating-point base-e exponential value
///
/// `cir.exp` computes the exponential of a floating-point operand and returns
/// a result of the same type.
///
/// Floating-point exceptions are ignored, and it does not set `errno`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Exp {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub src: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.exp10`
/// Computes the floating-point base-10 exponential value
///
/// `cir.exp10` computes the base-10 exponential of a floating-point operand and
/// returns a result of the same type.
///
/// Floating-point exceptions are ignored, and it does not set `errno`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Exp10 {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub src: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.exp2`
/// Computes the floating-point base-2 exponential value
///
/// `cir.exp2` computes the base-2 exponential of a floating-point operand and
///  returns a result of the same type.
///
/// Floating-point exceptions are ignored, and it does not set `errno`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Exp2 {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub src: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.fabs`
/// Computes the floating-point absolute value
///
/// `cir.fabs` computes the absolute value of a floating-point operand
/// and returns a result of the same type.
///
/// Floating-point exceptions are ignored, and it does not set `errno`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fabs {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub src: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.fadd`
/// Floating-point addition
///
/// The `cir.fadd` operation performs floating-point addition on its operands.
/// Both operands and the result must have the same floating-point scalar or
/// vector-of-float type.
///
/// Example:
///
/// ```
/// %0 = cir.fadd %a, %b : !cir.float
/// %1 = cir.fadd %a, %b : !cir.double
/// %2 = cir.fadd %va, %vb : !cir.vector<4 x !cir.float>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fadd {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub lhs: super::ValueId,
    /// floating point or vector of floating point type
    pub rhs: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.fdiv`
/// Floating-point division
///
/// The `cir.fdiv` operation performs floating-point division on its operands.
/// Both operands and the result must have the same floating-point scalar or
/// vector-of-float type.
///
/// Example:
///
/// ```
/// %0 = cir.fdiv %a, %b : !cir.float
/// %1 = cir.fdiv %a, %b : !cir.double
/// %2 = cir.fdiv %va, %vb : !cir.vector<4 x !cir.float>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fdiv {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub lhs: super::ValueId,
    /// floating point or vector of floating point type
    pub rhs: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.floor`
/// Computes the floating-point floor value
///
/// `cir.floor` computes the floor of a floating-point operand and returns
/// a result of the same type.
///
/// Floating-point exceptions are ignored, and it does not set `errno`.
///
/// Example:
///
/// ```
/// // $x : !cir.double
/// %y = cir.floor %x : !cir.double
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Floor {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub src: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.fma`
/// Fused Multiply-Add operation
///
/// Computes the fused multiply-add of three floating-point values or vector.
///
/// The inputs must be either:
///   • floating-point scalar types, or
///   • vectors whose element type is floating-point.
///
/// The result type must match the input type exactly.
///
/// Examples:
///   // scalar
///   %r = cir.fma %a, %b, %c : !cir.float
///
///   // vector
///   %v = cir.fma %a, %b, %c : !cir.vector<4 x !cir.float>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fma {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub a: super::ValueId,
    /// floating point or vector of floating point type
    pub b: super::ValueId,
    /// floating point or vector of floating point type
    pub c: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.fmaximum`
/// Returns the larger of two floating-point values (IEEE 754-2019)
///
/// `cir.fmaximum` returns the larger of its two operands according to
/// IEEE 754-2019 semantics. If either operand is NaN, NaN is returned.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fmaximum {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub lhs: super::ValueId,
    /// floating point or vector of floating point type
    pub rhs: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.fmaxnum`
/// Returns the larger of two floating-point values
///
/// `cir.fmaxnum` returns the larger of its two operands. If one operand is
/// NaN, the other operand is returned.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fmaxnum {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub lhs: super::ValueId,
    /// floating point or vector of floating point type
    pub rhs: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.fminimum`
/// Returns the smaller of two floating-point values (IEEE 754-2019)
///
/// `cir.fminimum` returns the smaller of its two operands according to
/// IEEE 754-2019 semantics. If either operand is NaN, NaN is returned.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fminimum {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub lhs: super::ValueId,
    /// floating point or vector of floating point type
    pub rhs: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.fminnum`
/// Returns the smaller of two floating-point values
///
/// `cir.fminnum` returns the smaller of its two operands. If one operand is
/// NaN, the other operand is returned.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fminnum {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub lhs: super::ValueId,
    /// floating point or vector of floating point type
    pub rhs: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.fmod`
/// Computes the floating-point remainder
///
/// `cir.fmod` computes the floating-point remainder of dividing the first
/// operand by the second operand.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fmod {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub lhs: super::ValueId,
    /// floating point or vector of floating point type
    pub rhs: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.fmul`
/// Floating-point multiplication
///
/// The `cir.fmul` operation performs floating-point multiplication on its
/// operands. Both operands and the result must have the same floating-point
/// scalar or vector-of-float type.
///
/// Example:
///
/// ```
/// %0 = cir.fmul %a, %b : !cir.float
/// %1 = cir.fmul %a, %b : !cir.double
/// %2 = cir.fmul %va, %vb : !cir.vector<4 x !cir.float>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fmul {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub lhs: super::ValueId,
    /// floating point or vector of floating point type
    pub rhs: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.fmuladd`
/// Contractable fused multiply-add operation
///
/// Computes `(a * b) + c`, allowing the multiply and add to be fused (or not)
/// into a single rounding step at the target's discretion. It lowers to the
/// `llvm.fmuladd` intrinsic (or its constrained variant when an `fenv`
/// attribute is present).
///
/// Unlike `cir.fma`, which maps to `llvm.fma` and guarantees a single
/// rounding, `cir.fmuladd` expresses the FP-contraction relaxation used for
/// `a * b + c` under `-ffp-contract=on` / `fast`, where the backend is free to
/// emit either a fused or an unfused sequence.
///
/// The inputs must be either:
///   • floating-point scalar types, or
///   • vectors whose element type is floating-point.
///
/// The result type must match the input type exactly.
///
/// Examples:
///   // scalar
///   %r = cir.fmuladd %a, %b, %c : !cir.float
///
///   // vector
///   %v = cir.fmuladd %a, %b, %c : !cir.vector<4 x !cir.float>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fmuladd {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub a: super::ValueId,
    /// floating point or vector of floating point type
    pub b: super::ValueId,
    /// floating point or vector of floating point type
    pub c: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.fneg`
/// Floating-point negation
///
/// The `cir.fneg` operation negates the operand. The operand and result must
/// have the same type.
///
/// Example:
///
/// ```
/// %1 = cir.fneg %0 : !cir.float
/// %3 = cir.fneg %2 : !cir.double
/// %5 = cir.fneg %4 : !cir.vector<4 x !cir.float>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fneg {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub input: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.freeze`
/// Stop propagation of undef and poison values
///
/// The `cir.freeze` operation takes a single operand and returns a value of the
/// same type. If the operand is a poison or undef value, `cir.freeze` returns
/// an arbitrary, but fixed, value of the operand type. Otherwise it returns
/// the operand unchanged.
///
/// Example:
///
/// ```mlir
/// %1 = cir.freeze %0 : !s32i
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Freeze {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// CIR void type or CIR bool type or CIR array type or CIR vector type or Integer type with arbitrary precision up to a fixed limit or single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type or CIR pointer type or CIR function type or CIR struct/class type or CIR union type or CIR complex type or CIR type that is used for the vptr member of C++ objects or CIR type that represents a pointer-to-data-member in C++ or CIR type that represents C++ pointer-to-member-function type or CIR exception handling token type or CIR cleanup token type or CIR catch token type
    pub input: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.frem`
/// Floating-point remainder
///
/// The `cir.frem` operation computes the floating-point remainder of its
/// operands. Both operands and the result must have the same floating-point
/// scalar or vector-of-float type.
///
/// Example:
///
/// ```
/// %0 = cir.frem %a, %b : !cir.float
/// %1 = cir.frem %a, %b : !cir.double
/// %2 = cir.frem %va, %vb : !cir.vector<4 x !cir.float>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Frem {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub lhs: super::ValueId,
    /// floating point or vector of floating point type
    pub rhs: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.frexp`
/// Decomposes a floating-point value into significand and exponent
///
/// `cir.frexp` splits a floating-point value into a normalized significand
/// (in the range [0.5, 1.0)) and an integral power-of-two exponent, such
/// that `src = significand * 2^exp`.  Returns both as separate results.
///
/// Lowers to `llvm.frexp`.
///
/// Example:
///   %sig, %exp = cir.frexp %x : !cir.float -> !cir.float, !s32i
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Frexp {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    pub exp: super::ValueId,
    pub exp_ty: crate::types::Type,
    /// single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type
    pub src: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.fsub`
/// Floating-point subtraction
///
/// The `cir.fsub` operation performs floating-point subtraction on its
/// operands. Both operands and the result must have the same floating-point
/// scalar or vector-of-float type.
///
/// Example:
///
/// ```
/// %0 = cir.fsub %a, %b : !cir.float
/// %1 = cir.fsub %a, %b : !cir.double
/// %2 = cir.fsub %va, %vb : !cir.vector<4 x !cir.float>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fsub {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub lhs: super::ValueId,
    /// floating point or vector of floating point type
    pub rhs: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.is_constant`
/// Test for manifest compile-time constant
///
/// Returns `true` if the argument is known to be a manifest compile-time
/// constant otherwise returns `false`. If the argument is a constant expression
/// which refers to a global (the address of which _is_ a constant, but not
/// manifest during the compile), then the intrinsic evaluates to `false`.
///
/// This is used to represent `__builtin_constant_p` in cases where the argument
/// isn't known to be constant during initial translation of the source code but
/// might be proven to be constant after later optimizations.
///
/// Example:
/// ```
/// %1 = cir.is_constant %2 : !s32i -> !cir.bool
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IsConstant {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// CIR void type or CIR bool type or CIR array type or CIR vector type or Integer type with arbitrary precision up to a fixed limit or single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type or CIR pointer type or CIR function type or CIR struct/class type or CIR union type or CIR complex type or CIR type that is used for the vptr member of C++ objects or CIR type that represents a pointer-to-data-member in C++ or CIR type that represents C++ pointer-to-member-function type or CIR exception handling token type or CIR cleanup token type or CIR catch token type
    pub val: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.is_fp_class`
/// Corresponding to the `__builtin_fpclassify` builtin function in clang
///
/// The `cir.is_fp_class` operation takes a floating-point value as its first
/// argument and a bitfield of flags as its second argument. The operation
/// returns a boolean value indicating whether the floating-point value
/// satisfies the given flags.
///
/// The flags must be a compile time constant and the values are:
///
/// | Bit # | floating-point class |
/// | ----- | -------------------- |
/// |  0    | Signaling NaN        |
/// |  1    | Quiet NaN            |
/// |  2    | Negative infinity    |
/// |  3    | Negative normal      |
/// |  4    | Negative subnormal   |
/// |  5    | Negative zero        |
/// |  6    | Positive zero        |
/// |  7    | Positive subnormal   |
/// |  8    | Positive normal      |
/// |  9    | Positive infinity    |
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IsFpClass {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type
    pub src: super::ValueId,
    /// floating-point class test flags
    pub flags: crate::enums::FpClassTest,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.llrint`
/// Rounds floating-point to long long integer using current rounding mode
///
/// `cir.llrint` rounds a floating-point value to the nearest integer value
/// using the current rounding mode and returns the result as a `long long`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Llrint {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type
    pub src: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.llround`
/// Rounds floating-point to long long integer
///
/// `cir.llround` rounds a floating-point value to the nearest integer value,
/// rounding halfway cases away from zero, and returns the result as a
/// `long long`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Llround {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type
    pub src: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.log`
/// Computes the floating-point natural logarithm
///
/// `cir.log` computes the natural logarithm of a floating-point operand and
/// returns a result of the same type.
///
/// Floating-point exceptions are ignored, and it does not set `errno`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Log {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub src: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.log10`
/// Computes the floating-point base-10 logarithm
///
/// `cir.log10` computes the base-10 logarithm of a floating-point operand and
/// returns a result of the same type.
///
/// Floating-point exceptions are ignored, and it does not set `errno`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Log10 {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub src: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.log2`
/// Computes the floating-point base-2 logarithm
///
/// `cir.log2` computes the base-2 logarithm of a floating-point operand and
/// returns a result of the same type.
///
/// Floating-point exceptions are ignored, and it does not set `errno`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Log2 {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub src: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.lrint`
/// Rounds floating-point to long integer using current rounding mode
///
/// `cir.lrint` rounds a floating-point value to the nearest integer value
/// using the current rounding mode and returns the result as a `long`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Lrint {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type
    pub src: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.lround`
/// Rounds floating-point to long integer
///
/// `cir.lround` rounds a floating-point value to the nearest integer value,
/// rounding halfway cases away from zero, and returns the result as a `long`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Lround {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type
    pub src: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.max`
/// Integer maximum
///
/// The `cir.max` operation computes the maximum of two integer operands.
/// Both operands and the result must have the same integer type or vector of
/// integer type.
///
/// Example:
///
/// ```
/// %0 = cir.max %a, %b : !s32i
/// %1 = cir.max %a, %b : !u32i
/// %2 = cir.max %a, %b : !cir.vector<4 x !s32i>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Max {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// integer or vector of integer type
    pub lhs: super::ValueId,
    /// integer or vector of integer type
    pub rhs: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.min`
/// Integer minimum
///
/// The `cir.min` operation computes the minimum of two integer operands.
/// Both operands and the result must have the same integer type or vector of
/// integer type.
///
/// Example:
///
/// ```
/// %0 = cir.min %a, %b : !s32i
/// %1 = cir.min %a, %b : !u32i
/// %2 = cir.min %a, %b : !cir.vector<4 x !s32i>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Min {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// integer or vector of integer type
    pub lhs: super::ValueId,
    /// integer or vector of integer type
    pub rhs: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.modf`
/// Decomposes a floating-point value into fractional and integral parts
///
/// `cir.modf` splits a floating-point value into its fractional and integral
/// parts.  Both parts have the same sign as the input.
///
/// Lowers to `llvm.modf`.
///
/// Example:
///   %frac, %intpart = cir.modf %x : !cir.float -> !cir.float, !cir.float
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Modf {
    pub fractional: super::ValueId,
    pub fractional_ty: crate::types::Type,
    pub integral: super::ValueId,
    pub integral_ty: crate::types::Type,
    /// single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type
    pub src: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.mul`
/// Integer multiplication
///
/// The `cir.mul` operation performs multiplication on integer operands. Both
/// operands and the result must have the same integer or vector-of-integer
/// type.
///
/// The optional `nsw` (no signed wrap) and `nuw` (no unsigned wrap) unit
/// attributes indicate that the result is poison if signed or unsigned
/// overflow occurs, respectively.
///
/// Example:
///
/// ```
/// %0 = cir.mul %a, %b : !s32i
/// %1 = cir.mul nsw %a, %b : !s32i
/// %2 = cir.mul nuw %a, %b : !u32i
/// %3 = cir.mul %va, %vb : !cir.vector<4 x !s32i>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mul {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// integer or vector of integer type
    pub lhs: super::ValueId,
    /// integer or vector of integer type
    pub rhs: super::ValueId,
    /// unit property
    pub no_signed_wrap: bool,
    /// unit property
    pub no_unsigned_wrap: bool,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.nearbyint`
/// Rounds floating-point value to nearest integer
///
/// `cir.nearbyint` rounds a floating-point operand to the nearest integer value
/// and returns a result of the same type.
///
/// Floating-point exceptions are ignored, and it does not set `errno`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Nearbyint {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub src: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.objsize`
/// Implements the llvm.objsize builtin
///
/// The `cir.objsize` operation is designed to provide information to the
/// optimizer to determine whether a) an operation (like memcpy) will
/// overflow a buffer that corresponds to an object, or b) that a runtime
/// check for overflow isn’t necessary. An object in this context means an
/// allocation of a specific class, structure, array, or other object.
///
/// When the `min` attribute is present, the operation returns the minimum
/// guaranteed accessible size. When absent (max mode), it returns the maximum
/// possible object size. Corresponds to `llvm.objectsize`'s `min` argument.
///
/// The `dynamic` attribute determines if the value should be evaluated at
/// runtime. Corresponds to `llvm.objectsize`'s `dynamic` argument.
///
/// The `nullunknown` attribute controls how null pointers are handled. When
/// present, null pointers are treated as having unknown size. When absent,
/// null pointers are treated as having 0 size (in min mode) or -1 size
/// (in max mode). Corresponds to `llvm.objectsize`'s `nullunknown` argument.
///
/// Example:
///
/// ```
/// %size = cir.objsize min %ptr : !cir.ptr<i32> -> i64
/// %dsize = cir.objsize max dynamic %ptr : !cir.ptr<i32> -> i64
/// %nsize = cir.objsize min nullunknown %ptr : !cir.ptr<i32> -> i64
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Objsize {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// CIR pointer type
    pub ptr: super::ValueId,
    /// unit attribute
    pub min: bool,
    /// unit attribute
    pub nullunknown: bool,
    /// unit attribute
    pub dynamic: bool,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.or`
/// Bitwise OR
///
/// The `cir.or` operation performs a bitwise OR on integer operands.
/// Both operands and the result must have the same integer type.
///
/// Example:
///
/// ```
/// %0 = cir.or %a, %b : !s32i
/// %1 = cir.or %a, %b : !cir.bool
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Or {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// integer, boolean, or vector of bool or integer
    pub lhs: super::ValueId,
    /// integer, boolean, or vector of bool or integer
    pub rhs: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.pow`
/// Computes the power of a floating-point value
///
/// `cir.pow` computes the first operand raised to the power of the second
/// operand.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pow {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub lhs: super::ValueId,
    /// floating point or vector of floating point type
    pub rhs: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.rem`
/// Integer remainder
///
/// The `cir.rem` operation computes the remainder of division on integer
/// operands. Both operands and the result must have the same integer or
/// vector-of-integer type.
///
/// Example:
///
/// ```
/// %0 = cir.rem %a, %b : !s32i
/// %1 = cir.rem %a, %b : !u32i
/// %2 = cir.rem %va, %vb : !cir.vector<4 x !s32i>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rem {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// integer or vector of integer type
    pub lhs: super::ValueId,
    /// integer or vector of integer type
    pub rhs: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.rint`
/// Rounds floating-point value to nearest integer
///
/// `cir.rint` rounds a floating-point operand to the nearest integer value
/// and returns a result of the same type.
///
/// This operation does not set `errno`. Unlike `cir.nearbyint`, this operation
/// may raise the `FE_INEXACT` exception if the input value is not an exact
/// integer, but this is not guaranteed to happen.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rint {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub src: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.rotate`
/// Rotate the bits in the operand integer
///
/// The `cir.rotate` rotates the bits in `input` by the given amount `amount`.
/// The rotate direction is specified by the `left` and `right` keyword.
///
/// `input` must be an unsigned integer and its width must be either 8, 16, 32,
/// or 64. The types of `input`, `amount`, and the result must all match.
///
/// Example:
///
/// ```
/// %r = cir.rotate left %0, %1 : !u32i
/// %r = cir.rotate right %0, %1 : !u32i
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rotate {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// unsigned integer type of widths 8/16/32/64
    pub input: super::ValueId,
    /// Integer type with arbitrary precision up to a fixed limit
    pub amount: super::ValueId,
    /// unit attribute
    pub rotate_left: bool,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.round`
/// Rounds floating-point value to nearest integer
///
/// `cir.round` rounds a floating-point operand to the nearest integer value
/// and returns a result of the same type.
///
/// Floating-point exceptions are ignored, and it does not set `errno`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Round {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub src: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.roundeven`
/// Rounds floating-point value to nearest integer, ties to even
///
/// `cir.roundeven` rounds a floating-point operand to the nearest integer
/// value, with ties rounding to even (banker's rounding).
///
/// Floating-point exceptions are ignored, and it does not set `errno`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Roundeven {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub src: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.select`
/// Yield one of two values based on a boolean value
///
/// The `cir.select` operation takes three operands. The first operand
/// `condition` is either a boolean value of type `!cir.bool` or a boolean
/// vector of type `!cir.bool`.  The second and the third operand can be of
/// any CIR types, but their types must be the same. If the first operand
/// is `true`, the operation yields its second operand. Otherwise, the
/// operation yields its third operand.
///
/// In the case where the first operand is a boolean vector, then the second
/// and third operand needs to also be of some vectors of the same type to
/// each other and that the number of elements of all three operands needs to
/// be the same as well.
///
/// Example:
///
/// ```
/// %0 = cir.const #cir.bool<true> : !cir.bool
/// %1 = cir.const #cir.int<42> : !s32i
/// %2 = cir.const #cir.int<72> : !s32i
/// %3 = cir.select if %0 then %1 else %2 : (!cir.bool, !s32i, !s32i) -> !s32i
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Select {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// CIR bool type or vector of CIR bool type
    pub condition: super::ValueId,
    /// CIR void type or CIR bool type or CIR array type or CIR vector type or Integer type with arbitrary precision up to a fixed limit or single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type or CIR pointer type or CIR function type or CIR struct/class type or CIR union type or CIR complex type or CIR type that is used for the vptr member of C++ objects or CIR type that represents a pointer-to-data-member in C++ or CIR type that represents C++ pointer-to-member-function type or CIR exception handling token type or CIR cleanup token type or CIR catch token type
    pub true_value: super::ValueId,
    /// CIR void type or CIR bool type or CIR array type or CIR vector type or Integer type with arbitrary precision up to a fixed limit or single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type or CIR pointer type or CIR function type or CIR struct/class type or CIR union type or CIR complex type or CIR type that is used for the vptr member of C++ objects or CIR type that represents a pointer-to-data-member in C++ or CIR type that represents C++ pointer-to-member-function type or CIR exception handling token type or CIR cleanup token type or CIR catch token type
    pub false_value: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.shift`
/// Shift
///
/// The `cir.shift` operation performs a bitwise shift, either to the left or to
/// the right, based on the first operand. The second operand specifies the
/// value to be shifted, and the third operand determines the number of
/// positions by which the shift is applied, They must be either all vector of
/// integer type, or all integer type. If they are vectors, each vector element of
/// the shift target is shifted by the corresponding shift amount in
/// the shift amount vector.
///
/// ```
/// %res = cir.shift(left, %lhs : !u64i, %amount : !s32i) -> !u64i
/// %new_vec = cir.shift(left, %lhs : !cir.vector<2 x !s32i>, %rhs :
///     !cir.vector<2 x !s32i>) -> !cir.vector<2 x !s32i>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Shift {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// integer or vector of integer type
    pub value: super::ValueId,
    /// integer or vector of integer type
    pub amount: super::ValueId,
    /// unit attribute
    pub is_shiftleft: bool,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.sin`
/// Computes the floating-point sine
///
/// `cir.sin` computes the sine of a floating-point operand and returns
/// a result of the same type.
///
/// Floating-point exceptions are ignored, and it does not set `errno`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Sin {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub src: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.sinh`
/// Computes the floating-point hyperbolic sine
///
/// `cir.sinh` computes the hyperbolic sine of a floating-point operand and
/// returns a result of the same type.
///
/// Floating-point exceptions are ignored, and it does not set `errno`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Sinh {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub src: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.sqrt`
/// Floating-point square root operation
///
/// Computes the square root of a floating-point value or vector.
///
/// The input must be either:
///   • a floating-point scalar type, or
///   • a vector whose element type is floating-point.
///
/// The result type must match the input type exactly.
///
/// Examples:
///   // scalar
///   %r = cir.sqrt %x : !cir.fp64
///
///   // vector
///   %v = cir.sqrt %vec : !cir.vector<!cir.fp32 x 4>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Sqrt {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub src: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.sub`
/// Integer subtraction
///
/// The `cir.sub` operation performs subtraction on integer operands. Both
/// operands and the result must have the same integer or vector-of-integer
/// type.
///
/// The optional `nsw` (no signed wrap) and `nuw` (no unsigned wrap) unit
/// attributes indicate that the result is poison if signed or unsigned
/// overflow occurs, respectively. The optional `sat` (saturated) attribute
/// clamps the result to the type's representable range. The `nsw`/`nuw`
/// flags and `sat` are mutually exclusive.
///
/// Example:
///
/// ```
/// %0 = cir.sub %a, %b : !s32i
/// %1 = cir.sub nsw %a, %b : !s32i
/// %2 = cir.sub sat %a, %b : !s32i
/// %3 = cir.sub %va, %vb : !cir.vector<4 x !s32i>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Sub {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// integer or vector of integer type
    pub lhs: super::ValueId,
    /// integer or vector of integer type
    pub rhs: super::ValueId,
    /// unit property
    pub no_signed_wrap: bool,
    /// unit property
    pub no_unsigned_wrap: bool,
    /// unit property
    pub saturated: bool,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.tan`
/// Computes the floating-point tangent
///
/// `cir.tan` computes the tangent of a floating-point operand and returns
/// a result of the same type.
///
/// Floating-point exceptions are ignored, and it does not set `errno`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tan {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub src: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.tanh`
/// Computes the floating-point hyperbolic tangent
///
/// `cir.tanh` computes the hyperbolic tangent of a floating-point operand and
/// returns a result of the same type.
///
/// Floating-point exceptions are ignored, and it does not set `errno`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tanh {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub src: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.trunc`
/// Truncates floating-point value to integer
///
/// `cir.trunc` truncates a floating-point operand to an integer value
/// and returns a result of the same type.
///
/// Floating-point exceptions are ignored, and it does not set `errno`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Trunc {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// floating point or vector of floating point type
    pub src: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.xor`
/// Bitwise XOR
///
/// The `cir.xor` operation performs a bitwise XOR on integer operands.
/// Both operands and the result must have the same integer type.
///
/// Example:
///
/// ```
/// %0 = cir.xor %a, %b : !s32i
/// %1 = cir.xor %a, %b : !cir.bool
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Xor {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// integer, boolean, or vector of bool or integer
    pub lhs: super::ValueId,
    /// integer, boolean, or vector of bool or integer
    pub rhs: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}