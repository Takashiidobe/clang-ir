//! Complex-number operations.

/// `cir.complex.add`
/// Complex addition
///
/// The `cir.complex.add` operation takes two complex numbers and returns
/// their sum.
///
/// Example:
///
/// ```
/// %2 = cir.complex.add %0, %1 : !cir.complex<!cir.float>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComplexAdd {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// CIR complex type
    pub lhs: super::ValueId,
    /// CIR complex type
    pub rhs: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.complex.conj`
/// Complex conjugate
///
/// The `cir.complex.conj` operation takes a complex number and returns its
/// complex conjugate, which is formed by negating the imaginary part.
///
/// Example:
///
/// ```
/// %1 = cir.complex.conj %0 : !cir.complex<!cir.float>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComplexConj {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// CIR complex type
    pub operand: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.complex.create`
/// Create a complex value from its real and imaginary parts
///
/// The `cir.complex.create` operation takes two operands that represent the
/// real and imaginary part of a complex number, and yields the complex number.
///
/// ```
/// %0 = cir.const #cir.fp<1.000000e+00> : !cir.double
/// %1 = cir.const #cir.fp<2.000000e+00> : !cir.double
/// %2 = cir.complex.create %0, %1 : !cir.double -> !cir.complex<!cir.double>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComplexCreate {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// integer or floating point type
    pub real: super::ValueId,
    /// integer or floating point type
    pub imag: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.complex.div`
/// Complex division
///
/// The `cir.complex.div` operation takes two complex numbers and returns
/// their quotient.
///
/// For complex types with floating-point components, the `range` attribute
/// specifies the algorithm to be used when the operation is lowered to
/// the LLVM dialect. For division, 'improved' produces Smith's algorithms for
/// Complex division with no additional handling for NaN values. If 'promoted'
/// is used, the values are promoted to a higher precision type, if possible,
/// and the calculation is performed using the algebraic formula, with
/// no additional handling for NaN values. We fall back on Smith's algorithm
/// when the target doesn't support a higher precision type. If 'full' is used,
/// a runtime-library function is called if one of the intermediate
/// calculations produced a NaN value. and for 'basic' algebraic formula with
/// no additional handling for the NaN value will be used. For integers types
/// `range` attribute will be ignored.
///
/// Example:
///
/// ```
/// %2 = cir.complex.div %0, %1 range(basic) : !cir.complex<!cir.float>
/// %2 = cir.complex.div %0, %1 range(full) : !cir.complex<!cir.float>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComplexDiv {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// CIR complex type
    pub lhs: super::ValueId,
    /// CIR complex type
    pub rhs: super::ValueId,
    /// complex multiplication and division implementation
    pub range: crate::enums::ComplexRangeKind,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.complex.imag`
/// Extract the imaginary part of a complex value
///
/// `cir.complex.imag` operation takes an operand of `!cir.complex`, `!cir.int`
/// `!cir.bool` or `!cir.float`. If the operand is `!cir.complex`, the imag
/// part of it will be returned, otherwise a zero value will be returned.
///
/// Example:
///
/// ```
/// %imag = cir.complex.imag %complex : !cir.complex<!cir.float> -> !cir.float
/// %imag = cir.complex.imag %scalar : !cir.float -> !cir.float
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComplexImag {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// complex, integer, boolean or floating point type
    pub operand: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.complex.imag_ptr`
/// Derive a pointer to the imaginary part of a complex value
///
/// `cir.complex.imag_ptr` operation takes a pointer operand that points to a
/// complex value of type `!cir.complex` and yields a pointer to the imaginary
/// part of the operand.
///
/// Example:
///
/// ```
/// %1 = cir.complex.imag_ptr %0 : !cir.ptr<!cir.complex<!cir.double>>
///   -> !cir.ptr<!cir.double>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComplexImagPtr {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// pointer to complex type
    pub operand: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.complex.mul`
/// Complex multiplication
///
/// The `cir.complex.mul` operation takes two complex numbers and returns
/// their product.
///
/// For complex types with floating-point components, the `range` attribute
/// specifies the algorithm to be used when the operation is lowered to
/// the LLVM dialect. For multiplication, 'improved', 'promoted', and 'basic'
/// are all handled equivalently, producing the algebraic formula with no
/// special handling for NaN value. If 'full' is used, a runtime-library
/// function is called if one of the intermediate calculations produced
/// a NaN value.
///
/// Example:
///
/// ```
/// %2 = cir.complex.mul %0, %1 range(basic) : !cir.complex<!cir.float>
/// %2 = cir.complex.mul %0, %1 range(full) : !cir.complex<!cir.float>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComplexMul {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// CIR complex type
    pub lhs: super::ValueId,
    /// CIR complex type
    pub rhs: super::ValueId,
    /// complex multiplication and division implementation
    pub range: crate::enums::ComplexRangeKind,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.complex.real`
/// Extract the real part of a complex value
///
/// `cir.complex.real` operation takes an operand of `!cir.complex`, `cir.int`,
/// `!cir.bool` or `!cir.float`. If the operand is `!cir.complex`, the real
/// part of it will be returned, otherwise the value returned unmodified.
///
/// Example:
///
/// ```
/// %real = cir.complex.real %complex : !cir.complex<!cir.float> -> !cir.float
/// %real = cir.complex.real %scalar : !cir.float -> !cir.float
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComplexReal {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// complex, integer, boolean or floating point type
    pub operand: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.complex.real_ptr`
/// Derive a pointer to the real part of a complex value
///
/// `cir.complex.real_ptr` operation takes a pointer operand that points to a
/// complex value of type `!cir.complex` and yields a pointer to the real part
/// of the operand.
///
/// Example:
///
/// ```
/// %1 = cir.complex.real_ptr %0 : !cir.ptr<!cir.complex<!cir.double>>
///   -> !cir.ptr<!cir.double>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComplexRealPtr {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// pointer to complex type
    pub operand: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.complex.sub`
/// Complex subtraction
///
/// The `cir.complex.sub` operation takes two complex numbers and returns
/// their difference.
///
/// Example:
///
/// ```
/// %2 = cir.complex.sub %0, %1 : !cir.complex<!cir.float>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComplexSub {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// CIR complex type
    pub lhs: super::ValueId,
    /// CIR complex type
    pub rhs: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}