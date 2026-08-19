//! Operations without a more specific category.

/// `cir.add.overflow`
/// Integer addition with overflow checking
///
/// `cir.add.overflow` performs addition with overflow checking on integral
/// operands. See `CIR_BinOpOverflow` for semantics.
///
/// Example:
///
/// ```
/// %result, %overflow = cir.add.overflow %a, %b : !u32i -> !u32i
/// %result, %overflow = cir.add.overflow %a, %b : !cir.int<s, 33> -> !s32i
/// %result, %overflow = cir.add.overflow %a, %b : !s32i -> !cir.bool
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddOverflow {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    pub overflow: super::ValueId,
    pub overflow_ty: crate::types::Type,
    /// Integer type with arbitrary precision up to a fixed limit
    pub lhs: super::ValueId,
    /// Integer type with arbitrary precision up to a fixed limit
    pub rhs: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.address_of_return_address`
/// The place stores the return address of the current function
///
/// Represents a call to builtin function `_AddressOfReturnAddress` in CIR.
/// This builtin function returns a pointer to the place in the stack frame
/// where the return address of the current function is stored.
///
/// Examples:
///
/// ```
/// %addr = address_of_return_address() : !cir.ptr<!u8i>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddressOfReturnAddress {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.asm`
///
/// The `cir.asm` operation represents C/C++ asm inline.
///
/// CIR constraints strings follow the same rules that are established for
/// the C level assembler constraints with several differences caused by
/// clang::AsmStmt processing.
///
/// Thus, numbers that appears in the constraint string may also refer to:
/// - the output variable index referenced by the input operands.
/// - the index of early-clobber operand
///
/// Operand attributes are a storage, where each element corresponds to the
/// operand with the same index. The first index relates to the operation
/// result (if any).
/// The operands themselves are stored as VariadicOfVariadic in the following
/// order: output, input and then in/out operands. When several output operands
/// are present, the result type may be represented as an anonymous record type.
///
/// Example:
/// ```C++
/// __asm__("foo" : : : );
/// __asm__("bar $42 %[val]" : [val] "=r" (x), "+&r"(x));
/// __asm__("baz $42 %[val]" : [val] "=r" (x), "+&r"(x) : "[val]"(y));
/// ```
///
/// ```
/// !rec_22anon2E022 = !cir.record<struct "anon.0" {!cir.int<s, 32>, !cir.int<s, 32>}>
/// !rec_22anon2E122 = !cir.record<struct "anon.1" {!cir.int<s, 32>, !cir.int<s, 32>}>
/// ...
/// %0 = cir.alloca "x" align(4) init : !cir.ptr<!s32i>
/// %1 = cir.alloca "y" align(4) init : !cir.ptr<!s32i>
/// ...
/// %2 = cir.load %0 : !cir.ptr<!s32i>, !s32i
/// %3 = cir.load %1 : !cir.ptr<!s32i>, !s32i
///
/// cir.asm(x86_att,
///   out = [],
///   in = [],
///   in_out = [],
///   {"foo" "~{dirflag},~{fpsr},~{flags}"}) side_effects
///
/// cir.asm(x86_att,
///   out = [],
///   in = [],
///   in_out = [%2 : !s32i],
///   {"bar $$42 $0" "=r,=&r,1,~{dirflag},~{fpsr},~{flags}"}) -> !rec_22anon2E022
///
/// cir.asm(x86_att,
///   out = [],
///   in = [%3 : !s32i],
///   in_out = [%2 : !s32i],
///   {"baz $$42 $0" "=r,=&r,0,1,~{dirflag},~{fpsr},~{flags}"}) -> !rec_22anon2E122
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Asm {
    pub res: Option<super::ValueId>,
    pub res_ty: Option<crate::types::Type>,
    /// variadic of any non-token type
    pub asm_operands: Vec<Vec<super::ValueId>>,
    /// string attribute
    pub asm_string: String,
    /// string attribute
    pub constraints: String,
    /// unit attribute
    pub side_effects: bool,
    /// ATT or Intel
    pub asm_flavor: crate::enums::AsmFlavor,
    /// array attribute
    pub operand_attrs: crate::attrs::Attribute,
    /// i32 dense array attribute
    pub operands_segments: crate::attrs::Attribute,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.assume`
/// Tell the optimizer that a boolean value is true
///
/// The `cir.assume` operation takes a single boolean predicate as its first
/// argument and does not have any results. The operation tells the optimizer
/// that the predicate is always true.
///
/// An optional operand bundle carries additional assumption metadata,
/// mirroring MLIR LLVM dialect's `llvm.assume` operand bundles
/// (`mlir/include/mlir/Dialect/LLVMIR/LLVMIntrinsicOps.td`).  The bundle
/// kind is a `AssumeBundleKind` enum; bundle operands follow the kind
/// keyword in the assembly form.  For example,
/// `__builtin_assume_dereferenceable(p, n)` lowers to
/// `cir.assume %true dereferenceable(%p, %n : !cir.ptr<!void>, !u64i)
/// : !cir.bool`, which in turn lowers to
/// `call void @llvm.assume(i1 true) [ "dereferenceable"(ptr, i64) ]`.
///
/// This operation corresponds to the `__assume` / `__builtin_assume`
/// builtins (no bundle), as well as `__builtin_assume_aligned`
/// (`align` bundle), `__builtin_assume_separate_storage`
/// (`separate_storage` bundle), and `__builtin_assume_dereferenceable`
/// (`dereferenceable` bundle).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Assume {
    /// CIR bool type
    pub predicate: super::ValueId,
    /// kind of cir.assume operand bundle
    pub bundle_kind: crate::attrs::Attribute,
    /// variadic of CIR void type or CIR bool type or CIR array type or CIR vector type or Integer type with arbitrary precision up to a fixed limit or single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type or CIR pointer type or CIR function type or CIR struct/class type or CIR union type or CIR complex type or CIR type that is used for the vptr member of C++ objects or CIR type that represents a pointer-to-data-member in C++ or CIR type that represents C++ pointer-to-member-function type or CIR exception handling token type or CIR cleanup token type or CIR catch token type
    pub bundle_args: Vec<super::ValueId>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.await`
/// Wraps C++ co_await implicit logic
///
/// The under the hood effect of using C++ `co_await expr` roughly
/// translates to:
///
/// ```c++
/// // co_await expr;
///
/// auto &&x = CommonExpr();
/// if (!x.await_ready()) {
///    ...
///    x.await_suspend(...);
///    ...
/// }
/// x.await_resume();
/// ```
///
/// `cir.await` represents this logic by using 3 regions:
///   - ready: covers veto power from x.await_ready()
///   - suspend: wraps actual x.await_suspend() logic
///   - resume: handles x.await_resume()
///
/// Breaking this up in regions allows individual scrutiny of conditions
/// which might lead to folding some of them out. Lowerings coming out
/// of CIR, e.g. LLVM, should use the `suspend` region to track more
/// lower level codegen (e.g. intrinsic emission for coro.save/coro.suspend).
///
/// There are also 4 flavors of `cir.await` available:
/// - `init`: compiler generated initial suspend via implicit `co_await`.
/// - `user`: also known as normal, representing a user written `co_await`.
/// - `yield`: user written `co_yield` expressions.
/// - `final`: compiler generated final suspend via implicit `co_await`.
///
/// ```
///   cir.scope {
///     ... // auto &&x = CommonExpr();
///     cir.await(user, ready : {
///       ... // x.await_ready()
///     }, suspend : {
///       ... // x.await_suspend()
///     }, resume : {
///       ... // x.await_resume()
///     })
///   }
/// ```
///
/// Note that resulution of the common expression is assumed to happen
/// as part of the enclosing await scope.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Await {
    /// await kind
    pub kind: crate::enums::AwaitKind,
    pub ready: super::Region,
    pub suspend: super::Region,
    pub resume: super::Region,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.bitreverse`
/// Reverse the bit pattern of the operand integer
///
/// The `cir.bitreverse` operation reverses the bits of the operand integer. Its
/// only argument must be of unsigned integer types of width 8, 16, 32, or 64.
///
/// Example:
///
/// ```
/// %1 = cir.bitreverse %0: !u32i
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bitreverse {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// unsigned integer type of widths 8/16/32/64
    pub input: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.block_address`
/// Get the address of a cir.label within a function
///
///  The `cir.blockaddress` operation takes a function name and a label and
///  produces a pointer value that represents the address of that cir.label
///  within the specified function.
///
///  This operation models GCC's "labels as values" extension (`&&label`), which
///  allows taking the address of a local label and using it as a computed
///  jump target (e.g., with `goto *addr;`).
///
///  Example:
///  ```
///  %1 = cir.alloca "ptr" align(8) init : !cir.ptr<!cir.ptr<!void>>
///  %addr = cir.block_address <@c, "label1"> : !cir.ptr<!cir.void>
///  cir.store align(8) %addr, %1 : !cir.ptr<!void>, !cir.ptr<!cir.ptr<!void>>
///  cir.br ^bb1
/// ^bb1:
///  cir.label "label"
///  ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BlockAddress {
    pub addr: super::ValueId,
    pub addr_ty: crate::types::Type,
    /// Block address attribute
    pub block_addr_info: crate::attrs::Attribute,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.builtin_int_cast`
/// Cast between a CIR integer and a builtin integer
///
/// Convert between a CIR integer type (`!cir.int`) and a builtin MLIR integer
/// type (`AnyInteger`, e.g. `i32`, `si32`, `ui32`) or `index`, and vice versa.
///
/// This allows using operations from e.g. OpenMP or OpenACC dialects
/// that expect the builtin types with CIR operations. Casting can be done
/// in either direction.
///
/// Example:
///
/// ```mlir
/// // CIR integer cast to a builtin integer.
/// %0 = cir.builtin_int_cast %ciri : !cir.int<s, 32> -> i32
///
/// // Builtin induction variable / bound cast to CIR type.
/// %1 = cir.builtin_int_cast %iv : index -> !cir.int<u, 64>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BuiltinIntCast {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// Integer type with arbitrary precision up to a fixed limit or integer or index
    pub src: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.cast`
/// Conversion between values of different types
///
/// Apply the usual C/C++ conversion rules between values. This operation models
/// a subset of conversions as defined in Clang's `OperationKinds.def`
/// (`llvm-project/clang/include/clang/AST/OperationKinds.def`).
///
/// Note: not all conversions are implemented using `cir.cast`. For instance,
/// lvalue-to-rvalue conversion is modeled as a `cir.load` instead.  Currently
/// supported kinds:
///
/// - `bitcast`
/// - `array_to_ptrdecay`
/// - `member_ptr_to_bool
/// - `int_to_ptr`
/// - `ptr_to_int`
/// - `ptr_to_bool`
/// - `integral`
/// - `int_to_bool`
/// - `int_to_float`
/// - `float_to_int`
/// - `float_to_bool`
/// - `bool_to_int`
/// - `floating`
/// - `float_complex`
/// - `int_complex_to_real`
/// - `int_complex_to_bool`
/// - `int_complex`
/// - `int_complex_to_float_complex`
/// - `address_space`
///
/// CIR also supports some additional conversions that are not part of the classic
/// Clang codegen:
///
/// - `bool_to_float`
///
/// The optional `fenv` attribute describes constraints on the floating-point
/// handling of the operation. It is only valid for floating-point cast kinds:
/// `floating`, `int_to_float`, `float_to_int`, `float_to_bool`,
/// `bool_to_float`, `float_to_complex`, `float_complex_to_real`,
/// `float_complex_to_bool`, `float_complex`, `float_complex_to_int_complex`,
/// and `int_complex_to_float_complex`.
///
/// Example:
///
/// ```
/// %4 = cir.cast int_to_bool %3 : i32 -> !cir.bool
/// ...
/// %x = cir.cast array_to_ptrdecay %0
///    : !cir.ptr<!cir.array<i32 x 10>> -> !cir.ptr<i32>
/// %y = cir.cast floating %f : !cir.double -> !cir.float {
///   fenv = #cir.fenv<>
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cast {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// cast kind
    pub kind: crate::enums::CastKind,
    /// CIR void type or CIR bool type or CIR array type or CIR vector type or Integer type with arbitrary precision up to a fixed limit or single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type or CIR pointer type or CIR function type or CIR struct/class type or CIR union type or CIR complex type or CIR type that is used for the vptr member of C++ objects or CIR type that represents a pointer-to-data-member in C++ or CIR type that represents C++ pointer-to-member-function type or CIR exception handling token type or CIR cleanup token type or CIR catch token type
    pub src: super::ValueId,
    /// Describes floating-point environment constraints
    pub fenv: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.clrsb`
/// Get the number of leading redundant sign bits in the input
///
/// Compute the number of leading redundant sign bits in the input integer.
///
/// The input integer must be a signed integer. The most significant bit of the
/// input integer is the sign bit. The `cir.clrsb` operation returns the number
/// of consecutive bits following the sign bit that are identical to the sign
/// bit.
///
/// The bit width of the input integer must be either 32 or 64.
///
/// Examples:
///
/// ```
/// // %0 = 0b1101_1110_1010_1101_1011_1110_1110_1111
/// %0 = cir.const #cir.int<3735928559> : !s32i
/// // %1 will be 1 because there is 1 bit following the most significant bit
/// // that is identical to it.
/// %1 = cir.clrsb %0 : !s32i
///
/// // %2 = 1, 0b0000_0000_0000_0000_0000_0000_0000_0001
/// %2 = cir.const #cir.int<1> : !s32i
/// // %3 will be 30 because there are 30 consecutive bits following the sign
/// // bit that are identical to the sign bit.
/// %3 = cir.clrsb %2 : !s32i
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Clrsb {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// signed integer type of widths 32/64
    pub input: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.clz`
/// Get the number of leading 0-bits in the input
///
/// Compute the number of leading 0-bits in the input.
///
/// The input integer must be an unsigned integer. The `cir.clz` operation
/// returns the number of consecutive 0-bits at the most significant bit
/// position in the input.
///
/// If the `poison_zero` attribute is present, this operation will have
/// undefined behavior if the input value is 0.
///
/// Example:
///
/// ```
/// // %0 = 0b0000_0000_0000_0000_0000_0000_0000_1000
/// %0 = cir.const #cir.int<8> : !u32i
/// // %1 will be 28
/// %1 = cir.clz %0 poison_zero : !u32i
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Clz {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// unsigned integer type of widths 8/16/32/64/128
    pub input: super::ValueId,
    /// unit attribute
    pub poison_zero: bool,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.co_return`
/// Coroutine return operation
///
/// The `cir.co_return` operation models a coroutine return point inside a
/// `cir.coro.body` region.
/// This operation is expected to appear only within a `cir.coro.body` region,
/// but it may be nested within other operations or regions inside that body.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CoReturn {
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.coro.body`
/// Region containing the user-authored coroutine body
///
/// The `cir.coro.body` operation models the region where the user-authored
/// coroutine code is emitted.
///
/// This operation serves as a structural boundary separating the coroutine
/// setup and teardown logic (e.g. initial suspend, final suspend, and cleanup)
/// from the user-provided statements inside the coroutine.
///
/// The body region contains the code corresponding to the original function
/// body, including `co_await` and `co_return` expressions. In particular,
/// `cir.co_return` operations inside this region mark coroutine exit points
/// and introduce structured control flow that transfers execution to the
/// final suspend point of the coroutine.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CoroBody {
    pub body: super::Region,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.coro.intrinsic.alloc`
/// Represents llvm.coro.alloc
///
/// Queries whether the coroutine identified by `id` needs a dynamically
/// allocated frame. Returns `true` if the coroutine frame must be allocated,
/// or `false` otherwise.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CoroIntrinsicAlloc {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// token
    pub id: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.coro.intrinsic.begin`
/// Represents llvm.coro.begin
///
/// Initializes the coroutine frame using `coroframeAddr`. `id` is the token
/// from `coro.intrinsic.id`, and `coroframeAddr` points to the memory used
/// for the coroutine frame. Returns the coroutine handle.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CoroIntrinsicBegin {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// token
    pub id: super::ValueId,
    /// pointer to void type
    pub coroframe_addr: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.coro.intrinsic.end`
/// Represents llvm.coro.end
///
/// Marks a point at which a coroutine must be suspended or destroyed for the
/// last time, e.g. right before the coroutine returns control to its caller
/// for the final time, or along an exceptional unwind path. `handle` is the
/// coroutine handle produced by `coro.intrinsic.begin`, and `unwind`
/// indicates whether this occurrence of `coro.intrinsic.end` lies on the
/// unwind path (`true`) or the normal control-flow path (`false`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CoroIntrinsicEnd {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// pointer to void type
    pub handle: super::ValueId,
    /// boolean type
    pub unwind: super::ValueId,
    /// token
    pub result_token: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.coro.intrinsic.free`
/// Represents llvm.coro.free
///
/// Given the coroutine identified by `id` and its frame pointer `coroframe`
/// (the handle from `coro.intrinsic.begin`), returns the pointer that must
/// be passed to the deallocation function to free the coroutine frame, or a
/// null pointer if the coroutine frame was not dynamically allocated.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CoroIntrinsicFree {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// token
    pub id: super::ValueId,
    /// pointer to void type
    pub coroframe: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.coro.intrinsic.id`
/// Represents llvm.coro.id
///
/// Marks the beginning of a coroutine's lifetime and identifies it to the
/// rest of the coroutine intrinsics. Takes the required alignment of the
/// coroutine frame, a pointer to the coroutine promise (or null if none),
/// the address of the coroutine function itself, and an opaque pointer used
/// by the frontend to convey additional information to the coroutine
/// lowering passes (or null).
///
/// The result is a token that must be passed to every other
/// `coro.intrinsic.*` operation associated with this coroutine
/// (`coro.alloc`, `coro.begin`, `coro.free`, etc.), tying them all to the
/// same coroutine instance.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CoroIntrinsicId {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// integer type
    pub align: super::ValueId,
    /// pointer to void type
    pub promise: super::ValueId,
    /// pointer to void type
    pub coroaddr: super::ValueId,
    /// pointer to void type
    pub fnaddrs: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.coro.intrinsic.size`
/// Represents llvm.coro.size
///
/// Returns the size, in bytes, of the coroutine frame.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CoroIntrinsicSize {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.cpuid`
/// Get information about the CPU
///
/// The `cir.cpuid` operation retrieves different types of CPU information and
/// stores it in an array of 4 integers.
///
/// This operation takes 3 arguments: `cpu_info`, a pointer to an array of 4
/// integers; `function_id`, an integer determining what type of information to
/// be retrieved (for instance, basic information, processor information and
/// features, or cache/TLB information); and `sub_function_id`, an integer that
/// adds more detail about what information is requested.
///
/// As a result, the array of 4 integers is filled with the requested
/// information.
///
/// Example:
///
/// ```
/// cir.cpuid %cpui_info, %function_id, %sub_function_id : (!cir.ptr<!s32i>,
///     !s32i, !s32i)
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cpuid {
    /// array address
    pub cpu_info: super::ValueId,
    /// 32-bit signed integer
    pub function_id: super::ValueId,
    /// 32-bit signed integer
    pub sub_function_id: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.ctz`
/// Get the number of trailing 0-bits in the input
///
/// Compute the number of trailing 0-bits in the input.
///
/// The input integer must be an unsigned integer. The `cir.ctz` operation
/// counts the number of consecutive 0-bits starting from the least significant
/// bit.
///
/// If the `poison_zero` attribute is present, this operation will have
/// undefined behavior if the input value is 0.
///
/// Example:
///
/// ```
/// // %0 = 0b1000
/// %0 = cir.const #cir.int<8> : !u32i
/// // %1 will be 3
/// %1 = cir.ctz %0 poison_zero : !u32i
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ctz {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// unsigned integer type of widths 8/16/32/64/128
    pub input: super::ValueId,
    /// unit attribute
    pub poison_zero: bool,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.dec`
/// Decrement an integer by one
///
/// The `cir.dec` operation decrements the operand by one. The operand and
/// result must have the same type.
///
/// The optional `nsw` (no signed wrap) attribute indicates that the result
/// is poison if signed overflow occurs.
///
/// Example:
///
/// ```
/// %1 = cir.dec %0 : !s32i
/// %3 = cir.dec nsw %2 : !s32i
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dec {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// integer or vector of integer type
    pub input: super::ValueId,
    /// unit property
    pub no_signed_wrap: bool,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.delete_array`
/// Delete address representing an array
///
/// `cir.delete_array` operation deletes an array. For example, `delete[] ptr;`
/// will be translated to `cir.delete_array %ptr`.
///
/// The `delete_fn` attribute specifies the operator delete function to call.
/// The `delete_params` attribute describes the parameters needed by the
/// operator delete call.
///
/// The `element_dtor` attribute, when present, specifies the destructor to call
/// on each array element before deallocation.
///
/// The `dtor_may_throw` unit property, when present, indicates that the
/// element destructor may throw exceptions.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeleteArray {
    /// CIR pointer type
    pub address: super::ValueId,
    /// flat symbol reference attribute
    pub delete_fn: crate::attrs::Attribute,
    /// Parameters describing the usual operator delete signature
    pub delete_params: crate::attrs::Attribute,
    /// flat symbol reference attribute
    pub element_dtor: Option<crate::attrs::Attribute>,
    /// unit property
    pub dtor_may_throw: bool,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.dyn_cast`
/// Perform dynamic cast on record pointers
///
/// The `cir.dyn_cast` operation models part of the semantics of the
/// `dynamic_cast` operator in C++. It can be used to perform 3 kinds of casts
/// on record pointers:
///
/// - Down-cast, which casts a base class pointer to a derived class pointer;
/// - Side-cast, which casts a class pointer to a sibling class pointer;
/// - Cast-to-complete, which casts a class pointer to a void pointer.
///
/// The input of the operation must be a record pointer. The result of the
/// operation is either a record pointer or a void pointer.
///
/// The parameter `kind` specifies the semantics of this operation. If its value
/// is `ptr`, then the operation models dynamic casts on pointers. Otherwise, if
/// its value is `ref`, the operation models dynamic casts on references.
/// Specifically:
///
/// - When the input pointer is a null pointer value:
///   - If `kind` is `ref`, the operation will invoke undefined behavior. A
///     sanitizer check will be emitted if sanitizer is on.
///   - Otherwise, the operation will return a null pointer value as its result.
/// - When the runtime type check fails:
///   - If `kind` is `ref`, the operation will throw a `bad_cast` exception.
///   - Otherwise, the operation will return a null pointer value as its result.
///
/// The `info` argument gives detailed information about the requested dynamic
/// cast operation. It is an optional `#cir.dyn_cast_info` attribute that is
/// only present when the operation models a down-cast or a side-cast.
///
/// The `relative_layout` argument specifies whether the Itanium C++ ABI vtable
/// uses relative layout. It is only meaningful when the operation models a
/// cast-to-complete operation.
///
/// Examples:
///
/// ```
/// %0 = cir.dyn_cast ptr %p : !cir.ptr<!rec_Base> -> !cir.ptr<!rec_Derived>
/// %1 = cir.dyn_cast ptr relative_layout %p : !cir.ptr<!rec_Base>
///           -> !cir.ptr<!rec_Derived>
/// %2 = cir.dyn_cast ref %r : !cir.ptr<!rec_Base> -> !cir.ptr<!rec_Derived>
///           #cir.dyn_cast_info<
///             srcRtti = #cir.global_view<@_ZTI4Base> : !cir.ptr<!u8i>,
///             destRtti = #cir.global_view<@_ZTI7Derived> : !cir.ptr<!u8i>,
///             runtimeFunc = @__dynamic_cast,
///             badCastFunc = @__cxa_bad_cast,
///             offsetHint = #cir.int<0> : !s64i
///           >
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DynCast {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// dynamic cast kind
    pub kind: crate::enums::DynamicCastKind,
    /// pointer to record type
    pub src: super::ValueId,
    /// ABI specific information about a dynamic cast
    pub info: Option<crate::attrs::Attribute>,
    /// unit attribute
    pub relative_layout: bool,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.expect`
/// Tell the optimizer that two values are likely to be equal.
///
/// The `cir.expect` operation may take 2 or 3 arguments.
///
/// When the argument `prob` is missing, this operation effectively models the
/// `__builtin_expect` builtin function. It tells the optimizer that `val` and
/// `expected` are likely to be equal.
///
/// When the argument `prob` is present, this operation effectively models the
/// `__builtin_expect_with_probability` builtin function. It tells the
/// optimizer that `val` and `expected` are equal to each other with a certain
/// probability.
///
/// `val` and `expected` must be integers and their types must match.
///
/// The result of this operation is always equal to `val`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Expect {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// fundamental integer type
    pub val: super::ValueId,
    /// fundamental integer type
    pub expected: super::ValueId,
    /// 64-bit float attribute
    pub prob: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.ffs`
/// Get the position of the least significant 1-bit in input
///
/// Compute the 1-based position of the least significant 1-bit of the input.
///
/// The input integer must be a signed integer. The `cir.ffs` operation returns
/// one plus the index of the least significant 1-bit of the input signed
/// integer. If the input integer is 0, `cir.ffs` yields 0.
///
/// Example:
///
/// ```
/// !s32i = !cir.int<s, 32>
///
/// // %0 = 0x0010_1000
/// %0 = cir.const #cir.int<40> : !s32i
/// // #1 will be 4 since the 4th least significant bit is 1.
/// %1 = cir.ffs %0 : !s32i
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ffs {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// signed integer type of widths 32/64
    pub input: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.frame_address`
/// The frame address of the current function, or of one of its callers
///
/// Represents a call to builtin function ` __builtin_frame_address` in CIR.
/// This builtin function returns the frame address of the current function,
/// or of one of its callers. The frame is the area on the stack that holds
/// local variables and saved registers. The frame address is normally the
/// address of the first word pushed on to the stack by the function.
/// However, the exact definition depends upon the processor and the calling
/// convention. If the processor has a dedicated frame pointer register, and
/// the function has a frame, then __builtin_frame_address returns the value of
/// the frame pointer register.
///
/// The `level` argument is number of frames to scan up the call stack.
/// For instance, value of 0 yields the frame address of the current function,
/// value of 1 yields the frame address of the caller of the current function,
/// and so forth.
///
/// Examples:
///
/// ```
/// %p = frame_address(%level) : !cir.ptr<!u8i>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FrameAddress {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// 32-bit unsigned integer
    pub level: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.inc`
/// Increment an integer by one
///
/// The `cir.inc` operation increments the operand by one. The operand and
/// result must have the same type.
///
/// The optional `nsw` (no signed wrap) attribute indicates that the result
/// is poison if signed overflow occurs.
///
/// Example:
///
/// ```
/// %1 = cir.inc %0 : !s32i
/// %3 = cir.inc nsw %2 : !s32i
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Inc {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// integer or vector of integer type
    pub input: super::ValueId,
    /// unit property
    pub no_signed_wrap: bool,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.indirect_br`
/// Indirect branch
///
/// The `cir.indirectbr` operation represents an indirect branch to one of
/// several possible successor blocks. The target block is computed from
/// the value of the given address operand.
///
/// This operation is typically generated when handling constructs like
/// the GCC extension `&&label` combined with an indirect `goto *ptr;`.
///
/// The `poison` attribute is used to mark an `indirectbr` that was created
/// but is known to be invalid, for instance when a label address was
/// taken but no indirect branch was ever emitted.
///
/// Example:
///
/// ```
///   %0 = cir.block_address <@A, "A"> : !cir.ptr<!void>
///   cir.indirectbr %0 poison : <!void>, [
///   ^bb1
///   ]
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IndirectBr {
    /// pointer to void type
    pub addr: super::ValueId,
    /// unit attribute
    pub poison: bool,
    /// variadic of any non-token type
    pub succ_operands: Vec<Vec<super::ValueId>>,
    /// i32 dense array attribute
    pub operand_segments: crate::attrs::Attribute,
    pub successors: Vec<String>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.launder`
/// Launder operation
///
///   This operation represents a call to 'launder' in C++,
///   which acts as an optimization boundary that breaks type invariance.
///
/// Example:
/// ```
///   %0 = cir.alloca "" align(8) : !cir.ptr<!cir.ptr<!rec_S>>
///   %1 = cir.load align(8) %1 : !cir.ptr<!cir.ptr<!rec_S>>, !cir.ptr<!rec_S>
///   %2 = cir.launder(%1) : !cir.ptr<!rec_S>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Launder {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// CIR pointer type
    pub arg: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.minus`
/// Integer unary minus (negation)
///
/// The `cir.minus` operation negates the operand. The operand and result
/// must have the same type.
///
/// The optional `nsw` (no signed wrap) attribute indicates that the result
/// is poison if signed overflow occurs (e.g. negating the minimum signed
/// integer).
///
/// Example:
///
/// ```
/// %1 = cir.minus %0 : !s32i
/// %3 = cir.minus nsw %2 : !s32i
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Minus {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// integer or vector of integer type
    pub input: super::ValueId,
    /// unit property
    pub no_signed_wrap: bool,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.mul.overflow`
/// Integer multiplication with overflow checking
///
/// `cir.mul.overflow` performs multiplication with overflow checking on
/// integral operands. See `CIR_BinOpOverflow` for semantics.
///
/// Example:
///
/// ```
/// %result, %overflow = cir.mul.overflow %a, %b : !u32i -> !u32i
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MulOverflow {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    pub overflow: super::ValueId,
    pub overflow_ty: crate::types::Type,
    /// Integer type with arbitrary precision up to a fixed limit
    pub lhs: super::ValueId,
    /// Integer type with arbitrary precision up to a fixed limit
    pub rhs: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.not`
/// Bitwise NOT / logical NOT
///
/// The `cir.not` operation performs a bitwise NOT on integer types or a
/// logical NOT on boolean types. The operand and result must have the same
/// type.
///
/// Example:
///
/// ```
/// %1 = cir.not %0 : !s32i
/// %3 = cir.not %2 : !cir.bool
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Not {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// integer, boolean, or vector of bool or integer
    pub input: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.parity`
/// Get the parity of input
///
/// Compute the parity of the input. The parity of an integer is the number of
/// 1-bits in it modulo 2.
///
/// The input must be an unsigned integer.
///
/// Example:
///
/// ```
/// // %0 = 0x0110_1000
/// %0 = cir.const #cir.int<104> : !u32i
/// // %1 will be 1 since there are three 1-bits in %0
/// %1 = cir.parity %0 : !u32i
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Parity {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// unsigned integer type of widths 32/64
    pub input: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.popcount`
/// Get the number of 1-bits in input
///
/// Compute the number of 1-bits in the input.
///
/// The input must be an unsigned integer.
///
/// Example:
///
/// ```
/// // %0 = 0x0110_1000
/// %0 = cir.const #cir.int<104> : !u32i
/// // %1 will be 3 since there are 3 1-bits in %0
/// %1 = cir.popcount %0 : !u32i
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Popcount {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// unsigned integer type
    pub input: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.prefetch`
/// Prefetch operation
///
/// The `cir.prefetch` operation is a hint to the code generator to insert a
/// prefetch instruction if supported; otherwise, it is a noop. Prefetches
/// have no effect on the behavior of the program but can change its
/// performance characteristics.
///
/// ```
/// cir.prefetch(%0 : !cir.ptr<!void>) locality(1) write
/// ```
///
/// $locality is a temporal locality specifier ranging from (0) - no locality,
/// to (3) - extremely local, keep in cache. If $locality is not present, the
/// default value is 3.
///
/// $isWrite specifies whether the prefetch is for a 'read' or 'write'. If
/// $isWrite is not specified, it means that prefetch is prepared for 'read'.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prefetch {
    /// pointer to void type
    pub addr: super::ValueId,
    /// 32-bit signless integer attribute whose minimum value is 0 whose maximum value is 3
    pub locality: crate::attrs::Attribute,
    /// unit attribute
    pub is_write: bool,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.return_address`
/// The return address of the current function, or of one of its callers
///
/// Represents a call to builtin function ` __builtin_return_address` in CIR.
/// This builtin function returns the return address of the current function,
/// or of one of its callers.
///
/// The `level` argument is number of frames to scan up the call stack.
/// For instance, value of 0 yields the return address of the current function,
/// value of 1 yields the return address of the caller of the current function,
/// and so forth.
///
/// Examples:
///
/// ```
/// %p = return_address(%level) : !cir.ptr<!void>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReturnAddress {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// 32-bit unsigned integer
    pub level: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.signbit`
/// Checks the sign of a floating-point number
///
/// It returns whether the sign bit (i.e. the highest bit) of the input operand
/// is set.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Signbit {
    pub res: super::ValueId,
    pub res_ty: crate::types::Type,
    /// single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type
    pub input: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.stackrestore`
/// restores the state of the function stack
///
/// Restore the state of the function stack to the state it was
/// in when the corresponding cir.stacksave executed.
/// This is used during the lowering of variable length array allocas.
///
/// This operation corresponds to LLVM intrinsic `stackrestore`.
///
/// ```
/// %0 = cir.alloca "saved_stack" align(8) : !cir.ptr<!cir.ptr<!u8i>>
/// %1 = cir.stacksave : <!u8i>
/// cir.store %1, %0 : !cir.ptr<!u8i>, !cir.ptr<!cir.ptr<!u8i>>
/// %2 = cir.load %0 : !cir.ptr<!cir.ptr<!u8i>>, !cir.ptr<!u8i>
/// cir.stackrestore %2 : !cir.ptr<!u8i>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Stackrestore {
    /// CIR pointer type
    pub ptr: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.stacksave`
/// remembers the current state of the function stack
///
/// Saves current state of the function stack. Returns a pointer to an opaque object
/// that later can be passed into cir.stackrestore.
/// This is used during the lowering of variable length array allocas.
///
/// This operation corresponds to LLVM intrinsic `stacksave`.
///
/// ```
/// %0 = cir.stacksave : <!u8i>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Stacksave {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.sub.overflow`
/// Integer subtraction with overflow checking
///
/// `cir.sub.overflow` performs subtraction with overflow checking on integral
/// operands. See `CIR_BinOpOverflow` for semantics.
///
/// Example:
///
/// ```
/// %result, %overflow = cir.sub.overflow %a, %b : !u32i -> !u32i
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubOverflow {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    pub overflow: super::ValueId,
    pub overflow_ty: crate::types::Type,
    /// Integer type with arbitrary precision up to a fixed limit
    pub lhs: super::ValueId,
    /// Integer type with arbitrary precision up to a fixed limit
    pub rhs: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.try_call`
/// try_call operation
///
/// Similar to `cir.call` but requires two destination blocks,
/// one which is used if the call returns without throwing an
/// exception (the "normal" destination) and another which is used
/// if an exception is thrown (the "unwind" destination).
///
/// This operation is used only after the CFG flatterning pass.
///
/// Example:
///
/// ```
/// // Before CFG flattening
/// cir.try {
///   %call = cir.call @division(%a, %b) : () -> !s32i
///   cir.yield
/// } catch all {
///   cir.yield
/// }
///
/// // After CFG flattening
/// %call = cir.try_call @division(%a, %b) ^normalDest, ^unwindDest
///   : (f32, f32) -> f32
/// ^normalDest:
///   cir.br ^afterTryBlock
/// ^unwindDest:
///   %exception_ptr, %type_id = cir.eh.inflight_exception
///   cir.br ^catchHandlerBlock(%exception_ptr : !cir.ptr<!void>)
/// ^catchHandlerBlock:
///   ...
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TryCall {
    pub result: Option<super::ValueId>,
    pub result_ty: Option<crate::types::Type>,
    /// flat symbol reference attribute
    pub callee: Option<crate::attrs::Attribute>,
    /// variadic of CIR void type or CIR bool type or CIR array type or CIR vector type or Integer type with arbitrary precision up to a fixed limit or single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type or CIR pointer type or CIR function type or CIR struct/class type or CIR union type or CIR complex type or CIR type that is used for the vptr member of C++ objects or CIR type that represents a pointer-to-data-member in C++ or CIR type that represents C++ pointer-to-member-function type or CIR exception handling token type or CIR cleanup token type or CIR catch token type
    pub args: Vec<super::ValueId>,
    /// unit attribute
    pub nothrow: bool,
    /// Inline kind attribute
    pub inline_kind: Option<crate::enums::InlineKind>,
    /// unit attribute
    pub musttail: bool,
    /// allowed side effects of a function
    pub side_effect: crate::attrs::Attribute,
    /// Array of dictionary attributes
    pub arg_attrs: Option<crate::attrs::Attribute>,
    /// Array of dictionary attributes
    pub res_attrs: Option<crate::attrs::Attribute>,
    pub successors: Vec<String>,
    pub loc: Option<crate::ast::SourceLocation>,
}