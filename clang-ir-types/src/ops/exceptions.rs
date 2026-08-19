//! Exception and cleanup operations.

/// `cir.alloc.exception`
/// Allocates an exception according to Itanium ABI
///
/// Implements a slightly higher level __cxa_allocate_exception:
///
/// `void *__cxa_allocate_exception(size_t thrown_size);`
///
/// If the operation fails, the program terminates rather than throw.
///
/// Example:
///
/// ```
/// // if (b == 0) {
/// //   ...
/// //   throw "...";
/// cir.if %10 {
///     %11 = cir.alloc_exception 8 -> !cir.ptr<!void>
///     ... // store exception content into %11
///     cir.throw %11 : !cir.ptr<!cir.ptr<!u8i>>, ...
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AllocException {
    pub addr: super::ValueId,
    pub addr_ty: crate::types::Type,
    /// 64-bit signless integer attribute
    pub size: crate::attrs::Attribute,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.begin_catch`
/// Begin a catch handler
///
/// `cir.begin_catch` marks the beginning of a catch handler. It takes a
/// `!cir.eh_token` representing the inflight exception and returns a
/// `!cir.catch_token` along with a pointer to the exception object.
///
/// The catch token must be passed to the corresponding `cir.end_catch`
/// operation. The exception pointer points to the caught exception object
/// and can be used to access the exception value.
///
/// For `catch(...)` (catch all), the exception pointer type is
/// `!cir.ptr<!void>`.
///
/// In the high-level CIR representation, this operation appears as the
/// first operation in a catch handler region of a `cir.try` operation,
/// taking the region's `!cir.eh_token` argument. In the flattened CIR
/// representation, it appears in a catch block, taking the block's
/// `!cir.eh_token` argument.
///
/// Example:
///
/// ```
/// // High-level form (inside cir.try catch handler region):
/// } catch [type #cir.global_view<@_ZTIi> : !cir.ptr<!u8i>] (%eh_token : !cir.eh_token) {
///   %catch_token, %exn_ptr = cir.begin_catch %eh_token
///     : !cir.eh_token -> (!cir.catch_token, !cir.ptr<!s32i>)
///   cir.cleanup.scope {
///     // Handle exception...
///     cir.yield
///   } cleanup eh {
///     cir.end_catch %catch_token : !cir.catch_token
///     cir.yield
///   }
///   cir.yield
/// }
///
/// // Flattened form (inside a catch block):
/// ^catch_int(%eh_token : !cir.eh_token):
///   %catch_token, %exn_ptr = cir.begin_catch %eh_token
///     : !cir.eh_token -> (!cir.catch_token, !cir.ptr<!s32i>)
///   // Handle exception...
///   cir.end_catch %catch_token : !cir.catch_token
///   cir.br ^continue
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BeginCatch {
    pub catch_token: super::ValueId,
    pub catch_token_ty: crate::types::Type,
    pub exn_ptr: super::ValueId,
    pub exn_ptr_ty: crate::types::Type,
    /// CIR exception handling token type
    pub eh_token: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.begin_cleanup`
/// Begin a cleanup block during exception unwinding
///
/// `cir.begin_cleanup` marks the beginning of a cleanup handler during
/// exception unwinding. It takes a `!cir.eh_token` and returns a
/// `!cir.cleanup_token` that must be passed to the corresponding
/// `cir.end_cleanup` operation.
///
/// The cleanup code between `cir.begin_cleanup` and `cir.end_cleanup` will be
/// executed during exception unwinding before control is transferred to
/// the exception dispatcher.
///
/// Example:
///
/// ```
/// ^cleanup(%eh_token : !cir.eh_token):
///   %cleanup_token = cir.begin_cleanup %eh_token : !cir.eh_token
///                                                -> !cir.cleanup_token
///   cir.call @destructor() : () -> ()
///   cir.end_cleanup %cleanup_token : !cir.cleanup_token
///   cir.br ^dispatch(%eh_token : !cir.eh_token)
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BeginCleanup {
    pub cleanup_token: super::ValueId,
    pub cleanup_token_ty: crate::types::Type,
    /// CIR exception handling token type
    pub eh_token: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.catch_param`
/// Represents the catch clause formal parameter
///
/// The `cir.catch_param` is used to retrieve the exception object inside
/// the handler regions of `cir.try`.
///
/// This operation is used only before the CFG flatterning pass.
///
/// Example:
///
/// ```
/// %exception = cir.catch_param : !cir.ptr<!void>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CatchParam {
    pub param: Option<super::ValueId>,
    pub param_ty: Option<crate::types::Type>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.construct_catch_param`
/// Construct a catch parameter from the in-flight exception
///
/// `cir.construct_catch_param` abstractly represents the target-specific work
/// that must be performed before `cir.begin_catch` to bind the in-flight
/// exception object to the local alloca used for the catch parameter.
///
/// For example: for non-pointer, non-reference catch parameters whose type has
/// a non-trivial copy constructor, the Itanium C++ ABI requires
/// calling `__cxa_get_exception_ptr` to obtain the adjusted exception pointer
/// and then invoking the catch parameter's copy constructor to create a local
/// copy of the object before `__cxa_begin_catch` is invoked.
///
/// This operation takes a `!cir.eh_token` that represents the in-flight
/// exception and the alloca value that is used for the local copy of the
/// exception object. The `copy_fn` attribute is a flat symbol reference to a
/// `cir.func` thunk that copies the exception object to a local alloca value.
///
/// This operation is replaced with a target-specific representation during
/// the EHABI lowering pass. For some targets, such as the Microsoft ABI,
/// this operation is a no-op and is simply erased during lowering.
///
/// Example:
///
/// ```
/// cir.construct_catch_param non_trivial_copy %eh_token to %param_addr
///   using @__clang_cir_catch_init_T : !cir.ptr<!rec_T>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConstructCatchParam {
    /// CIR exception handling token type
    pub eh_token: super::ValueId,
    /// CIR pointer type
    pub param_addr: super::ValueId,
    /// allowed 32-bit signless integer cases: 0, 1, 2, 3, 4, 5
    pub kind: crate::enums::InitCatchKind,
    /// flat symbol reference attribute
    pub copy_fn: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.eh.dispatch`
/// Dispatch to exception handlers based on exception type
///
/// `cir.eh.dispatch` is a terminator operation that dispatches control flow
/// based on the type of the in-flight exception. It takes an `!cir.eh_token`
/// as input and branches to an eh handler block based on the exception type.
///
/// The operation contains a list of handlers specified as type-block pairs,
/// plus either a `catch_all` handler or an `unwind` handler, which continues
/// unwinding if no catch handler matches. Exactly one of `catch_all` or
/// `unwind` must be present.
///
/// When the type of the in-flight exception matches a type handler, control
/// is transfered to the corresponding block with the eh_token as an argument.
/// The `catch_all` handler, if present, catches any exception not matched by
/// another type handler. The `unwind` handler is used when no handler is
/// matched.
///
/// Example:
///
/// ```
/// cir.eh.dispatch %eh_token : !cir.eh_token [
///   catch (#cir.global_view<@_ZTIi> : !cir.ptr<!u8i>) : ^catch_int,
///   catch (#cir.global_view<@_ZTIPKc> : !cir.ptr<!u8i>) : ^catch_str,
///   catch_all : ^catch_all
/// ]
///
/// cir.eh.dispatch %eh_token : !cir.eh_token [
///   catch (#cir.global_view<type @_ZTIi> : !cir.ptr<!u8i>) : ^catch_int,
///   unwind : ^continue_unwind
/// ]
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EhDispatch {
    /// CIR exception handling token type
    pub eh_token: super::ValueId,
    /// array attribute
    pub catch_types: Option<crate::attrs::Attribute>,
    /// unit attribute
    pub default_is_catch_all: bool,
    pub successors: Vec<String>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.eh.inflight_exception`
/// Materialize the catch clause formal parameter
///
/// `cir.eh.inflight_exception` returns two values:
///   - `exception_ptr`: The exception pointer for the inflight exception
///   - `type_id`: the type info index for the exception type
/// This operation is expected to be the first operation in the unwind
/// destination basic blocks of a `cir.try_call` operation.
///
/// The `cleanup` attribute indicates that clean up code must be run before the
/// values produced by this operation are used to dispatch the exception. This
/// cleanup code must be executed even if the exception is not caught.
/// This helps CIR to pass down more accurate information for LLVM lowering
/// to landingpads.
///
/// The `catch_all` attribute indicates that a catch-all handler exists for
/// the exception being dispatched. When lowered to LLVM IR, this results in
/// a `catch ptr null` clause on the landing pad. When `catch_all` is present
/// alongside typed catches, the landing pad will contain both the typed catch
/// clauses and a trailing `catch ptr null`.
///
/// Example:
///
/// ```
/// %exception_ptr, %type_id = cir.eh.inflight_exception
/// %exception_ptr, %type_id = cir.eh.inflight_exception [@_ZTIi, @_ZTIPKc]
/// %exception_ptr, %type_id = cir.eh.inflight_exception cleanup
/// %exception_ptr, %type_id = cir.eh.inflight_exception catch_all [@_ZTIi]
/// ``
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EhInflightException {
    pub exception_ptr: super::ValueId,
    pub exception_ptr_ty: crate::types::Type,
    pub type_id: super::ValueId,
    pub type_id_ty: crate::types::Type,
    /// unit attribute
    pub cleanup: bool,
    /// unit attribute
    pub catch_all: bool,
    /// flat symbol ref array attribute
    pub catch_type_list: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.eh.initiate`
/// Initiate exception handling in flattened CIR
///
/// `cir.eh.initiate` is the first operation in the unwind destination of a
/// `cir.try_call` operation after CFG flattening. It returns an opaque
/// `!cir.eh_token` that represents the in-flight exception.
///
/// The `cleanup` attribute indicates that cleanup code must be executed before
/// the exception is dispatched to any handlers. When present, the operation
/// will be followed by cleanup code before branching to a `cir.eh.dispatch`
/// operation.
///
/// The returned token is passed to `cir.begin_cleanup`, `cir.begin_catch`,
/// or `cir.eh.dispatch` operations.
///
/// Example:
///
/// ```
/// ^unwind:
///   %eh_token = cir.eh.initiate : !cir.eh_token
///   cir.br ^dispatch(%eh_token : !cir.eh_token)
///
/// ^unwind_with_cleanup:
///   %eh_token = cir.eh.initiate cleanup : !cir.eh_token
///   cir.br ^cleanup(%eh_token : !cir.eh_token)
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EhInitiate {
    pub eh_token: super::ValueId,
    pub eh_token_ty: crate::types::Type,
    /// unit attribute
    pub cleanup: bool,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.eh.longjmp`
/// CIR longjmp operation
///
/// Restore the environment (e.g., stack pointer, instruction pointer,
/// signal mask, and other registers) at the time of setjmp() call, by using
/// the information saved in `env` by setjmp().
///
/// Examples:
/// ```
///   cir.eh.longjmp %arg0 : !cir.ptr<!cir.void>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EhLongjmp {
    /// CIR pointer type
    pub env: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.eh.setjmp`
/// CIR setjmp operation
///
/// Saves call-site information (e.g., stack pointer, instruction pointer,
/// signal mask, and other registers) in memory at `env` for use by longjmp().
/// In this case, setjmp() returns 0. Following a successful longjmp(),
/// execution proceeds from cir.eh.setjmp with the operation yielding a
/// non-zero value.
///
/// Examples:
/// ```
///   %0 = cir.eh.setjmp %arg0 : (!cir.ptr<!cir.void>) -> !s32i
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EhSetjmp {
    pub res: super::ValueId,
    pub res_ty: crate::types::Type,
    /// CIR pointer type
    pub env: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.eh.terminate`
/// Terminate due to exception thrown during cleanup
///
/// `cir.eh.terminate` terminates program execution when an exception is thrown
/// while executing cleanup code during exception unwinding. The C++ standard
/// requires that `std::terminate()` be called in this scenario.
///
/// This operation takes an `!cir.eh_token` from a `cir.eh.initiate` operation
/// and acts as a terminator. It is produced during CFG flattening when throwing
/// calls are found in EH cleanup regions.
///
/// During EH ABI lowering, this is replaced with target-specific termination
/// code. For the Itanium ABI, the `cir.eh.initiate` is lowered to
/// `cir.eh.inflight_exception` (producing an exception pointer), and the
/// `cir.eh.terminate` becomes a call to `__clang_call_terminate` with that
/// pointer, followed by an unreachable operation.
///
/// Example:
///
/// ```
/// ^terminate_unwind:
///   %eh_token = cir.eh.initiate : !cir.eh_token
///   cir.eh.terminate %eh_token : !cir.eh_token
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EhTerminate {
    /// CIR exception handling token type
    pub eh_token: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.eh.typeid`
/// Compute exception type id from its global type symbol
///
/// Returns the exception type id for a given global symbol representing
/// a type.
///
/// Example:
/// ```
/// %type_id = cir.eh.typeid @_ZTIi
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EhTypeid {
    pub type_id: super::ValueId,
    pub type_id_ty: crate::types::Type,
    /// flat symbol reference attribute
    pub type_sym: crate::attrs::Attribute,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.end_catch`
/// End a catch handler
///
/// `cir.end_catch` marks the end of a catch handler. It takes the
/// `!cir.catch_token` returned by the corresponding `cir.begin_catch`
/// operation.
///
/// In the high-level CIR representation, this operation appears inside
/// the cleanup region of the `cir.cleanup.scope` that follows
/// `cir.begin_catch` in a catch handler region. In the flattened CIR
/// representation, it appears at the end of a path that exits the catch
/// handler.
///
/// Example:
///
/// ```
/// // High-level form (inside cir.try catch handler region):
/// } catch [type #cir.global_view<@_ZTIi> : !cir.ptr<!u8i>] (%eh_token : !cir.eh_token) {
///   %catch_token, %exn_ptr = cir.begin_catch %eh_token
///     : !cir.eh_token -> (!cir.catch_token, !cir.ptr<!s32i>)
///   cir.cleanup.scope {
///     // Handle exception...
///     cir.yield
///   } cleanup eh {
///     cir.end_catch %catch_token : !cir.catch_token
///     cir.yield
///   }
/// }
///
/// // Flattened form:
/// ^catch_int(%eh_token : !cir.eh_token):
///   %catch_token, %exn_ptr = cir.begin_catch %eh_token
///     : !cir.eh_token -> (!cir.catch_token, !cir.ptr<!s32i>)
///   // Handle exception...
///   cir.end_catch %catch_token : !cir.catch_token
///   cir.br ^continue
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EndCatch {
    /// CIR catch token type
    pub catch_token: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.end_cleanup`
/// End a cleanup block during exception unwinding
///
/// `cir.end_cleanup` marks the end of a cleanup block during exception
/// unwinding. It takes the `!cir.cleanup_token` returned by the corresponding
/// `cir.begin_cleanup` operation.
///
/// After the cleanup is complete, control typically transfers to either a
/// catch dispatch block or continues unwinding via `cir.resume`.
///
/// Example:
///
/// ```
/// ^cleanup(%eh_token : !cir.eh_token):
///   %cleanup_token = cir.begin_cleanup %eh_token : !cir.eh_token
///                                                -> !cir.cleanup_token
///   cir.call @destructor() : () -> ()
///   cir.end_cleanup %cleanup_token : !cir.cleanup_token
///   cir.br ^dispatch(%eh_token : !cir.eh_token)
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EndCleanup {
    /// CIR cleanup token type
    pub cleanup_token: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.init_catch_param`
/// Initialize a catch parameter from the exception pointer
///
/// `cir.init_catch_param` represents copying or otherwise materializing the
/// caught exception object into the local catch parameter variable. It takes
/// the exception pointer returned by `cir.begin_catch` and the address of
/// the alloca created for the catch parameter, and has no result.
///
/// This operation is target-independent. It is replaced during EHABI
/// lowering with the appropriate target/ABI-specific sequence (for example,
/// the Itanium C++ ABI may emit a load and store for scalar/pointer catch
/// types, an aggregate copy for record types, or a call to a copy
/// constructor when one is required).
///
/// Example:
///
/// ```
/// %catch_token, %exn_ptr = cir.begin_catch %eh_token
///   : !cir.eh_token -> (!cir.catch_token, !cir.ptr<!void>)
/// %param_addr = cir.alloca "e" align(4) init : !cir.ptr<!s32i>
/// cir.init_catch_param %exn_ptr to %param_addr
///   : !cir.ptr<!void>, !cir.ptr<!s32i>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitCatchParam {
    /// CIR pointer type
    pub exn_ptr: super::ValueId,
    /// CIR pointer type
    pub param_addr: super::ValueId,
    /// allowed 32-bit signless integer cases: 0, 1, 2, 3, 4, 5
    pub kind: crate::enums::InitCatchKind,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.resume`
/// Resumes execution after not catching exceptions
///
/// The `cir.resume` operation handles an uncaught exception scenario.
///
/// Before CFG flattening, this operation is used as the terminator of a
/// `CatchUnwind` region of `cir.try`, where it receives an `!cir.eh_token`
/// argument representing the in-flight exception.
///
/// During CFG flattening, this operation may temporarily appear inside any
/// structured CIR operation (scope, loop, switch, etc.) when an inner cleanup
/// scope is flattened before the enclosing structured op. When the enclosing
/// op is subsequently flattened, the resume will end up in a `cir.try`
/// operation or at function level.
///
/// After CFG flattening, this operation appears at the function level (inside
/// `cir.func`) to indicate that the exception should be re-thrown to the
/// caller after cleanup code has been executed.
///
/// Examples:
/// ```
/// // Before CFG flattening (in try unwind region)
/// cir.try {
///   cir.yield
/// } unwind (%eh_token : !cir.eh_token) {
///   cir.resume %eh_token : !cir.eh_token
/// }
///
/// // After CFG flattening (at function level, after cleanup)
/// ^eh_cleanup(%eh_token : !cir.eh_token):
///   %ct = cir.begin_cleanup %eh_token : !cir.eh_token -> !cir.cleanup_token
///   cir.call @destructor() : () -> ()
///   cir.end_cleanup %ct : !cir.cleanup_token
///   cir.resume %eh_token : !cir.eh_token
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Resume {
    /// CIR exception handling token type
    pub eh_token: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.resume.flat`
/// A flattened version of `cir.resume`
///
/// The `cir.resume.flat` operation is a region-less and simplified
/// version of the `cir.resume`.
///
/// Its representation is closer to LLVM IR dialect
/// than the C/C++ language feature.
///
/// This operation is used only after the CFG flatterning pass.
///
/// Examples:
/// ```
/// cir.resume.flat %exception_ptr, %type_id
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResumeFlat {
    /// pointer to void type
    pub exception_ptr: super::ValueId,
    /// 32-bit unsigned integer
    pub type_id: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.throw`
/// (Re)Throws an exception
///
/// This operation is equivalent to either __cxa_throw or __cxa_rethrow,
/// depending on the arguments.
///
/// The absense of arguments for `cir.throw` means it rethrows.
///
/// For the no-rethrow version, it must have at least two operands, the RTTI
/// information, a pointer to the exception object (likely allocated via
/// `cir.alloc_exception`) and finally an optional dtor, which might run as
/// part of this operation.
///
/// Example:
///
/// ```
/// // re-throw;
/// cir.throw
///
/// // if (b == 0)
/// //   throw "Division by zero condition!";
///
/// // Type info for char const*
/// cir.global "private" constant external @_ZTIPKc : !cir.ptr<!u8i>
/// cir.if %cond {
///   %exception_addr = cir.alloc_exception 8 -> !cir.ptr<!void>
///   ...
///   // Store string addr for "Division by zero condition!"
///   cir.store %string_addr, %exception_addr : !cir.ptr<!s8i>,
///     !cir.ptr<!cir.ptr<!s8i>>
///   cir.throw %exception_addr : !cir.ptr<!cir.ptr<!u8i>>,
///     @_ZTIPKc
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Throw {
    /// CIR pointer type
    pub exception_ptr: Option<super::ValueId>,
    /// flat symbol reference attribute
    pub type_info: Option<crate::attrs::Attribute>,
    /// flat symbol reference attribute
    pub dtor: Option<crate::attrs::Attribute>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.token.none`
/// Produces an empty token value.
///
/// Produces a `none` token value, mirroring LLVM IR's `none` token
/// literal. Lowers to `llvm::ConstantTokenNone`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TokenNone {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.try`
/// C++ try block
///
/// Holds the lexical scope of `try {}`. Note that resources used on catch
/// clauses are usually allocated in the same parent as `cir.try`.
///
/// `cleanup`: indicates that there are cleanups that must be performed
/// when exiting the try region via exception, even if the exception is not
/// caught.
///
/// Each catch handler region and unwind region receives a `!cir.eh_token`
/// argument representing the inflight exception. The first operation in a
/// catch handler region must be a `cir.begin_catch` operation. This must
/// be followed by a `cir.cleanup.scope` operation, with the `cir.end_catch`
/// operation in its cleanup region. The catch handling code will be emitted
/// into the body of the cleanup scope. This ensures that all paths out of the
/// catch handler will execute the end_catch operation.
///
/// Example:
///
/// ```
/// cir.try {
///   cir.call exception @function() : () -> ()
///   cir.yield
/// } catch [type #cir.global_view<@_ZTIPf> : !cir.ptr<!u8i>] (%eh_token : !cir.eh_token) {
///   %catch_token, %exn_ptr = cir.begin_catch %eh_token
///     : !cir.eh_token -> (!cir.catch_token, !cir.ptr<!cir.float>)
///   cir.cleanup.scope {
///     ...
///     cir.yield
///   } cleanup eh {
///     cir.end_catch %catch_token : !cir.catch_token
///     cir.yield
///   }
///   cir.yield
/// } unwind (%eh_token : !cir.eh_token) {
///   cir.resume %eh_token : !cir.eh_token
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Try {
    /// unit attribute
    pub cleanup: bool,
    /// catch all or unwind or global view array attribute
    pub handler_types: crate::attrs::Attribute,
    pub try_region: super::Region,
    pub handler_regions: Vec<super::Region>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.try_throw`
/// throw an exception with an unwind destination
///
/// Similar to `cir.throw` but acts as a terminator with two destination
/// blocks: a `normalDest` that should contain a `cir.unreachable`
/// operation (since a throw never returns) and an `unwindDest` that
/// receives control when the throw needs to unwind through an enclosing
/// cleanup or catch handler. This is the EH counterpart of `cir.throw`,
/// analogous to how `cir.try_call` is the EH counterpart of `cir.call`.
///
/// Like `cir.throw`, the absence of operands means rethrow. With operands,
/// it carries the same exception pointer, type info, and optional
/// destructor as `cir.throw`.
///
/// This operation is produced by the FlattenCFG pass for `cir.throw`
/// operations that appear inside a cleanup scope or try region. It is
/// later lowered by the EHABI lowering pass to a `cir.try_call` of
/// `__cxa_throw` (or `__cxa_rethrow`).
///
/// Example:
///
/// ```
/// cir.try_throw %exception_addr : !cir.ptr<!s32i>, @_ZTIi
///     ^normalDest, ^unwindDest
/// ^normalDest:
///   cir.unreachable
/// ^unwindDest:
///   ...
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TryThrow {
    /// CIR pointer type
    pub exception_ptr: Option<super::ValueId>,
    /// flat symbol reference attribute
    pub type_info: Option<crate::attrs::Attribute>,
    /// flat symbol reference attribute
    pub dtor: Option<crate::attrs::Attribute>,
    pub successors: Vec<String>,
    pub loc: Option<crate::ast::SourceLocation>,
}