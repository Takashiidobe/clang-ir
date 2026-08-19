//! CIR types generated from CIRTypes.td.

#![allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// `!cir.array`
    /// CIR array type
    ///
    /// `!cir.array` represents C/C++ constant arrays.
    Array { element_type: Box<Type>, size: u64 },
    /// `!cir.bf16`
    /// CIR bfloat16 16-bit float type
    ///
    /// A 16-bit floating-point type in the bfloat16 format, which is the same as
    /// IEEE `binary32` except that the lower 16 bits of the mantissa are missing.
    /// It represents the type `std::bfloat16_t` in C++, also spelled `__bf16` in
    /// some implementations.
    Bf16,
    /// `!cir.bool`
    /// CIR bool type
    ///
    /// `!cir.bool` represents C++ bool type.
    Bool,
    /// `!cir.catch_token`
    /// CIR catch token type
    ///
    /// `!cir.catch_token` is an opaque type used to track catch handling state
    /// in flattened CIR. It is returned by `cir.begin_catch` and consumed by
    /// `cir.end_catch`.
    ///
    /// This token ensures that catch handlers are properly paired and allows
    /// the ABI lowering pass to generate appropriate exception catching code.
    CatchToken,
    /// `!cir.cleanup_token`
    /// CIR cleanup token type
    ///
    /// `!cir.cleanup_token` is an opaque type used to track cleanup handling state
    /// in flattened CIR. It is returned by `cir.begin_cleanup` and consumed by
    /// `cir.end_cleanup`.
    ///
    /// This token ensures that cleanup regions are properly paired and allows
    /// the ABI lowering pass to generate appropriate cleanup handling code.
    CleanupToken,
    /// `!cir.complex`
    /// CIR complex type
    ///
    /// CIR type that represents a C complex number. `cir.complex` models the C type
    /// `T _Complex`.
    ///
    /// `cir.complex` type is not directly mapped to `std::complex`.
    ///
    /// The type models complex values, per C99 6.2.5p11. It supports the C99
    /// complex float types as well as the GCC integer complex extensions.
    ///
    /// The parameter `elementType` gives the type of the real and imaginary part of
    /// the complex number. `elementType` must be either a CIR integer type or a CIR
    /// floating-point type.
    ///
    /// ```
    /// !cir.complex<!s32i>
    /// !cir.complex<!cir.float>
    /// ```
    Complex {
        /// integer or floating point type
        element_type: Box<Type>,
    },
    /// `!cir.data_member`
    /// CIR type that represents a pointer-to-data-member in C++
    ///
    /// `cir.data_member` models a pointer-to-data-member in C++. Values of this
    /// type are essentially offsets of the pointed-to member within one of its
    /// containing record.
    DataMember { member_ty: Box<Type>, class_ty: Box<Type> },
    /// `!cir.double`
    /// CIR double-precision 64-bit float type
    ///
    /// A 64-bit floating-point type whose format is IEEE-754 `binary64`. It
    /// represents the types `double', '_Float64`, `std::float64_t`, and `_Float32x`
    /// in C and C++.  This is the underlying type for `long double` on some
    /// platforms, including Windows.
    Double,
    /// `!cir.eh_token`
    /// CIR exception handling token type
    ///
    /// `!cir.eh_token` is an opaque type used to track exception handling state
    /// in flattened CIR. It is returned by `cir.eh.initiate` and passed to
    /// `cir.eh.dispatch`, `cir.begin_cleanup`, and `cir.begin_catch` operations.
    ///
    /// This token represents an in-flight exception and is used during ABI-lowering
    /// to generate the appropriate exception handling code.
    EhToken,
    /// `!cir.f16`
    /// CIR half-precision 16-bit float type
    ///
    /// A 16-bit floating-point type whose format is IEEE-754 `binary16`. It
    /// represents the types '_Float16` and `std::float16_t` in C and C++.
    Fp16,
    /// `!cir.f80`
    /// CIR x87 80-bit float type
    ///
    /// An 80-bit floating-point type in the x87 extended precision format.  The
    /// size and alignment of the type are both 128 bits, even though only 80 of
    /// those bits are used.  This is the underlying type for `long double` on Linux
    /// x86 platforms, and it is available as an extension in some implementations.
    Fp80,
    /// `!cir.f128`
    /// CIR quad-precision 128-bit float type
    ///
    /// A 128-bit floating-point type whose format is IEEE-754 `binary128`. It
    /// represents the types `_Float128` and `std::float128_t` in C and C++, and the
    /// extension `__float128` in some implementations.  This is the underlying type
    /// for `long double` on some platforms including Linux Arm.
    Fp128,
    /// `!cir.func`
    /// CIR function type
    ///
    /// The `!cir.func` is a function type. It consists of an optional return type,
    /// a list of parameter types and can optionally be variadic.
    ///
    /// Example:
    ///
    /// ```
    /// !cir.func<()>
    /// !cir.func<() -> bool>
    /// !cir.func<(!s8i, !s8i)>
    /// !cir.func<(!s8i, !s8i) -> !s32i>
    /// !cir.func<(!s32i, ...) -> !s32i>
    /// ```
    Func { inputs: Vec<Type>, optional_return_type: Option<Box<Type>>, var_arg: bool },
    /// `!cir.int`
    /// Integer type with arbitrary precision up to a fixed limit
    ///
    /// CIR type that represents integer types with arbitrary precision, including
    /// standard integral types such as `int` and `long`, extended integral types
    /// such as `__int128`, and arbitrary width types such as `_BitInt(n)`.
    ///
    /// Those integer types that are directly available in C/C++ standard are called
    /// fundamental integer types. Said types are: `signed char`, `short`, `int`,
    /// `long`, `long long`, and their unsigned variations.
    ///
    /// Examples: `!cir.int<s, 32>`, `!cir.int<u, 64>`, `!cir.int<s, 128, bitint>`
    Int { width: u32, is_signed: bool, is_bit_int: bool },
    /// `!cir.long_double`
    /// CIR float type for `long double`
    ///
    /// A floating-point type that represents the `long double` type in C and C++.
    ///
    /// The underlying floating-point format of a `long double` value depends on the
    /// target platform and the implementation. The `underlying` parameter specifies
    /// the CIR floating-point type that corresponds to this format. Underlying
    /// types of IEEE 64-bit, IEEE 128-bit, x87 80-bit, and IBM's double-double
    /// format are all in use.
    LongDouble {
        /// expects !cir.double, !cir.fp80 or !cir.fp128
        underlying: Box<Type>,
    },
    /// `!cir.method`
    /// CIR type that represents C++ pointer-to-member-function type
    ///
    /// `cir.method` models the pointer-to-member-function type in C++. The layout
    /// of this type is ABI-dependent.
    Method { member_func_ty: Box<Type>, class_ty: Box<Type> },
    /// `!cir.ptr`
    /// CIR pointer type
    ///
    /// The `!cir.ptr` type is a typed pointer type. It is used to represent
    /// pointers to objects in C/C++. The type of the pointed-to object is given by
    /// the `pointee` parameter. The `addrSpace` parameter is an optional address
    /// space attribute that specifies the address space of the pointer. If not
    /// specified, the pointer is assumed to be in the default address space.
    ///
    /// The `!cir.ptr` type can point to any type, including fundamental types,
    /// records, arrays, vectors, functions, and other pointers. It can also point
    /// to incomplete types, such as incomplete records.
    ///
    /// Examples:
    ///
    /// ```
    /// !cir.ptr<!cir.int<u, 8>>
    /// !cir.ptr<!cir.float>
    /// !cir.ptr<!cir.record<struct "MyStruct">>
    /// !cir.ptr<!cir.int<u, 8>, target_address_space(1)>
    /// !cir.ptr<!cir.record<struct "MyStruct">, target_address_space(5)>
    /// ```
    Pointer { pointee: Box<Type>, addr_space: Option<String> },
    /// `!cir.float`
    /// CIR single-precision 32-bit float type
    ///
    /// A 32-bit floating-point type whose format is IEEE-754 `binary32`.  It
    /// represents the types `float`, `_Float32`, and `std::float32_t` in C and C++.
    Single,
    /// `!cir.struct`
    /// CIR struct/class type
    ///
    /// Each unique clang::RecordDecl with struct or class kind is mapped to a
    /// `cir.struct` type.  Any object in C/C++ that has a struct or class type
    /// will have a `!cir.struct` in CIR.
    ///
    /// There are three possible formats:
    ///
    ///  - Identified and complete: unique name and a known body.
    ///  - Identified and incomplete: unique name and unknown body.
    ///  - Anonymous: no name and a known body.
    ///
    /// Typically, the full data size of a struct object is that which is calculated
    /// by adding the aligned sizes of all the members. However, note that there is
    /// one exception to this Flexible Array Members. When the last element is an
    /// array type of size zero(in CIR), this represents the C/C++ concept of
    /// 'flexible array member', where as long as the type is allocated properly (or
    /// a global constant), an access through the array member may access outside of
    /// its bounds. This is necessary to represent C/C++ semantics.
    ///
    /// The optional `class` keyword distinguishes C++ class declarations from
    /// plain struct declarations.  Both are semantically identical; the keyword
    /// preserves the original source spelling.
    ///
    /// Every member has a kind, described by `CIR_RecordMemberKind`, saying what
    /// it holds, and every member is spelled with its mark.
    ///
    /// Examples:
    ///
    /// ```
    ///     !rec_complete = !cir.struct<"complete" {data !u8i}>
    ///     !rec_class    = !cir.struct<class "MyClass" {data !s32i}>
    ///     !rec_incomplete = !cir.struct<"incomplete" incomplete>
    ///     !anonymous    = !cir.struct<{data !u8i}>
    ///     !rec_packed   = !cir.struct<"p1" packed {data !u8i, data !u8i}>
    ///     !rec_pad      = !cir.struct<"p3" {data !u8i, pad !cir.array<!u8i x 3>}>
    ///     !rec_empty    = !cir.struct<"e" {empty !u8i}>
    ///     !recursive    = !cir.struct<"Node" {data !cir.ptr<!cir.struct<"Node">>}>
    /// ```
    Struct {
        members: Option<Vec<Type>>,
        name: Option<String>,
        incomplete: bool,
        packed: bool,
        member_kinds: Vec<crate::enums::RecordMemberKind>,
        is_class: bool,
    },
    /// `!cir.union`
    /// CIR union type
    ///
    /// Each unique clang::RecordDecl with union kind is mapped to a `cir.union`
    /// type.  Any object in C/C++ that has a union type will have a `!cir.union`
    /// in CIR.
    ///
    /// There are three possible formats:
    ///
    ///  - Identified and complete: unique name and a known body.
    ///  - Identified and incomplete: unique name and unknown body.
    ///  - Anonymous: no name and a known body.
    ///
    /// Padded unions carry an explicit tail-padding type to ensure the LLVM struct
    /// that models the union has the correct byte size.  That slot is separate
    /// from the per-member kinds described by `CIR_RecordMemberKind`, which say
    /// what each variant holds.  The parser rejects a mark on that slot.
    ///
    /// Examples:
    ///
    /// ```
    ///     !u_complete   = !cir.union<"U" {data !s32i, data !u8i}>
    ///     !u_incomplete = !cir.union<"U" incomplete>
    ///     !u_anonymous  = !cir.union<{data !s32i, data !u8i}>
    ///     !u_padded     = !cir.union<"U" {data !s32i, data !u8i}, padding = {!u8i}>
    ///     !u_empty      = !cir.union<"U" {empty !u8i}>
    /// ```
    Union {
        members: Option<Vec<Type>>,
        name: Option<String>,
        incomplete: bool,
        packed: bool,
        padding: Option<Box<Type>>,
        member_kinds: Vec<crate::enums::RecordMemberKind>,
    },
    /// `!cir.vptr`
    /// CIR type that is used for the vptr member of C++ objects
    ///
    /// `cir.vptr` is a special type used as the type for the vptr member of a C++
    /// object. This avoids using arbitrary pointer types to declare vptr values
    /// and allows stronger type-based checking for operations that use or provide
    /// access to the vptr.
    ///
    /// This type will be the element type of the 'vptr' member of structures that
    /// require a vtable pointer. The `cir.vtable.address_point` operation returns
    /// this type. The `cir.vtable.get_vptr` operations returns a pointer to this
    /// type. This pointer may be passed to the `cir.vtable.get_virtual_fn_addr`
    /// operation to get the address of a virtual function pointer.
    ///
    /// The pointer may also be cast to other pointer types in order to perform
    /// pointer arithmetic based on information encoded in the AST layout to get
    /// the offset from a pointer to a dynamic object to the base object pointer,
    /// the base object offset value from the vtable, or the type information
    /// entry for an object.
    /// TODO: We should have special operations to do that too.
    VPtr,
    /// `!cir.vector`
    /// CIR vector type
    ///
    /// The `!cir.vector` type represents a one-dimensional vector.
    /// It takes three parameters: the element type, the number of elements and the
    /// scalability flag (optional, defaults to `false`).
    ///
    /// Syntax:
    ///
    /// ```
    /// vector-type ::= !cir.vector<size x element-type>
    /// size ::= (decimal-literal | `[` decimal-literal `]`)
    /// element-type ::= float-type | integer-type | pointer-type
    /// ```
    ///
    /// The `element-type` must be a scalar CIR type. Zero-sized vectors are not
    /// allowed. The `size` must be a positive integer.
    ///
    /// Examples:
    ///
    /// ```
    /// !cir.vector<4 x !cir.int<u, 8>>
    /// !cir.vector<2 x !cir.float>
    /// ```
    ///
    /// Scalable vectors are indicated by enclosing size in square brackets.
    ///
    /// Example:
    /// ```
    /// !cir.vector<[4] x !cir.int<u, 8>>
    /// ```
    Vector {
        /// any cir boolean, integer, floating point or pointer type
        element_type: Box<Type>,
        size: u64,
        is_scalable: Option<bool>,
    },
    /// `!cir.void`
    /// CIR void type
    ///
    /// The `!cir.void` type represents the C and C++ `void` type.
    Void,
    /// A named type alias.
    Named(String),
    /// A builtin signless integer type.
    Integer(u32),
    /// The builtin `index` type.
    Index,
    /// A builtin function type.
    FunctionType { inputs: Vec<Type>, results: Vec<Type> },
    /// A type outside the CIR-specific variants.
    Dialect { dialect: String, mnemonic: String, raw: Option<String> },
}
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn write_list(
            f: &mut std::fmt::Formatter<'_>,
            values: &[Type],
        ) -> std::fmt::Result {
            for (i, value) in values.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{value}")?;
            }
            Ok(())
        }
        match self {
            Self::Named(name) => write!(f, "{name}"),
            Self::Integer(width) => write!(f, "i{width}"),
            Self::Index => write!(f, "index"),
            Self::FunctionType { inputs, results } => {
                write!(f, "(")?;
                write_list(f, inputs)?;
                write!(f, ") -> ")?;
                match results.as_slice() {
                    [one] => write!(f, "{one}"),
                    many => {
                        write!(f, "(")?;
                        write_list(f, many)?;
                        write!(f, ")")
                    }
                }
            }
            Self::Int { width, is_signed, is_bit_int } => {
                write!(f, "{}{width}", if * is_signed { "s" } else { "u" })?;
                if *is_bit_int {
                    write!(f, "_bitint")?;
                }
                Ok(())
            }
            Self::Bool => write!(f, "bool"),
            Self::Void => write!(f, "void"),
            Self::Single => write!(f, "float"),
            Self::Double => write!(f, "double"),
            Self::Fp16 => write!(f, "f16"),
            Self::Bf16 => write!(f, "bf16"),
            Self::Fp80 => write!(f, "f80"),
            Self::Fp128 => write!(f, "f128"),
            Self::LongDouble { underlying } => write!(f, "long_double<{underlying}>"),
            Self::Pointer { pointee, addr_space } => {
                write!(f, "{pointee}*")?;
                if let Some(raw) = addr_space {
                    write!(f, " {raw}")?;
                }
                Ok(())
            }
            Self::Array { element_type, size } => write!(f, "{element_type}[{size}]"),
            Self::Vector { element_type, size, .. } => {
                write!(f, "vector<{size} x {element_type}>")
            }
            Self::Func { inputs, optional_return_type, var_arg } => {
                write!(f, "(")?;
                write_list(f, inputs)?;
                if *var_arg {
                    if !inputs.is_empty() {
                        write!(f, ", ")?;
                    }
                    write!(f, "...")?;
                }
                write!(f, ") -> ")?;
                match optional_return_type {
                    Some(ty) => write!(f, "{ty}"),
                    None => write!(f, "void"),
                }
            }
            Self::Struct { name, .. } => {
                write!(f, "struct {}", name.as_deref().unwrap_or("<anon>"))
            }
            Self::Union { name, .. } => {
                write!(f, "union {}", name.as_deref().unwrap_or("<anon>"))
            }
            Self::Complex { element_type } => write!(f, "complex<{element_type}>"),
            Self::DataMember { member_ty, class_ty } => {
                write!(f, "data_member<{member_ty} in {class_ty}>")
            }
            Self::Method { member_func_ty, class_ty } => {
                write!(f, "method<{member_func_ty} in {class_ty}>")
            }
            Self::VPtr => write!(f, "vptr"),
            Self::EhToken => write!(f, "eh_token"),
            Self::CleanupToken => write!(f, "cleanup_token"),
            Self::CatchToken => write!(f, "catch_token"),
            Self::Dialect { dialect, mnemonic, raw } => {
                write!(f, "{dialect}.{mnemonic}")?;
                if let Some(raw) = raw {
                    write!(f, "<{raw}>")?;
                }
                Ok(())
            }
        }
    }
}