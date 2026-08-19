//! Global, function, and constant declaration operations.

/// `cir.const`
/// Create a CIR constant from a literal attribute
///
/// The `cir.const` operation turns a literal into an SSA value. The data is
/// attached to the operation as an attribute.
///
/// ```
///   %0 = cir.const #cir.int<4> : !u32i
///   %1 = cir.const #cir.fp<1.500000e+00> : !cir.float
///   %2 = cir.const #cir.ptr<null> : !cir.ptr<!void>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Const {
    pub res: super::ValueId,
    pub res_ty: crate::types::Type,
    /// TypedAttr instance
    pub value: crate::attrs::Attribute,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.func`
/// Declare or define a function
///
/// The `cir.func` operation defines a function, similar to the `mlir::FuncOp`
/// built-in.
///
/// The function linkage information is specified by `linkage`, as defined by
/// `GlobalLinkageKind` attribute.
///
/// The `calling_conv` attribute specifies the calling convention of the function.
/// By default calling convention is `CallingConv::C`. When printed, C calling
/// convention is omitted. Other calling conventions are printed as `cc(<mnemonic>)`,
/// e.g. `cc(amdgpu_kernel)`.
///
/// A compiler builtin function must be marked as `builtin` for further
/// processing when lowering from CIR.
///
/// The `coroutine` keyword is used to mark a coroutine function, which requires
/// at least one `cir.await` instruction to be used in its body.
///
/// The `lambda` translates to a C++ `operator()` that implements a lambda, this
/// allow callsites to make certain assumptions about the real function nature
/// when writing analysis.
///
/// The `no_proto` keyword is used to identify functions that were declared
/// without a prototype and, consequently, may contain calls with invalid
/// arguments and undefined behavior.
///
/// The `global_ctor` keyword indicates whether a function should execute before
/// `main()` function, as specified by `__attribute__((constructor))`. An
/// execution priority can also be specified `global_ctor(<priority>)`.
/// Similarly, for global destructors both `global_dtor` and
/// `global_dtor(<priority>)` are available.
///
/// The `no_inline` attribute marks a function that should not be inlined.
/// The `always_inline` attribute marks a function that should always be inlined.
/// The `inline_hint` attribute suggests that the function should be inlined.
///
/// The `personality` attribute specifies the personality function to use for
/// exception handling. This is a symbol reference to the personality function
/// (e.g., `@__gxx_personality_v0` for C++ exceptions).
///
/// Example:
///
/// ```
/// // External function definitions.
/// cir.func @abort()
///
/// // A function with internal linkage.
/// cir.func internal @count(%x: i64) -> (i64)
///   return %x : i64
///
/// // Linkage information
/// cir.func linkonce_odr @some_method(...)
///
/// // Calling convention information
/// cir.func @func1(...) cc(amdgpu_kernel)
///
/// // Inline information
/// cir.func no_inline @some_method(...)
///
/// // Builtin function
/// cir.func builtin @__builtin_coro_end(!cir.ptr<i8>, !cir.bool) -> !cir.bool
/// // Coroutine
/// cir.func coroutine @_Z10silly_taskv() -> !CoroTask {
///   ...
///   cir.await(...)
///   ...
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Func {
    /// string attribute
    pub sym_name: String,
    /// C/C++ visibility
    pub global_visibility: bool,
    /// type attribute of CIR function type
    pub function_type: crate::types::Type,
    /// unit attribute
    pub builtin: bool,
    /// unit attribute
    pub coroutine: bool,
    /// Inline kind attribute
    pub inline_kind: Option<crate::enums::InlineKind>,
    /// unit attribute
    pub lambda: bool,
    /// unit attribute
    pub no_proto: bool,
    /// unit attribute
    pub dso_local: bool,
    /// linkage kind
    pub linkage: crate::attrs::Attribute,
    /// calling convention
    pub calling_conv: crate::attrs::Attribute,
    /// string attribute
    pub sym_visibility: Option<String>,
    /// unit attribute
    pub comdat: bool,
    /// Array of dictionary attributes
    pub arg_attrs: Option<crate::attrs::Attribute>,
    /// Array of dictionary attributes
    pub res_attrs: Option<crate::attrs::Attribute>,
    /// flat symbol reference attribute
    pub aliasee: Option<crate::attrs::Attribute>,
    /// allowed side effects of a function
    pub side_effect: Option<crate::enums::SideEffect>,
    /// flat symbol reference attribute
    pub personality: Option<crate::attrs::Attribute>,
    /// 32-bit signless integer attribute whose minimum value is 101 whose maximum value is 65535
    pub global_ctor_priority: Option<crate::attrs::Attribute>,
    /// 32-bit signless integer attribute whose minimum value is 101 whose maximum value is 65535
    pub global_dtor_priority: Option<crate::attrs::Attribute>,
    /// function information attribute
    pub func_info: Option<crate::attrs::Attribute>,
    /// array of cir.annotation attributes
    pub annotations: Option<crate::attrs::Attribute>,
    pub body: super::Region,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.get_global`
/// Get the address of a global variable
///
/// The `cir.get_global` operation retrieves the address pointing to a
/// named global variable. If the global variable is marked constant, writing
/// to the resulting address (such as through a `cir.store` operation) is
/// undefined. The resulting type must always be a `!cir.ptr<...>` type with the
/// same address space as the global variable.
///
/// Addresses of thread local globals can only be retrieved if this operation
/// is marked `thread_local`, which indicates the address isn't constant.
///
/// The `static_local` attribute indicates that this global is a function-local
/// static variable that requires guarded initialization (e.g., C++ static
/// local variables with non-constant initializers).
///
/// Example:
/// ```
/// %x = cir.get_global @gv : !cir.ptr<i32>
/// ...
/// %y = cir.get_global thread_local @tls_gv : !cir.ptr<i32>
/// ...
/// %w = cir.get_global static_local @func_static : !cir.ptr<i32>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetGlobal {
    pub addr: super::ValueId,
    pub addr_ty: crate::types::Type,
    /// flat symbol reference attribute
    pub name: crate::attrs::Attribute,
    /// unit attribute
    pub tls: bool,
    /// unit attribute
    pub static_local: bool,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.global`
/// Declare or define a global variable
///
/// The `cir.global` operation declares or defines a named global variable.
///
/// The backing memory for the variable is allocated statically and is
/// described by the type of the variable.
///
/// The `linkage` tracks C/C++ linkage types, currently very similar to LLVM's.
/// Symbol visibility in `sym_visibility` is defined in terms of MLIR's visibility
/// and verified to be in accordance to `linkage`.
///
/// The `static_local_guard` attribute indicates that this global represents a
/// function-local static variable that requires guarded initialization
/// (e.g., C++ static local variables with non-constant initializers).
/// It contains the mangled name of the guard variable.
///
/// The `strictfp` attribute indicates that the dynamic initialization emitted
/// into the `ctorRegion`/`dtorRegion` runs under a constrained floating-point
/// environment. LoweringPrepare forwards it to the `strictfp` attribute of the
/// generated `__cxx_global_var_init` function.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Global {
    /// string attribute
    pub sym_name: String,
    /// C/C++ visibility
    pub global_visibility: bool,
    /// string attribute
    pub sym_visibility: Option<String>,
    /// any type attribute
    pub sym_type: crate::types::Type,
    /// linkage kind
    pub linkage: crate::enums::GlobalLinkageKind,
    /// MemorySpaceAttrInterface instance
    pub addr_space: Option<crate::attrs::Attribute>,
    /// TLS Model attribute
    pub tls_model: Option<crate::enums::TlsModel>,
    /// Wrapper and Init function names for thread local variables
    pub tls_refs: Option<crate::attrs::Attribute>,
    /// any attribute
    pub initial_value: Option<crate::attrs::Attribute>,
    /// unit property
    pub comdat: bool,
    /// unit property
    pub constant: bool,
    /// unit property
    pub dso_local: bool,
    /// Guard variable name for static local variables
    pub static_local_guard: Option<crate::attrs::Attribute>,
    /// 64-bit signless integer attribute
    pub alignment: Option<crate::attrs::Attribute>,
    /// ASTVarDeclInterface instance
    pub ast: Option<crate::attrs::Attribute>,
    /// string attribute
    pub section: Option<String>,
    /// array of cir.annotation attributes
    pub annotations: Option<crate::attrs::Attribute>,
    /// flat symbol reference attribute
    pub aliasee: Option<crate::attrs::Attribute>,
    /// unit attribute
    pub strictfp: bool,
    pub ctor_region: super::Region,
    pub dtor_region: super::Region,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.local_init`
/// initialize a static or thread local object
///
/// The 'cir.local_init' operation has no result, but is responsible for
/// containing the regions to initialize and destroy the static local
/// variable. This will be handled during lowering-prepare to include the
/// guard variables correctly for the variable.
///
/// This operation may also represent a static local thread local variable,
/// which would be indicated by the 'tls' flag.
///
/// Example:
/// ```
/// // Note: despite this always being static_local, we print it anyway to be
/// // visually consistent with get_global, and as a 'counter' to 'tls'.
/// cir.local_init static_local @GlobalName ctor {
///   %4 = cir.get_global static_local @GlobalName : !cir.ptr<!rec_CtorDtor>
///   %5 = cir.call @_Z5get_iv() : () -> !s32i
///   cir.call @_ZN8CtorDtorC1Ei(%4, %5) : !cir.ptr<!rec_CtorDtor>
///   cir.yield
/// }, dtor {
///   %4 = cir.get_global static_local @_ZZ3foovE8localCD2 :
///                                               !cir.ptr<!rec_CtorDtor>
///   cir.call @_ZN8CtorDtorD1Ev(%4) : (!cir.ptr<!rec_CtorDtor>) -> ()
///   cir.yield
/// }
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LocalInit {
    /// flat symbol reference attribute
    pub global_name: crate::attrs::Attribute,
    /// unit property
    pub tls: bool,
    pub ctor_region: super::Region,
    pub dtor_region: super::Region,
    pub loc: Option<crate::ast::SourceLocation>,
}