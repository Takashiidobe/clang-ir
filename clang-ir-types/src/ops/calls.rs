//! Call-like operations.

/// `cir.call`
/// call a function
///
/// The `cir.call` operation represents a function call. It could represent
/// either a direct call or an indirect call.
///
/// If the operation represents a direct call, the callee should be defined
/// within the same symbol scope as the call. The `callee` attribute contains a
/// symbol reference to the callee function. All operands of this operation are
/// arguments to the callee function.
///
/// If the operation represents an indirect call, the `callee` attribute is
/// empty. The first operand of this operation must be a pointer to the callee
/// function. The rest operands are arguments to the callee function.
///
/// The `calling_conv` attribute specifies the calling convention used at the
/// call site. It defaults to `CallingConv::C`, which is omitted when printed.
/// Other calling conventions are printed as `cc(<mnemonic>)`, e.g.
/// `cc(spir_function)`.
///
/// Example:
///
/// ```
/// %0 = cir.call @foo()
/// cir.call @bar() cc(spir_function) : () -> ()
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Call {
    pub result: Option<super::ValueId>,
    pub result_ty: Option<crate::types::Type>,
    pub indirect_callee: Option<super::ValueId>,
    pub arg_types: Vec<crate::types::Type>,
    pub noreturn: bool,
    /// flat symbol reference attribute
    pub callee: Option<crate::attrs::Attribute>,
    /// variadic of CIR void type or CIR bool type or CIR array type or CIR vector type or CIR matrix type or Integer type with arbitrary precision up to a fixed limit or single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type or CIR pointer type or CIR function type or CIR struct/class type or CIR union type or CIR bit-field access unit member type or CIR complex type or CIR type that is used for the vptr member of C++ objects or CUDA device surface reference type or CUDA device texture reference type or CIR type that represents a pointer-to-data-member in C++ or CIR type that represents C++ pointer-to-member-function type or CIR exception handling token type or CIR cleanup token type or CIR catch token type
    pub args: Vec<super::ValueId>,
    /// unit attribute
    pub nothrow: bool,
    /// unit attribute
    pub nounwind: bool,
    /// unit attribute
    pub willreturn: bool,
    /// inline kind
    pub inline_kind: Option<crate::enums::InlineKind>,
    /// unit attribute
    pub musttail: bool,
    /// Describes the possible memory effects of a function or callee
    pub memory_effects: Option<crate::attrs::Attribute>,
    /// Array of dictionary attributes
    pub arg_attrs: Option<crate::attrs::Attribute>,
    /// Array of dictionary attributes
    pub res_attrs: Option<crate::attrs::Attribute>,
    /// calling convention
    pub calling_conv: crate::attrs::Attribute,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.call_llvm_intrinsic`
/// Call to llvm intrinsic functions that is not defined in CIR
///
/// `cir.call_llvm_intrinsic` operation represents a call-like expression which has
/// return type and arguments that maps directly to a llvm intrinsic.
/// It only records intrinsic `intrinsic_name`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CallLlvmIntrinsic {
    pub result: Option<super::ValueId>,
    pub result_ty: Option<crate::types::Type>,
    /// string attribute
    pub intrinsic_name: String,
    /// fast-math flags
    pub fastmath_flags: Option<crate::enums::FastMathFlags>,
    /// variadic of CIR void type or CIR bool type or CIR array type or CIR vector type or CIR matrix type or Integer type with arbitrary precision up to a fixed limit or single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type or CIR pointer type or CIR function type or CIR struct/class type or CIR union type or CIR bit-field access unit member type or CIR complex type or CIR type that is used for the vptr member of C++ objects or CUDA device surface reference type or CUDA device texture reference type or CIR type that represents a pointer-to-data-member in C++ or CIR type that represents C++ pointer-to-member-function type or CIR exception handling token type or CIR cleanup token type or CIR catch token type
    pub arg_ops: Vec<super::ValueId>,
    pub loc: Option<crate::ast::SourceLocation>,
}