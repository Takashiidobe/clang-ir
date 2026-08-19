//! Standard-library operations.

/// `cir.libc.memchr`
/// libc's `memchr`
///
/// Search for `pattern` in data range from `src` to `src` + `len`.
/// `len` provides a bound to the search in `src`. `result` is a pointer to
/// found `pattern` or a null pointer.
///
/// Examples:
///
/// ```
/// %p = cir.libc.memchr(%src, %pattern, %len)
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LibcMemchr {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    pub src: super::ValueId,
    /// 32-bit signed integer
    pub pattern: super::ValueId,
    /// 64-bit unsigned integer
    pub len: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.libc.memcpy`
/// Equivalent to libc's `memcpy`
///
/// Given two CIR pointers, `src` and `dst`, `cir.libc.memcpy` will copy `len`
/// bytes from the memory pointed by `src` to the memory pointed by `dst`.
///
/// While `cir.copy` is meant to be used for implicit copies in the code where
/// the length of the copy is known, `cir.memcpy` copies only from and to void
/// pointers, requiring the copy length to be passed as an argument.
///
/// As is the case for memcpy in the C standard library, this operation
/// exhibits undefined behavior (UB) if any of the following conditions hold:
///   * `src` and/or `dst` are null pointers; or
///   * the memory regions referenced by `src` and `dst` overlap.
///
/// Examples:
///
/// ```
///   // Copying 2 bytes from one array to a record:
///   %2 = cir.const #cir.int<2> : !u32i
///   cir.libc.memcpy %2 bytes from %arr to %record : !cir.ptr<!arr> -> !cir.ptr<!record>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LibcMemcpy {
    pub dst: super::ValueId,
    pub src: super::ValueId,
    /// fundamental unsigned integer type
    pub len: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.libc.memmove`
/// Equivalent to libc's `memmove`
///
/// Given two CIR pointers, `src` and `dst`, `cir.libc.memmove` will copy `len`
/// bytes from the memory pointed by `src` to the memory pointed by `dst`.
///
/// similiar to `cir.libc.memcpy` but accounts for overlapping memory.
///
/// Examples:
///
/// ```
///   // Copying 2 bytes from one array to a record:
///   %2 = cir.const #cir.int<2> : !u32i
///   cir.libc.memmove %2 bytes from %arr to %record : !cir.ptr<!void>, !u64i
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LibcMemmove {
    pub dst: super::ValueId,
    pub src: super::ValueId,
    /// fundamental unsigned integer type
    pub len: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.libc.memset`
/// Equivalent to libc's `memset`
///
/// Given the CIR pointer, `dst`, `cir.libc.memset` will set the first `len`
/// bytes of the memory pointed by `dst` to the specified `val`.
///
/// Examples:
///
/// ```
///   // Set 2 bytes in a record to 0:
///   %len = cir.const #cir.int<2> : !u32i
///   %zero = cir.const #cir.int<0> : !u8i
///   cir.libc.memset %len bytes at %record to %zero : !cir.ptr<!void>,
///                                                    !s32i, !u64i
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LibcMemset {
    pub dst: super::ValueId,
    /// 64-bit signless integer attribute
    pub alignment: Option<crate::attrs::Attribute>,
    /// 8-bit unsigned integer
    pub val: super::ValueId,
    /// fundamental unsigned integer type
    pub len: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.std.find`
/// std::find()
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StdFind {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// CIR void type or CIR bool type or CIR array type or CIR vector type or Integer type with arbitrary precision up to a fixed limit or single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type or CIR pointer type or CIR function type or CIR struct/class type or CIR union type or CIR complex type or CIR type that is used for the vptr member of C++ objects or CIR type that represents a pointer-to-data-member in C++ or CIR type that represents C++ pointer-to-member-function type or CIR exception handling token type or CIR cleanup token type or CIR catch token type
    pub first: super::ValueId,
    /// CIR void type or CIR bool type or CIR array type or CIR vector type or Integer type with arbitrary precision up to a fixed limit or single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type or CIR pointer type or CIR function type or CIR struct/class type or CIR union type or CIR complex type or CIR type that is used for the vptr member of C++ objects or CIR type that represents a pointer-to-data-member in C++ or CIR type that represents C++ pointer-to-member-function type or CIR exception handling token type or CIR cleanup token type or CIR catch token type
    pub last: super::ValueId,
    /// CIR void type or CIR bool type or CIR array type or CIR vector type or Integer type with arbitrary precision up to a fixed limit or single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type or CIR pointer type or CIR function type or CIR struct/class type or CIR union type or CIR complex type or CIR type that is used for the vptr member of C++ objects or CIR type that represents a pointer-to-data-member in C++ or CIR type that represents C++ pointer-to-member-function type or CIR exception handling token type or CIR cleanup token type or CIR catch token type
    pub pattern: super::ValueId,
    /// flat symbol reference attribute
    pub original_fn: crate::attrs::Attribute,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.std.strlen`
/// C standard library strlen()
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StdStrlen {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// pointer to 8-bit character type
    pub string: super::ValueId,
    /// flat symbol reference attribute
    pub original_fn: crate::attrs::Attribute,
    pub loc: Option<crate::ast::SourceLocation>,
}