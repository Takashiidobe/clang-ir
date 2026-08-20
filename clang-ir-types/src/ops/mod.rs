//! Typed CIR operations, split by operation category.

#![allow(non_camel_case_types)]
pub type ValueId = String;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Region {
    pub blocks: Vec<Block>,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Block {
    pub label: Option<String>,
    pub args: Vec<(ValueId, crate::types::Type)>,
    pub ops: Vec<Op>,
}
pub mod arithmetic;
pub mod arrays;
pub mod atomics;
pub mod calls;
pub mod complex;
pub mod control_flow;
pub mod exceptions;
pub mod globals;
pub mod memory;
pub mod misc;
pub mod stdlib;
pub mod varargs;
pub mod vectors;
pub mod vtables;
pub use arithmetic::*;
pub use arrays::*;
pub use atomics::*;
pub use calls::*;
pub use complex::*;
pub use control_flow::*;
pub use exceptions::*;
pub use globals::*;
pub use memory::*;
pub use misc::*;
pub use stdlib::*;
pub use varargs::*;
pub use vectors::*;
pub use vtables::*;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Op {
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
    Abs(arithmetic::Abs),
    /// `cir.acos`
    /// Computes the arcus cosine of the specified value
    ///
    /// `cir.acos`computes the arcus cosine of a given value and
    /// returns a result of the same type.
    ///
    /// Floating-point exceptions are ignored, and it does not set `errno`.
    Acos(arithmetic::Acos),
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
    Add(arithmetic::Add),
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
    AddOverflow(misc::AddOverflow),
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
    AddressOfReturnAddress(misc::AddressOfReturnAddress),
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
    AllocException(exceptions::AllocException),
    /// `cir.alloca`
    /// Defines a scope-local variable
    ///
    /// The `cir.alloca` operation defines a scope-local variable.
    ///
    /// The presence of the `const` attribute indicates that the local variable is
    /// declared with C/C++ `const` keyword.
    ///
    /// The `dynAllocSize` specifies the size to dynamically allocate on the stack
    /// and ignores the allocation size based on the original type. This is useful
    /// when handling VLAs or the `alloca` builtin and is omitted when declaring
    /// regular local variables.
    ///
    /// The `cleanup_dest_slot` attribute indicates that this was a temporary
    /// alloca generated by the compiler to handle cleanup exit dispatching.
    ///
    /// The result type is a pointer to the input's type.
    ///
    /// Example:
    ///
    /// ```
    /// // int count;
    /// %0 = cir.alloca "count" align(4) : !cir.ptr<i32>
    ///
    /// // int *ptr;
    /// %1 = cir.alloca "ptr" align(8) : !cir.ptr<!cir.ptr<i32>>
    /// ...
    /// ```
    Alloca(memory::Alloca),
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
    And(arithmetic::And),
    /// `cir.array.ctor`
    /// Initialize array elements with C++ constructors
    ///
    /// Initialize each array element using the same C++ constructor. This
    /// operation has a `body` region and an optional `partial_dtor` region.
    /// Both regions have a single block whose argument is a pointer to the
    /// current array element.
    ///
    /// The `body` region contains the constructor call for one element.
    ///
    /// The `partial_dtor` region, when non-empty, contains the destructor call
    /// for one element. During lowering, it is used to build a cleanup that
    /// destroys already-constructed elements if a constructor throws. When the
    /// element type has a trivial destructor or exceptions are disabled, the
    /// `partial_dtor` region is left empty.
    ///
    /// When `num_elements` is absent, `addr` must be a pointer to a fixed-size
    /// CIR array type and the element count is derived from that array type.
    ///
    /// When `num_elements` is present, `addr` is a pointer to the first element
    /// and `num_elements` provides the runtime element count (for example `new
    /// T[n]`).
    ///
    /// Examples:
    ///
    /// ```
    /// // Fixed size without partial destructor:
    /// cir.array.ctor(%0 : !cir.ptr<!cir.array<!rec_S x 42>>) {
    ///   ^bb0(%arg0: !cir.ptr<!rec_S>):
    ///     cir.call @some_ctor(%arg0) : (!cir.ptr<!rec_S>) -> ()
    /// }
    ///
    /// // Variable size without partial destructor:
    /// cir.array.ctor(%ptr, %n : !cir.ptr<!rec_S>, !u64i) {
    ///   ^bb0(%arg0: !cir.ptr<!rec_S>):
    ///     cir.call @some_ctor(%arg0) : (!cir.ptr<!rec_S>) -> ()
    /// }
    ///
    /// // Fixed size with partial destructor:
    /// cir.array.ctor(%0 : !cir.ptr<!cir.array<!rec_S x 42>>) {
    ///   ^bb0(%arg0: !cir.ptr<!rec_S>):
    ///     cir.call @some_ctor(%arg0) : (!cir.ptr<!rec_S>) -> ()
    /// } partial_dtor {
    ///   ^bb0(%arg0: !cir.ptr<!rec_S>):
    ///     cir.call @some_dtor(%arg0) : (!cir.ptr<!rec_S>) -> ()
    /// }
    /// ```
    ArrayCtor(arrays::ArrayCtor),
    /// `cir.array.dtor`
    /// Destroy array elements with C++ destructors
    ///
    /// Destroy each array element using the same C++ destructor. This operation
    /// has one region with one block whose argument is a pointer to the current
    /// array element.
    ///
    /// When `num_elements` is absent, `addr` must be a pointer to a fixed-size
    /// CIR array type and the element count is derived from that array type.
    ///
    /// When `num_elements` is present, `addr` is a pointer to the first element
    /// and `num_elements` provides the runtime element count (e.g. from an array
    /// cookie for `delete[]`).
    ///
    /// When `dtor_may_throw` is present, the element destructor call may throw
    /// an exception.
    ///
    /// Elements are destroyed in reverse order.
    ///
    /// Examples:
    ///
    /// ```
    /// // Fixed-size (stack array, global):
    /// cir.array.dtor %0 : !cir.ptr<!cir.array<!rec_S x 42>> {
    ///   ^bb0(%arg0: !cir.ptr<!rec_S>):
    ///     cir.call @_ZN1SD1Ev(%arg0) : (!cir.ptr<!rec_S>) -> ()
    /// }
    ///
    /// // Dynamic count (delete[] with destructor):
    /// cir.array.dtor %ptr, %n : !cir.ptr<!rec_S>, !u64i {
    ///   ^bb0(%arg0: !cir.ptr<!rec_S>):
    ///     cir.call @_ZN1SD1Ev(%arg0) : (!cir.ptr<!rec_S>) -> ()
    /// }
    ///
    /// // Dynamic count (delete[] with throwing destructor):
    /// cir.array.dtor %ptr, %n : !cir.ptr<!rec_S>, !u64i dtor_may_throw {
    ///   ^bb0(%arg0: !cir.ptr<!rec_S>):
    ///     cir.call @_ZN1SD1Ev(%arg0) : (!cir.ptr<!rec_S>) -> ()
    /// }
    /// ```
    ArrayDtor(arrays::ArrayDtor),
    /// `cir.asin`
    /// Computes the arcus sine of the specified value
    ///
    /// `cir.asin`computes the arcus sine of a given value and
    /// returns a result of the same type.
    ///
    /// Floating-point exceptions are ignored, and it does not set `errno`.
    Asin(arithmetic::Asin),
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
    Asm(misc::Asm),
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
    Assume(misc::Assume),
    /// `cir.atan`
    /// Computes the floating-point arcus tangent value
    ///
    /// `cir.atan` computes the arcus tangent of a floating-point operand
    /// and returns a result of the same type.
    ///
    /// Floating-point exceptions are ignored, and it does not set `errno`.
    Atan(arithmetic::Atan),
    /// `cir.atan2`
    /// Computes the arc tangent of y/x
    ///
    /// `cir.atan2` computes the arc tangent of the first operand divided by the
    /// second operand, using the signs of both to determine the quadrant.
    Atan2(arithmetic::Atan2),
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
    AtomicClear(atomics::AtomicClear),
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
    AtomicCmpxchg(atomics::AtomicCmpxchg),
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
    AtomicFence(atomics::AtomicFence),
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
    AtomicFetch(atomics::AtomicFetch),
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
    AtomicTestAndSet(atomics::AtomicTestAndSet),
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
    AtomicXchg(atomics::AtomicXchg),
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
    Await(misc::Await),
    /// `cir.base_class_addr`
    /// Get the base class address for a class/struct
    ///
    /// The `cir.base_class_addr` operaration gets the address of a particular
    /// non-virtual base class given a derived class pointer. The offset in bytes
    /// of the base class must be passed in, since it is easier for the front end
    /// to calculate that than the MLIR passes. The operation contains a flag for
    /// whether or not the operand may be nullptr. That depends on the context and
    /// cannot be known by the operation, and that information affects how the
    /// operation is lowered.
    ///
    /// The validity of the relationship of derived and base cannot yet be verified.
    /// If the target class is not a valid base class for the object, the behavior
    /// is undefined.
    ///
    /// Example:
    /// ```c++
    /// struct Base { };
    /// struct Derived : Base { };
    /// Derived d;
    /// Base& b = d;
    /// ```
    /// will generate
    /// ```
    /// %3 = cir.base_class_addr %1 : !cir.ptr<!rec_Derived> nonnull [0] -> !cir.ptr<!rec_Base>
    /// ```
    BaseClassAddr(memory::BaseClassAddr),
    /// `cir.base_data_member`
    /// Cast a derived class data member pointer to a base class data member pointer
    ///
    /// The `cir.base_data_member` operation casts a data member pointer of type
    /// `T Derived::*` to a data member pointer of type `T Base::*`, where `Base`
    /// is an accessible non-ambiguous non-virtual base class of `Derived`.
    ///
    /// The `offset` parameter gives the offset in bytes of the `Base` base class
    /// subobject within a `Derived` object.
    BaseDataMember(memory::BaseDataMember),
    /// `cir.base_method`
    ///
    /// Cast a derived class pointer-to-member-function to a base class
    /// pointer-to-member-function
    ///
    ///
    /// The `cir.base_method` operation casts a pointer-to-member-function of type
    /// `Ret (Derived::*)(Args)` to a pointer-to-member-function of type
    /// `Ret (Base::*)(Args)`, where `Base` is a non-virtual base class of
    /// `Derived`.
    ///
    /// The `offset` parameter gives the offset in bytes of the `Base` base class
    /// subobject within a `Derived` object.
    ///
    /// Example:
    ///
    /// ```
    /// %1 = cir.base_method %0 [16] : !cir.method<!cir.func<(!s32i)> in !rec_Derived> -> !cir.method<!cir.func<(!s32i)> in !rec_Base>
    /// ```
    BaseMethod(memory::BaseMethod),
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
    BeginCatch(exceptions::BeginCatch),
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
    BeginCleanup(exceptions::BeginCleanup),
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
    Bitreverse(misc::Bitreverse),
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
    BlockAddress(misc::BlockAddress),
    /// `cir.br`
    /// Unconditional branch
    ///
    /// The `cir.br` branches unconditionally to a block. Used to represent C/C++
    /// goto's and general block branching.
    ///
    /// Note that for source level `goto`'s crossing scope boundaries, those are
    /// usually represented with the "symbolic" `cir.goto` operation.
    ///
    /// Example:
    ///
    /// ```
    ///   ...
    ///     cir.br ^bb3
    ///   ^bb3:
    ///     cir.return
    /// ```
    Br(control_flow::Br),
    /// `cir.brcond`
    /// Conditional branch
    ///
    /// The `cir.brcond %cond, ^bb0, ^bb1` branches to 'bb0' block in case
    /// %cond (which must be a !cir.bool type) evaluates to true, otherwise
    /// it branches to 'bb1'.
    ///
    /// Example:
    ///
    /// ```
    ///   ...
    ///     cir.brcond %a, ^bb3, ^bb4
    ///   ^bb3:
    ///     cir.return
    ///   ^bb4:
    ///     cir.yield
    /// ```
    Brcond(control_flow::Brcond),
    /// `cir.break`
    /// C/C++ `break` statement equivalent
    ///
    /// The `cir.break` operation is used to cease the execution of the current loop
    /// or switch operation and transfer control to the parent operation. It is only
    /// allowed within a breakable operations (loops and switches).
    Break(control_flow::Break),
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
    BuiltinIntCast(misc::BuiltinIntCast),
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
    ByteSwap(arithmetic::ByteSwap),
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
    /// Example:
    ///
    /// ```
    /// %0 = cir.call @foo()
    /// ```
    Call(calls::Call),
    /// `cir.call_llvm_intrinsic`
    /// Call to llvm intrinsic functions that is not defined in CIR
    ///
    /// `cir.call_llvm_intrinsic` operation represents a call-like expression which has
    /// return type and arguments that maps directly to a llvm intrinsic.
    /// It only records intrinsic `intrinsic_name`.
    CallLlvmIntrinsic(calls::CallLlvmIntrinsic),
    /// `cir.case`
    /// Case operation
    ///
    /// The `cir.case` operation represents a case within a C/C++ switch.
    /// The `cir.case` operation must be in a `cir.switch` operation directly
    /// or indirectly.
    ///
    /// The `cir.case` have 4 kinds:
    /// - `equal, <constant>`: equality of the second case operand against the
    /// condition.
    /// - `anyof, [constant-list]`: equals to any of the values in a subsequent
    /// following list.
    /// - `range, [lower-bound, upper-bound]`: the condition is within the closed
    ///                                        interval.
    /// - `default`: any other value.
    ///
    /// Each case region must be explicitly terminated.
    Case(control_flow::Case),
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
    Cast(misc::Cast),
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
    CatchParam(exceptions::CatchParam),
    /// `cir.ceil`
    /// Computes the ceiling of the specified value
    ///
    /// `cir.ceil` computes the ceiling of a given value and returns a result
    /// of the same type.
    ///
    /// Floating-point exceptions are ignored, and it does not set `errno`.
    Ceil(arithmetic::Ceil),
    /// `cir.cleanup.scope`
    /// Represents a scope with associated cleanup code
    ///
    /// `cir.cleanup.scope` contains a body region and a cleanup region. The body
    /// region is executed first, and the cleanup region is executed when the body
    /// region is exited, either normally or due to an exception.
    ///
    /// The cleanup kind attribute specifies when the cleanup region should be
    /// executed:
    /// - `none`: No cleanup (cleanup region is empty/unused)
    /// - `normal`: Cleanup is executed only on normal exit
    /// - `eh`: Cleanup is executed only on exception unwinding
    /// - `all`: Cleanup is executed on both normal exit and exception unwinding
    ///
    /// Examples:
    ///
    /// ```
    /// // Cleanup that runs on both normal and exception paths
    /// cir.cleanup.scope {
    ///   cir.call @mayThrow() : () -> ()
    ///   cir.yield
    /// } cleanup all {
    ///   cir.call @destructor() : () -> ()
    ///   cir.yield
    /// }
    ///
    /// // EH-only cleanup (destructor only called on exception)
    /// cir.cleanup.scope {
    ///   cir.call @mayThrow() : () -> ()
    ///   cir.yield
    /// } cleanup eh {
    ///   cir.call @destructor() : () -> ()
    ///   cir.yield
    /// }
    /// ```
    ///
    /// Both regions must be terminated. If a region has only one block, the
    /// terminator can be left out, and `cir.yield` will be inserted implicitly.
    CleanupScope(control_flow::CleanupScope),
    /// `cir.clear_cache`
    /// Clear the processor's instruction cache if required.
    ///
    /// The `cir.clear_cache` operation provides a representation for the
    /// `__builtin__clear_cache` builtin and corresponds to the
    /// `llvm.clear_cache` intrinsic in LLVM IR.
    ///
    /// This operation ensures visibility of modifications in the specified
    /// range to the execution unit of the processor. On targets with
    /// non-unified instruction and data cache, the implementation flushes
    /// the instruction cache.
    ///
    /// On platforms with coherent instruction and data caches (e.g., x86),
    /// this intrinsic is a nop. On platforms with non-coherent instruction
    /// and data cache (e.g., ARM, MIPS), the operation will be lowered
    /// either to appropriate instructions or a system call, if cache
    /// flushing requires special privileges.
    ///
    /// The default behavior is to emit a call to `__clear_cache` from the
    /// runtime library.
    ///
    /// This operation does not empty the instruction pipeline. Modifications
    /// of the current function are outside the scope of the operation.
    ClearCache(memory::ClearCache),
    /// `cir.clear_padding`
    /// Clear Padding Operation
    ///
    /// This operation represents a `__builtin_clear_padding` call, which sets all
    /// padding bits in the pointee `arg` to zero. The `arg`'s alignment is also
    /// stored in this operation so we can properly emit the stores/loads later.
    ///
    /// Offsets into the variable are stored as an `OffsetPairAttr`.
    ///
    /// During LLVM-IR lowering, this is converted into `getelementptr` and `store`
    /// operations.
    ///
    /// Example:
    ///
    /// ```
    /// cir.clear_padding(align(1) %1,
    ///                   [#cir.offset_pair<0, 2>, #cir.offset_pair<6, 8>])
    ///                   : <rec_Type> -> ()
    /// ```
    ClearPadding(memory::ClearPadding),
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
    Clrsb(misc::Clrsb),
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
    Clz(misc::Clz),
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
    Cmp(arithmetic::Cmp),
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
    Cmp3way(arithmetic::Cmp3way),
    /// `cir.co_return`
    /// Coroutine return operation
    ///
    /// The `cir.co_return` operation models a coroutine return point inside a
    /// `cir.coro.body` region.
    /// This operation is expected to appear only within a `cir.coro.body` region,
    /// but it may be nested within other operations or regions inside that body.
    CoReturn(misc::CoReturn),
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
    ComplexAdd(complex::ComplexAdd),
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
    ComplexConj(complex::ComplexConj),
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
    ComplexCreate(complex::ComplexCreate),
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
    ComplexDiv(complex::ComplexDiv),
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
    ComplexImag(complex::ComplexImag),
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
    ComplexImagPtr(complex::ComplexImagPtr),
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
    ComplexMul(complex::ComplexMul),
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
    ComplexReal(complex::ComplexReal),
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
    ComplexRealPtr(complex::ComplexRealPtr),
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
    ComplexSub(complex::ComplexSub),
    /// `cir.condition`
    /// Loop continuation condition.
    ///
    /// The `cir.condition` terminates conditional regions. It takes a single
    /// `cir.bool` operand and, depending on its value, may branch to different
    /// regions:
    ///
    ///  - When in the `cond` region of a loop, it continues the loop
    ///    if true, or exits it if false.
    ///  - When in the `ready` region of a `cir.await`, it branches to the `resume`
    ///    region when true, and to the `suspend` region when false.
    ///
    /// Example:
    ///
    /// ```
    /// cir.for : cond {
    ///   cir.condition(%val) // Branches to `body` region or exits.
    /// } body {
    ///   cir.yield
    /// } step {
    ///   cir.yield
    /// }
    ///
    /// cir.await(user, ready : {
    ///   cir.condition(%arg0) // Branches to `resume` or `suspend` region.
    /// }, suspend : {
    ///   [...]
    /// }, resume : {
    ///   [...]
    /// },)
    /// ```
    Condition(control_flow::Condition),
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
    Const(globals::Const),
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
    ConstructCatchParam(exceptions::ConstructCatchParam),
    /// `cir.continue`
    /// C/C++ `continue` statement equivalent
    ///
    /// The `cir.continue` operation is used to end execution of the current
    /// iteration of a loop and resume execution beginning at the next iteration.
    /// It is only allowed within loop regions.
    Continue(control_flow::Continue),
    /// `cir.copy`
    /// Copies contents from a CIR pointer to another
    ///
    /// Given two CIR pointers, `src` and `dst`, `cir.copy` will copy the memory
    /// pointed by `src` to the memory pointed by `dst`.
    ///
    /// The number of bytes copied is inferred from the pointee type. The pointee
    /// type of `src` and `dst` must match and both must implement the
    /// `DataLayoutTypeInterface`. The pointers may differ in address space (e.g.
    /// when assigning through an address-space-qualified pointer); when the two
    /// pointer types differ, both are printed, `src` first.
    ///
    /// The `volatile` keyword indicates that the operation is volatile.
    ///
    /// The `skip_tail_padding` keyword indicates that only the data bytes should
    /// be copied, excluding any tail padding. This is used when copying
    /// potentially-overlapping subobjects where the tail padding might be occupied
    /// by other objects (e.g. fields marked with `[[no_unique_address]]`). This
    /// is only valid when the pointee type is a record type.
    ///
    /// Examples:
    ///
    /// ```
    ///   // Copying contents from one record to another:
    ///   cir.copy %0 to %1 : !cir.ptr<!record_ty>
    ///
    ///   // Copying without tail padding (for overlapping subobjects):
    ///   cir.copy %0 to %1 skip_tail_padding : !cir.ptr<!record_ty>
    /// ```
    Copy(memory::Copy),
    /// `cir.copysign`
    /// Copies the sign of a floating-point value
    ///
    /// `cir.copysign` returns a value with the magnitude of the first operand
    /// and the sign of the second operand.
    Copysign(arithmetic::Copysign),
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
    CoroBody(misc::CoroBody),
    /// `cir.coro.intrinsic.alloc`
    /// Represents llvm.coro.alloc
    ///
    /// Queries whether the coroutine identified by `id` needs a dynamically
    /// allocated frame. Returns `true` if the coroutine frame must be allocated,
    /// or `false` otherwise.
    CoroIntrinsicAlloc(misc::CoroIntrinsicAlloc),
    /// `cir.coro.intrinsic.begin`
    /// Represents llvm.coro.begin
    ///
    /// Initializes the coroutine frame using `coroframeAddr`. `id` is the token
    /// from `coro.intrinsic.id`, and `coroframeAddr` points to the memory used
    /// for the coroutine frame. Returns the coroutine handle.
    CoroIntrinsicBegin(misc::CoroIntrinsicBegin),
    /// `cir.coro.intrinsic.end`
    /// Represents llvm.coro.end
    ///
    /// Marks a point at which a coroutine must be suspended or destroyed for the
    /// last time, e.g. right before the coroutine returns control to its caller
    /// for the final time, or along an exceptional unwind path. `handle` is the
    /// coroutine handle produced by `coro.intrinsic.begin`, and `unwind`
    /// indicates whether this occurrence of `coro.intrinsic.end` lies on the
    /// unwind path (`true`) or the normal control-flow path (`false`).
    CoroIntrinsicEnd(misc::CoroIntrinsicEnd),
    /// `cir.coro.intrinsic.free`
    /// Represents llvm.coro.free
    ///
    /// Given the coroutine identified by `id` and its frame pointer `coroframe`
    /// (the handle from `coro.intrinsic.begin`), returns the pointer that must
    /// be passed to the deallocation function to free the coroutine frame, or a
    /// null pointer if the coroutine frame was not dynamically allocated.
    CoroIntrinsicFree(misc::CoroIntrinsicFree),
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
    CoroIntrinsicId(misc::CoroIntrinsicId),
    /// `cir.coro.intrinsic.size`
    /// Represents llvm.coro.size
    ///
    /// Returns the size, in bytes, of the coroutine frame.
    CoroIntrinsicSize(misc::CoroIntrinsicSize),
    /// `cir.cos`
    /// Computes the floating-point cosine value
    ///
    /// `cir.cos` computes the cosine of a floating-point operand and returns
    /// a result of the same type.
    ///
    /// Floating-point exceptions are ignored, and it does not set `errno`.
    Cos(arithmetic::Cos),
    /// `cir.cosh`
    /// Computes the floating-point hyperbolic cosine value
    ///
    /// `cir.cosh` computes the hyperbolic cosine of a floating-point operand and
    /// returns a result of the same type.
    ///
    /// Floating-point exceptions are ignored, and it does not set `errno`.
    Cosh(arithmetic::Cosh),
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
    Cpuid(misc::Cpuid),
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
    Ctz(misc::Ctz),
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
    Dec(misc::Dec),
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
    DeleteArray(misc::DeleteArray),
    /// `cir.derived_class_addr`
    /// Get the derived class address for a class/struct
    ///
    /// The `cir.derived_class_addr` operaration gets the address of a particular
    /// derived class given a non-virtual base class pointer. The offset in bytes
    /// of the base class must be passed in, similar to `cir.base_class_addr`, but
    /// going into the other direction. This means lowering to a negative offset.
    ///
    /// The operation contains a flag for whether or not the operand may be nullptr.
    /// That depends on the context and cannot be known by the operation, and that
    /// information affects how the operation is lowered.
    ///
    /// The validity of the relationship of derived and base cannot yet be verified.
    /// If the target class is not a valid derived class for the object, the
    /// behavior is undefined.
    ///
    /// Example:
    /// ```c++
    /// class A {};
    /// class B : public A {};
    ///
    /// B *getAsB(A *a) {
    ///   return static_cast<B*>(a);
    /// }
    /// ```
    ///
    /// leads to
    /// ```
    ///   %2 = cir.load %0 : !cir.ptr<!cir.ptr<!rec_A>>, !cir.ptr<!rec_A>
    ///   %3 = cir.base_class_addr %2 : !cir.ptr<!rec_B> [0] -> !cir.ptr<!rec_A>
    /// ```
    DerivedClassAddr(memory::DerivedClassAddr),
    /// `cir.derived_data_member`
    /// Cast a base class data member pointer to a derived class data member pointer
    ///
    /// The `cir.derived_data_member` operation casts a data member pointer of type
    /// `T Base::*` to a data member pointer of type `T Derived::*`, where `Base`
    /// is an accessible non-ambiguous non-virtual base class of `Derived`.
    ///
    /// The `offset` parameter gives the offset in bytes of the `Base` base class
    /// subobject within a `Derived` object.
    DerivedDataMember(memory::DerivedDataMember),
    /// `cir.derived_method`
    ///
    /// Cast a base class pointer-to-member-function to a derived class
    /// pointer-to-member-function
    ///
    ///
    /// The `cir.derived_method` operation casts a pointer-to-member-function of
    /// type `Ret (Base::*)(Args)` to a pointer-to-member-function of type
    /// `Ret (Derived::*)(Args)`, where `Base` is a non-virtual base class of
    /// `Derived`.
    ///
    /// The `offset` parameter gives the offset in bytes of the `Base` base class
    /// subobject within a `Derived` object.
    ///
    /// Example:
    ///
    /// ```
    /// %1 = cir.derived_method %0 [16] : !cir.method<!cir.func<(!s32i)> in !rec_Base> -> !cir.method<!cir.func<(!s32i)> in !rec_Derived>
    /// ```
    DerivedMethod(memory::DerivedMethod),
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
    Div(arithmetic::Div),
    /// `cir.do`
    /// C/C++ do-while loop
    ///
    /// Represents a C/C++ do-while loop. Identical to `cir.while` but the
    /// condition is evaluated after the body. Because a variable cannot be
    /// declared in the condition of a do-while loop, this operation cannot
    /// have a `cleanup` region. A cleanup scope should be created within the
    /// body region for any variables within the loop that require cleanup.
    ///
    /// Example:
    ///
    /// ```
    /// cir.do {
    ///   cir.break
    /// ^bb2:
    ///   cir.yield
    /// } while {
    ///   cir.condition(%cond)
    /// }
    /// ```
    Do(control_flow::Do),
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
    DynCast(misc::DynCast),
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
    EhDispatch(exceptions::EhDispatch),
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
    EhInflightException(exceptions::EhInflightException),
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
    EhInitiate(exceptions::EhInitiate),
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
    EhLongjmp(exceptions::EhLongjmp),
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
    EhSetjmp(exceptions::EhSetjmp),
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
    EhTerminate(exceptions::EhTerminate),
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
    EhTypeid(exceptions::EhTypeid),
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
    EndCatch(exceptions::EndCatch),
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
    EndCleanup(exceptions::EndCleanup),
    /// `cir.exp`
    /// Computes the floating-point base-e exponential value
    ///
    /// `cir.exp` computes the exponential of a floating-point operand and returns
    /// a result of the same type.
    ///
    /// Floating-point exceptions are ignored, and it does not set `errno`.
    Exp(arithmetic::Exp),
    /// `cir.exp10`
    /// Computes the floating-point base-10 exponential value
    ///
    /// `cir.exp10` computes the base-10 exponential of a floating-point operand and
    /// returns a result of the same type.
    ///
    /// Floating-point exceptions are ignored, and it does not set `errno`.
    Exp10(arithmetic::Exp10),
    /// `cir.exp2`
    /// Computes the floating-point base-2 exponential value
    ///
    /// `cir.exp2` computes the base-2 exponential of a floating-point operand and
    ///  returns a result of the same type.
    ///
    /// Floating-point exceptions are ignored, and it does not set `errno`.
    Exp2(arithmetic::Exp2),
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
    Expect(misc::Expect),
    /// `cir.extract_member`
    /// Extract the value of a member of a record value
    ///
    /// The `cir.extract_member` operation extracts the value of a particular member
    /// from the input record. Unlike `cir.get_member` which derives pointers, this
    /// operation operates on values. It takes a value of record type and extracts
    /// the value of the specified record member from the input record value.
    ///
    /// Currently `cir.extract_member` does not work on unions.
    ///
    /// Example:
    ///
    /// ```
    /// // Suppose we have a record with multiple members.
    /// !s32i = !cir.int<s, 32>
    /// !s8i = !cir.int<s, 32>
    /// !record_ty = !cir.record<"struct.Bar" {!s32i, !s8i}>
    ///
    /// // And suppose we have a value of the record type.
    /// %0 = cir.const #cir.const_record<{#cir.int<1> : !s32i,
    ///                               #cir.int<2> : !s8i}> : !record_ty
    ///
    /// // Extract the value of the second member of the record.
    /// %1 = cir.extract_member %0[1] : !record_ty -> !s8i
    /// ```
    ExtractMember(memory::ExtractMember),
    /// `cir.fabs`
    /// Computes the floating-point absolute value
    ///
    /// `cir.fabs` computes the absolute value of a floating-point operand
    /// and returns a result of the same type.
    ///
    /// Floating-point exceptions are ignored, and it does not set `errno`.
    Fabs(arithmetic::Fabs),
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
    Fadd(arithmetic::Fadd),
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
    Fdiv(arithmetic::Fdiv),
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
    Ffs(misc::Ffs),
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
    Floor(arithmetic::Floor),
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
    Fma(arithmetic::Fma),
    /// `cir.fmaximum`
    /// Returns the larger of two floating-point values (IEEE 754-2019)
    ///
    /// `cir.fmaximum` returns the larger of its two operands according to
    /// IEEE 754-2019 semantics. If either operand is NaN, NaN is returned.
    Fmaximum(arithmetic::Fmaximum),
    /// `cir.fmaxnum`
    /// Returns the larger of two floating-point values
    ///
    /// `cir.fmaxnum` returns the larger of its two operands. If one operand is
    /// NaN, the other operand is returned.
    Fmaxnum(arithmetic::Fmaxnum),
    /// `cir.fminimum`
    /// Returns the smaller of two floating-point values (IEEE 754-2019)
    ///
    /// `cir.fminimum` returns the smaller of its two operands according to
    /// IEEE 754-2019 semantics. If either operand is NaN, NaN is returned.
    Fminimum(arithmetic::Fminimum),
    /// `cir.fminnum`
    /// Returns the smaller of two floating-point values
    ///
    /// `cir.fminnum` returns the smaller of its two operands. If one operand is
    /// NaN, the other operand is returned.
    Fminnum(arithmetic::Fminnum),
    /// `cir.fmod`
    /// Computes the floating-point remainder
    ///
    /// `cir.fmod` computes the floating-point remainder of dividing the first
    /// operand by the second operand.
    Fmod(arithmetic::Fmod),
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
    Fmul(arithmetic::Fmul),
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
    Fmuladd(arithmetic::Fmuladd),
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
    Fneg(arithmetic::Fneg),
    /// `cir.for`
    /// C/C++ for loop counterpart
    ///
    /// Represents a C/C++ for loop. It consists of three or four regions:
    ///
    ///  - `cond`: single block region with the loop's condition. Should be
    ///  terminated with a `cir.condition` operation.
    ///  - `body`: contains the loop body and an arbitrary number of blocks.
    ///  - `step`: single block region with the loop's step.
    ///  - `cleanup`: optional region that runs on every per-iteration exit edge
    ///  (condition-false exit, end-of-iteration after the step, break/continue,
    ///  and EH unwinding when the cleanup kind includes EH). This is used to
    ///  destroy a condition variable whose lifetime is a single iteration. When
    ///  present, it carries a cleanup kind matching `cir.cleanup.scope` (`normal`
    ///  or `all`).
    ///
    /// Example:
    ///
    /// ```
    /// cir.for : cond {
    ///   cir.condition(%val)
    /// } body {
    ///   cir.break
    /// ^bb2:
    ///   cir.yield
    /// } step {
    ///   cir.yield
    /// }
    ///
    /// cir.for : cond {
    ///   cir.condition(%val)
    /// } body {
    ///   cir.yield
    /// } step {
    ///   cir.yield
    /// } cleanup all {
    ///   cir.yield
    /// }
    /// ```
    For(control_flow::For),
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
    FrameAddress(misc::FrameAddress),
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
    Freeze(arithmetic::Freeze),
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
    Frem(arithmetic::Frem),
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
    Frexp(arithmetic::Frexp),
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
    Fsub(arithmetic::Fsub),
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
    Func(globals::Func),
    /// `cir.get_bitfield`
    /// Get the information for a bitfield member
    ///
    /// The `cir.get_bitfield` operation provides a load-like access to
    /// a bit field of a record.
    ///
    /// It expects a name if a bit field, a pointer to a storage in the
    /// base record, a type of the storage, a name of the bitfield,
    /// a size the bit field, an offset of the bit field and a sign.
    ///
    /// A unit attribute `volatile` can be used to indicate a volatile load of the
    /// bitfield.
    /// ```
    ///   cir.get_bitfield(#bfi, %0 {is_volatile} : !cir.ptr<!u64i>) -> !s32i
    /// ```
    ///
    /// Example:
    /// Suppose we have a struct with multiple bitfields stored in
    /// different members. The `cir.get_bitfield` operation gets the value
    /// of the bitfield.
    /// ```C++
    /// typedef struct {
    ///   int a : 4;
    ///   int b : 27;
    ///   int c : 17;
    ///   int d : 2;
    ///   int e : 15;
    /// } S;
    ///
    /// int load_bitfield(S& s) {
    ///   return s.e;
    /// }
    /// ```
    ///
    /// ```
    /// // 'e' is in the storage with the index 1
    /// !cir.struct<"S" packed {!u64i, !u16i,
    ///                         pad !cir.array<!u8i x 2>}>
    /// #bfi_e = #cir.bitfield_info<name = "e", storage_type = !u16i, size = 15,
    ///                             offset = 0, is_signed = true>
    ///
    /// %2 = cir.load %0 : !cir.ptr<!cir.ptr<!record_type>>, !cir.ptr<!record_type>
    /// %3 = cir.get_member %2[1] {name = "e"} : !cir.ptr<!record_type>
    ///                                                          -> !cir.ptr<!u16i>
    /// %4 = cir.get_bitfield align(4) (#bfi_e, %3 : !cir.ptr<!u16i>) -> !s32i
    /// ```
    GetBitfield(memory::GetBitfield),
    /// `cir.get_element`
    /// Get the address of an array element
    ///
    /// The `cir.get_element` operation gets the address of a particular element
    /// from the `base` array.
    ///
    /// It expects a pointer to the `base` array and the `index` of the element.
    /// The result pointer preserves the address space of the base pointer.
    ///
    /// Example:
    /// ```
    /// // Suppose we have a array.
    /// !s32i = !cir.int<s, 32>
    /// !arr_ty = !cir.array<!s32i x 4>
    ///
    /// // Get the address of the element at index 1.
    /// %elem_1 = cir.get_element %0[1 : !s32i] : !cir.ptr<!array_ty> -> !cir.ptr<!s32i>
    ///
    /// // Get the address of the element at index %i.
    /// %i = ...
    /// %elem_i = cir.get_element %0[%i : !s32i] : !cir.ptr<!array_ty> -> !cir.ptr<!s32i>
    ///
    /// // With address space (e.g., GPU private memory):
    /// %elem_gpu = cir.get_element %gpu_arr[%i : !s32i] :
    ///   !cir.ptr<!cir.array<!s32i x 10>, target_address_space(5)> ->
    ///   !cir.ptr<!s32i, target_address_space(5)>
    ///
    /// ```
    GetElement(memory::GetElement),
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
    GetGlobal(globals::GetGlobal),
    /// `cir.get_member`
    /// Get the address of a member of a record
    ///
    /// The `cir.get_member` operation gets the address of a particular named
    /// member from the input record.
    ///
    /// It expects a pointer to the base record as well as the name of the member
    /// and its field index.
    ///
    /// Example:
    /// ```
    /// // Suppose we have a record with multiple members.
    /// !s32i = !cir.int<s, 32>
    /// !s8i = !cir.int<s, 8>
    /// !ty_B = !cir.record<"struct.B" {!s32i, !s8i}>
    ///
    /// // Get the address of the member at index 1.
    /// %1 = cir.get_member %0[1] {name = "i"} : (!cir.ptr<!ty_B>) -> !cir.ptr<!s8i>
    /// ```
    GetMember(memory::GetMember),
    /// `cir.get_method`
    /// Resolve a method to a function pointer as callee
    ///
    /// The `cir.get_method` operation takes a pointer to method (!cir.method) and
    /// a pointer to a class object (!cir.ptr<!cir.record>>) as input, and
    /// yields a function pointer that points to the actual function corresponding
    /// to the input method. The operation also applies any necessary adjustments to
    /// the input object pointer for calling the method and yields the adjusted
    /// pointer.
    ///
    /// This operation is generated when calling a method through a pointer-to-
    /// member-function in C++:
    ///
    /// ```cpp
    /// // Foo *object;
    /// // int arg;
    /// // void (Foo::*method)(int);
    ///
    /// (object->*method)(arg);
    /// ```
    ///
    /// The code above will generate CIR similar to:
    ///
    /// ```
    /// %callee, %this = cir.get_method %method, %object
    /// cir.call %callee(%this, %arg)
    /// ```
    ///
    /// The method type must match the callee type. That is:
    /// - The return type of the method must match the return type of the callee.
    /// - The first parameter of the callee must have type `!cir.ptr<!cir.void>`.
    /// - Types of other parameters of the callee must match the method's
    ///   parameters after the implicit `this` pointer.
    GetMethod(memory::GetMethod),
    /// `cir.get_runtime_member`
    /// Get the address of a member of a record
    ///
    /// The `cir.get_runtime_member` operation gets the address of a member from
    /// the input record. The target member is given by a value of type
    /// `!cir.data_member` (i.e. a pointer-to-data-member value).
    ///
    /// This operation differs from `cir.get_member` in when the target member can
    /// be determined. For the `cir.get_member` operation, the target member is
    /// specified as a constant index so the member it returns access to is known
    /// when the operation is constructed. For the `cir.get_runtime_member`
    /// operation, the target member is given through a pointer-to-data-member
    /// value which is unknown until the program being compiled is executed. In
    /// other words, `cir.get_member` represents a normal member access through the
    /// `.` operator in C/C++:
    ///
    /// ```cpp
    /// struct Foo { int x; };
    /// Foo f;
    /// (void)f.x;  // cir.get_member
    /// ```
    ///
    /// And `cir.get_runtime_member` represents a member access through the `.*` or
    /// the `->*` operator in C++:
    ///
    /// ```cpp
    /// struct Foo { int x; }
    /// Foo f;
    /// Foo *p;
    /// int Foo::*member;
    ///
    /// (void)f.*member;   // cir.get_runtime_member
    /// (void)p->*member;  // cir.get_runtime_member
    /// ```
    ///
    /// This operation expects a pointer to the base record as well as the pointer
    /// to the target member.
    GetRuntimeMember(memory::GetRuntimeMember),
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
    ///
    /// The `init_priority` attribute records the priority specified by a
    /// C++ `init_priority` attribute on the source variable declaration.
    /// LoweringPrepare uses it to place the dynamic initializer emitted from
    /// `ctorRegion` into a priority-specific `_GLOBAL__I_<priority>` function
    /// instead of the default `_GLOBAL__sub_I_*` function.
    Global(globals::Global),
    /// `cir.goto`
    ///
    ///
    /// Transfers control to the specified `label`. This requires a corresponding
    /// `cir.label` to exist and is used by to represent source level `goto`s
    /// that jump across region boundaries. Alternatively, `cir.br` is used to
    /// construct goto's that don't violate such boundaries.
    ///
    /// `cir.goto` is completely symbolic (i.e. it "jumps" on a label that isn't
    /// yet materialized) and should be taken into account by passes and analysis
    /// when deciding if it's safe to make some assumptions about a given region
    /// or basic block.
    ///
    /// Example:
    /// ```C++
    ///   int test(int x) {
    ///     if (x)
    ///       goto label;
    ///     {
    ///       x = 10;
    ///   label:
    ///       return x;
    ///     }
    ///   }
    /// ```
    ///
    /// ```
    ///   cir.scope {  // REGION #1
    ///     %2 = cir.load %0 : !cir.ptr<!s32i>, !s32i
    ///     %3 = cir.cast int_to_bool %2 : !s32i -> !cir.bool
    ///     cir.if %3 {
    ///       cir.goto "label"
    ///     }
    ///     }
    ///     cir.scope {  // REGION #2
    ///       %2 = cir.const #cir.int<10> : !s32i
    ///       cir.store %2, %0 : !s32i, !cir.ptr<!s32i>
    ///       cir.br ^bb1
    ///     ^bb1:  // pred: ^bb0
    ///       cir.label "label"
    ///       %3 = cir.load %0 : !cir.ptr<!s32i>, !s32i
    ///       cir.store %3, %1 : !s32i, !cir.ptr<!s32i>
    ///       %4 = cir.load %1 : !cir.ptr<!s32i>, !s32i
    ///       cir.return %4 : !s32i
    ///     }
    ///     cir.unreachable
    /// ```
    Goto(control_flow::Goto),
    /// `cir.if`
    /// the if-then-else operation
    ///
    /// The `cir.if` operation represents an if-then-else construct for
    /// conditionally executing two regions of code. The operand is a `cir.bool`
    /// type.
    ///
    /// Examples:
    ///
    /// ```
    /// cir.if %cond  {
    ///   ...
    /// } else {
    ///   ...
    /// }
    ///
    /// cir.if %cond  {
    ///   ...
    /// }
    ///
    /// cir.if %cond  {
    ///   ...
    ///   cir.br ^a
    /// ^a:
    ///   cir.yield
    /// }
    /// ```
    ///
    /// `cir.if` defines no values and the 'else' can be omitted. The if/else
    /// regions must be terminated. If the region has only one block, the terminator
    /// can be left out, and `cir.yield` terminator will be inserted implictly.
    /// Otherwise, the region must be explicitly terminated.
    If(control_flow::If),
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
    Inc(misc::Inc),
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
    IndirectBr(misc::IndirectBr),
    /// `cir.indirect_goto`
    /// Symbolic indirect goto
    ///
    /// Transfers control to the block whose address is held in `$addr`, the
    /// void-pointer value of a `goto *expr;` (the GNU computed-goto extension).
    /// Like `cir.goto`, it is symbolic: it references no successor block, so it is
    /// valid inside any region even before `FlattenCFG` merges nested scopes.
    /// `GotoSolver` runs after `FlattenCFG` and rewrites each `cir.indirect_goto`
    /// into a `cir.br` to a shared block holding a `cir.indirect_br` over every
    /// address-taken label.
    ///
    /// Example:
    ///
    /// ```mlir
    ///   %0 = cir.load %p : !cir.ptr<!cir.ptr<!void>>, !cir.ptr<!void>
    ///   cir.indirect_goto %0 : !cir.ptr<!void>
    /// ```
    IndirectGoto(control_flow::IndirectGoto),
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
    InitCatchParam(exceptions::InitCatchParam),
    /// `cir.insert_member`
    /// Overwrite the value of a member of a record value
    ///
    /// The `cir.insert_member` operation overwrites the value of a particular
    /// member in the input record, and returns the modified record. The result of
    /// this operation is equal to the input record, except for the member specified
    /// by `index_attr` whose value is equal to the given value.
    ///
    /// This operation is named after the LLVM instruction `insertvalue`.
    ///
    /// Currently `cir.insert_member` does not work on unions.
    ///
    /// Example:
    ///
    /// ```
    /// // Suppose we have a record with multiple members.
    /// !s32i = !cir.int<s, 32>
    /// !s8i = !cir.int<s, 32>
    /// !record_ty = !cir.record<"struct.Bar" {!s32i, !s8i}>
    ///
    /// // And suppose we have a value of the record type.
    /// %0 = cir.const #cir.const_record<{#cir.int<1> : !s32i, #cir.int<2> : !s8i}> : !record_ty
    /// // %0 is {1, 2}
    ///
    /// // Overwrite the second member of the record value.
    /// %1 = cir.const #cir.int<3> : !s8i
    /// %2 = cir.insert_member %0[1], %1 : !record_ty, !s8i
    /// // %2 is {1, 3}
    /// ```
    InsertMember(memory::InsertMember),
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
    IsConstant(arithmetic::IsConstant),
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
    IsFpClass(arithmetic::IsFpClass),
    /// `cir.label`
    ///
    /// An identifier which may be referred by cir.goto operation
    Label(control_flow::Label),
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
    Launder(misc::Launder),
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
    LibcMemchr(stdlib::LibcMemchr),
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
    LibcMemcpy(stdlib::LibcMemcpy),
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
    LibcMemmove(stdlib::LibcMemmove),
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
    LibcMemset(stdlib::LibcMemset),
    /// `cir.lifetime.end`
    /// Marks the end of the lifetime of a variable produced by a cir.alloca operation
    ///
    /// The `cir.lifetime.end` operation marks the end of the lifetime of the
    /// storage pointed to by `$ptr`. After this operation the underlying storage
    /// is considered dead, and the optimizer is free to reuse the storage for
    /// other purposes, until a subsequent `cir.lifetime.start` on the same
    /// pointer revives it.
    ///
    /// The `cir.scope` is the operation that models the block scope of the C/C++
    /// source. Once ClangIR is no longer structured, `cir.scope` can no longer express
    /// the lifetime of a local variable. This happens, for example, after `FlattenCFG`,
    /// where `cir.scope` regions are dissolved into plain basic blocks, or after
    /// `HoistAllocas`, where an alloca is moved to the function entry so that its
    /// position no longer reflects the scope it was declared in. In that form, these
    /// lifetime markers are what delimit the beginning and end of a variable's
    /// lifetime.
    ///
    /// The verifier requires `$ptr` to be produced by a `cir.alloca`.
    /// `cir.lifetime.end` should be preceded by a matching `cir.lifetime.start` on
    /// the same pointer on every control-flow path that reaches it.
    ///
    /// This operation corresponds to the LLVM intrinsic `llvm.lifetime.end`.
    ///
    /// Example:
    /// ```
    /// cir.lifetime.end %ptr : !cir.ptr<!s32i>
    /// ```
    LifetimeEnd(memory::LifetimeEnd),
    /// `cir.lifetime.start`
    /// Marks the start of the lifetime of a variable produced by a cir.alloca operation
    ///
    /// The `cir.lifetime.start` operation marks the beginning of the lifetime
    /// of the storage pointed to by `$ptr`. Between this operation and a
    /// matching `cir.lifetime.end` on the same pointer, the underlying storage
    /// is considered live; outside that range it is considered dead, and the
    /// optimizer is free to reuse the storage for other purposes.
    ///
    /// The `cir.scope` is the operation that models the block scope of the C/C++
    /// source. Once ClangIR is no longer structured, `cir.scope` can no longer express
    /// the lifetime of a local variable. This happens, for example, after `FlattenCFG`,
    /// where `cir.scope` regions are dissolved into plain basic blocks, or after
    /// `HoistAllocas`, where an alloca is moved to the function entry so that its
    /// position no longer reflects the scope it was declared in. In that form, these
    /// lifetime markers are what delimit the beginning and end of a variable's
    /// lifetime.
    ///
    /// The verifier requires `$ptr` to be produced by a `cir.alloca`. For the
    /// lifetime to be meaningful, a matching `cir.lifetime.end` on the same
    /// pointer should follow on every control-flow path. This is different from
    /// LLVM, where an `llvm.lifetime.start` may appear without a matching
    /// `llvm.lifetime.end` -- there the storage is also implicitly marked dead
    /// when the function returns (see the
    /// [LLVM LangRef](https://llvm.org/docs/LangRef.html#int-lifestart)).
    ///
    /// This operation corresponds to the LLVM intrinsic `llvm.lifetime.start`.
    ///
    /// Example:
    /// ```
    /// cir.lifetime.start %ptr : !cir.ptr<!s32i>
    /// ```
    LifetimeStart(memory::LifetimeStart),
    /// `cir.llrint`
    /// Rounds floating-point to long long integer using current rounding mode
    ///
    /// `cir.llrint` rounds a floating-point value to the nearest integer value
    /// using the current rounding mode and returns the result as a `long long`.
    Llrint(arithmetic::Llrint),
    /// `cir.llround`
    /// Rounds floating-point to long long integer
    ///
    /// `cir.llround` rounds a floating-point value to the nearest integer value,
    /// rounding halfway cases away from zero, and returns the result as a
    /// `long long`.
    Llround(arithmetic::Llround),
    /// `cir.load`
    /// Load value from memory adddress
    ///
    /// `cir.load` reads a value (lvalue to rvalue conversion) given an address
    /// backed up by a `cir.ptr` type. A unit attribute `deref` can be used to
    /// mark the resulting value as used by another operation to dereference
    /// a pointer. A unit attribute `volatile` can be used to indicate a volatile
    /// loading. Load can be marked atomic by using `atomic(<mem_order>)`.
    ///
    /// `alignment` can be used to specify an alignment that's different from the
    /// default, which is computed from `result`'s type ABI data layout.
    ///
    /// A unit attribute `invariant` can be used to indicate that the loaded memory
    /// never changes, mapping to LLVM IR's `!invariant.load` metadata.
    ///
    /// Example:
    ///
    /// ```
    ///
    /// // Read from local variable, address in %0.
    /// %1 = cir.load %0 : !cir.ptr<i32>, i32
    ///
    /// // Load address from memory at address %0. %3 is used by at least one
    /// // operation that dereferences a pointer.
    /// %3 = cir.load deref %0 : !cir.ptr<!cir.ptr<i32>>
    ///
    /// // Perform a volatile load from address in %0.
    /// %4 = cir.load volatile %0 : !cir.ptr<i32>, i32
    ///
    /// // Perform an invariant load from address in %0.
    /// %5 = cir.load invariant %0 : !cir.ptr<i32>, i32
    ///
    /// // Others
    /// %x = cir.load align(16) atomic(seq_cst) %0 : !cir.ptr<i32>, i32
    /// ```
    Load(memory::Load),
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
    LocalInit(globals::LocalInit),
    /// `cir.log`
    /// Computes the floating-point natural logarithm
    ///
    /// `cir.log` computes the natural logarithm of a floating-point operand and
    /// returns a result of the same type.
    ///
    /// Floating-point exceptions are ignored, and it does not set `errno`.
    Log(arithmetic::Log),
    /// `cir.log10`
    /// Computes the floating-point base-10 logarithm
    ///
    /// `cir.log10` computes the base-10 logarithm of a floating-point operand and
    /// returns a result of the same type.
    ///
    /// Floating-point exceptions are ignored, and it does not set `errno`.
    Log10(arithmetic::Log10),
    /// `cir.log2`
    /// Computes the floating-point base-2 logarithm
    ///
    /// `cir.log2` computes the base-2 logarithm of a floating-point operand and
    /// returns a result of the same type.
    ///
    /// Floating-point exceptions are ignored, and it does not set `errno`.
    Log2(arithmetic::Log2),
    /// `cir.lrint`
    /// Rounds floating-point to long integer using current rounding mode
    ///
    /// `cir.lrint` rounds a floating-point value to the nearest integer value
    /// using the current rounding mode and returns the result as a `long`.
    Lrint(arithmetic::Lrint),
    /// `cir.lround`
    /// Rounds floating-point to long integer
    ///
    /// `cir.lround` rounds a floating-point value to the nearest integer value,
    /// rounding halfway cases away from zero, and returns the result as a `long`.
    Lround(arithmetic::Lround),
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
    Max(arithmetic::Max),
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
    Min(arithmetic::Min),
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
    Minus(misc::Minus),
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
    Modf(arithmetic::Modf),
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
    Mul(arithmetic::Mul),
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
    MulOverflow(misc::MulOverflow),
    /// `cir.nearbyint`
    /// Rounds floating-point value to nearest integer
    ///
    /// `cir.nearbyint` rounds a floating-point operand to the nearest integer value
    /// and returns a result of the same type.
    ///
    /// Floating-point exceptions are ignored, and it does not set `errno`.
    Nearbyint(arithmetic::Nearbyint),
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
    Not(misc::Not),
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
    Objsize(arithmetic::Objsize),
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
    Or(arithmetic::Or),
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
    Parity(misc::Parity),
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
    Popcount(misc::Popcount),
    /// `cir.pow`
    /// Computes the power of a floating-point value
    ///
    /// `cir.pow` computes the first operand raised to the power of the second
    /// operand.
    Pow(arithmetic::Pow),
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
    Prefetch(misc::Prefetch),
    /// `cir.ptr_diff`
    /// Pointer subtraction arithmetic
    ///
    /// The cir.ptr_diff operation computes the difference between two pointers that
    /// have the same element type.
    ///
    /// The result reflects the ABI-defined size of the pointed-to type. For example,
    /// subtracting two !cir.ptr<!u64i> values may yield 1, representing an 8-byte
    /// difference. In contrast, for pointers to void or function types, a result of
    /// 8 corresponds to an 8-byte difference.
    ///
    /// For pointers to types whose size are not aligned with the target data
    /// layout, the size is generally rounded to the next power of 2 bits. For
    /// example, subtracting two !cir.ptr<!s24i> values for the _BitInt(24) type may
    /// yield 1, representing a 4-byte difference (as opposed to a 3-byte
    /// difference).
    ///
    /// Example:
    ///
    /// ```
    /// %7 = cir.ptr_diff %0, %1 : !cir.ptr<!u64i> -> !u64i
    /// ```
    PtrDiff(memory::PtrDiff),
    /// `cir.ptr_stride`
    /// Pointer access with stride
    ///
    /// The `cir.ptr_stride` operation computes a new pointer from a base pointer
    /// and an integer stride, similar to a single-index `getelementptr` in LLVM IR.
    /// It moves the pointer by `stride * sizeof(element_type)` bytes.
    ///
    /// ```
    /// %3 = cir.const 0 : i32
    /// %3 = cir.ptr_stride %1, %2 : (!cir.ptr<i32>, i32) -> !cir.ptr<i32>
    /// ```
    PtrStride(memory::PtrStride),
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
    Rem(arithmetic::Rem),
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
    Resume(exceptions::Resume),
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
    ResumeFlat(exceptions::ResumeFlat),
    /// `cir.return`
    /// Return from function
    ///
    /// The "return" operation represents a return operation within a function.
    /// The operation takes an optional operand and produces no results.
    /// The operand type must match the signature of the function that contains
    /// the operation.
    ///
    /// ```
    ///   func @foo() -> i32 {
    ///     ...
    ///     cir.return %0 : i32
    ///   }
    /// ```
    Return(control_flow::Return),
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
    ReturnAddress(misc::ReturnAddress),
    /// `cir.rint`
    /// Rounds floating-point value to nearest integer
    ///
    /// `cir.rint` rounds a floating-point operand to the nearest integer value
    /// and returns a result of the same type.
    ///
    /// This operation does not set `errno`. Unlike `cir.nearbyint`, this operation
    /// may raise the `FE_INEXACT` exception if the input value is not an exact
    /// integer, but this is not guaranteed to happen.
    Rint(arithmetic::Rint),
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
    Rotate(arithmetic::Rotate),
    /// `cir.round`
    /// Rounds floating-point value to nearest integer
    ///
    /// `cir.round` rounds a floating-point operand to the nearest integer value
    /// and returns a result of the same type.
    ///
    /// Floating-point exceptions are ignored, and it does not set `errno`.
    Round(arithmetic::Round),
    /// `cir.roundeven`
    /// Rounds floating-point value to nearest integer, ties to even
    ///
    /// `cir.roundeven` rounds a floating-point operand to the nearest integer
    /// value, with ties rounding to even (banker's rounding).
    ///
    /// Floating-point exceptions are ignored, and it does not set `errno`.
    Roundeven(arithmetic::Roundeven),
    /// `cir.scope`
    /// Represents a C/C++ scope
    ///
    /// `cir.scope` contains one region and defines a strict "scope" for all new
    /// values produced within its blocks.
    ///
    /// The region can contain an arbitrary number of blocks but usually defaults
    /// to one and can optionally return a value (useful for representing values
    /// coming out of C++ full-expressions) via `cir.yield`:
    ///
    ///
    /// ```
    /// %rvalue = cir.scope {
    ///   ...
    ///   cir.yield %value
    /// }
    /// ```
    ///
    /// The blocks can be terminated by `cir.yield`, `cir.return` or `cir.throw`.
    /// If `cir.scope` yields no value, the `cir.yield` can be left out, and
    /// will be inserted implicitly.
    Scope(control_flow::Scope),
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
    Select(arithmetic::Select),
    /// `cir.set_bitfield`
    /// Set the value of a bitfield member
    ///
    /// The `cir.set_bitfield` operation provides a store-like access to
    /// a bit field of a record.
    ///
    /// A bitfield info attribute must be provided to describe the location of
    /// the bitfield within the memory referenced by the $addr argument.
    /// The $src argument is inserted at the appropriate place in the memory and
    /// the value that was stored. Returns the value being stored.
    ///
    /// A unit attribute `volatile` can be used to indicate a volatile store of the
    /// bitfield.
    ///   ```
    ///     cir.set_bitfield(#bfi, %0 : !cir.ptr<!u32i>, %1 : !s32i) {is_volatile}
    ///                                                                    -> !s32i
    ///   ```
    ///
    /// Example.
    /// Suppose we have a struct with multiple bitfields stored in
    /// different storages. The `cir.set_bitfield` operation sets the value
    /// of the bitfield.
    /// ```C++
    /// typedef struct {
    ///   int a : 4;
    ///   int b : 27;
    ///   int c : 17;
    ///   int d : 2;
    ///   int e : 15;
    /// } S;
    ///
    /// void store_bitfield(S& s) {
    ///   s.e = 3;
    /// }
    /// ```
    ///
    /// ```
    /// // 'e' is in the storage with the index 1
    /// !record_type = !cir.struct<"S" packed {!u64i, !u16i,
    ///                                        pad !cir.array<!u8i x 2>}>
    /// #bfi_e = #cir.bitfield_info<name = "e", storage_type = !u16i, size = 15,
    ///                             offset = 0, is_signed = true>
    ///
    /// %1 = cir.const #cir.int<3> : !s32i
    /// %2 = cir.load %0 : !cir.ptr<!cir.ptr<!record_type>>, !cir.ptr<!record_type>
    /// %3 = cir.get_member %2[1] {name = "e"} : !cir.ptr<!record_type>
    ///                                                          -> !cir.ptr<!u16i>
    /// %4 = cir.set_bitfield align(4) (#bfi_e, %3 : !cir.ptr<!u16i>, %1 : !s32i)
    ///                                                                    -> !s32i
    /// ```
    SetBitfield(memory::SetBitfield),
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
    Shift(arithmetic::Shift),
    /// `cir.signbit`
    /// Checks the sign of a floating-point number
    ///
    /// It returns whether the sign bit (i.e. the highest bit) of the input operand
    /// is set.
    Signbit(misc::Signbit),
    /// `cir.sin`
    /// Computes the floating-point sine
    ///
    /// `cir.sin` computes the sine of a floating-point operand and returns
    /// a result of the same type.
    ///
    /// Floating-point exceptions are ignored, and it does not set `errno`.
    Sin(arithmetic::Sin),
    /// `cir.sinh`
    /// Computes the floating-point hyperbolic sine
    ///
    /// `cir.sinh` computes the hyperbolic sine of a floating-point operand and
    /// returns a result of the same type.
    ///
    /// Floating-point exceptions are ignored, and it does not set `errno`.
    Sinh(arithmetic::Sinh),
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
    Sqrt(arithmetic::Sqrt),
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
    Stackrestore(misc::Stackrestore),
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
    Stacksave(misc::Stacksave),
    /// `cir.std.find`
    /// std::find()
    StdFind(stdlib::StdFind),
    /// `cir.std.strlen`
    /// C standard library strlen()
    StdStrlen(stdlib::StdStrlen),
    /// `cir.store`
    /// Store value to memory address
    ///
    /// `cir.store` stores a value (first operand) to the memory address specified
    /// in the second operand. A unit attribute `volatile` can be used to indicate
    /// a volatile store. Store's can be marked atomic by using
    /// `atomic(<mem_order>)`.
    ///
    /// `alignment` can be used to specify an alignment that's different from the
    /// default, which is computed from `result`'s type ABI data layout.
    ///
    /// Example:
    ///
    /// ```
    /// // Store a function argument to local storage, address in %0.
    /// cir.store %arg0, %0 : i32, !cir.ptr<i32>
    ///
    /// // Perform a volatile store into memory location at the address in %0.
    /// cir.store volatile %arg0, %0 : i32, !cir.ptr<i32>
    ///
    /// // Others
    /// cir.store align(16) atomic(seq_cst) %x, %addr : i32, !cir.ptr<i32>
    /// ```
    Store(memory::Store),
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
    Sub(arithmetic::Sub),
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
    SubOverflow(misc::SubOverflow),
    /// `cir.switch`
    /// Switch operation
    ///
    /// The `cir.switch` operation represents C/C++ switch functionality for
    /// conditionally executing multiple regions of code. The operand to an switch
    /// is an integral condition value.
    ///
    /// Besides taking an integer condition and CIR regions, it also accepts an
    /// `all_enum_cases_covered` attribute indicating whether all enum cases are
    /// handled by the operation. Note that the presence of a default CaseOp does
    /// not imply `all_enum_cases_covered`. The original AST switch must explicitly list
    /// every enum case.
    ///
    /// The set of `cir.case` operations and their enclosing `cir.switch`
    /// represent the semantics of a C/C++ switch statement. Users can use
    /// `collectCases(llvm::SmallVector<CaseOp> &cases)` to collect the `cir.case`
    /// operation in the `cir.switch` operation easily.
    ///
    /// The `cir.case` operations don't have to be in the region of `cir.switch`
    /// directly. However, when all the `cir.case` operations live in the region
    /// of `cir.switch` directly and there are no other operations except the ending
    /// `cir.yield` operation in the region of `cir.switch` directly, we say the
    /// `cir.switch` operation is in a simple form. Users can use
    /// `bool isSimpleForm(llvm::SmallVector<CaseOp> &cases)` member function to
    /// detect if the `cir.switch` operation is in a simple form. The simple form
    /// makes it easier for analyses to handle the `cir.switch` operation
    /// and makes the boundary to give up clear.
    ///
    /// To make the simple form as common as possible, CIR code generation attaches
    /// operations corresponding to the statements that lives between top level
    /// cases into the closest `cir.case` operation.
    ///
    /// For example,
    ///
    /// ```
    /// switch(int cond) {
    ///   case 4:
    ///     a++;
    ///     b++;
    ///   case 5:
    ///     c++;
    ///
    ///   ...
    /// }
    /// ```
    ///
    /// The statement `b++` is not a sub-statement of the case statement `case 4`.
    /// But to make the generated `cir.switch` a simple form, we will attach the
    /// statement `b++` into the closest `cir.case` operation. So that the generated
    /// code will be like:
    ///
    /// ```
    /// cir.switch(int cond) {
    ///   cir.case(equal, 4) {
    ///     a++;
    ///     b++;
    ///     cir.yield
    ///   }
    ///   cir.case(equal, 5) {
    ///     c++;
    ///     cir.yield
    ///   }
    ///   ...
    /// }
    /// ```
    ///
    /// For the same reason, we will hoist the case statement as the substatement
    /// of another case statement so that they will be in the same level. For
    /// example,
    ///
    /// ```
    /// switch(int cond) {
    ///   case 4:
    ///   default;
    ///   case 5:
    ///     a++;
    ///   ...
    /// }
    /// ```
    ///
    /// will be generated as
    ///
    /// ```
    /// cir.switch(int cond) {
    ///   cir.case(equal, 4) {
    ///     cir.yield
    ///   }
    ///   cir.case(default) {
    ///     cir.yield
    ///   }
    ///   cir.case(equal, 5) {
    ///     a++;
    ///     cir.yield
    ///   }
    ///   ...
    /// }
    /// ```
    ///
    /// The cir.switch is not be considered "simple" if any of the following is
    /// true:
    /// - There are case statements of the switch statement that are scope
    ///   other than the top level compound statement scope. Note that a case
    ///   statement itself doesn't form a scope.
    /// - The sub-statement of the switch statement is not a compound statement.
    /// - There is any code before the first case statement. For example,
    ///
    /// ```
    /// switch(int cond) {
    ///   l:
    ///     b++;
    ///
    ///   case 4:
    ///     a++;
    ///     break;
    ///
    ///   case 5:
    ///     goto l;
    ///   ...
    /// }
    /// ```
    ///
    /// the generated CIR for this non-simple switch would be:
    ///
    /// ```
    /// cir.switch(int cond) {
    ///   cir.label "l"
    ///   b++;
    ///   cir.case(4) {
    ///     a++;
    ///     cir.break
    ///   }
    ///   cir.case(5) {
    ///     goto "l"
    ///   }
    ///   cir.yield
    /// }
    /// ```
    Switch(control_flow::Switch),
    /// `cir.switch.flat`
    /// A flattened version of cir.switch
    ///
    /// The `cir.switch.flat` operation is a region-less and simplified
    /// version of the `cir.switch`.
    /// Its representation is closer to LLVM IR dialect
    /// than the C/C++ language feature.
    SwitchFlat(control_flow::SwitchFlat),
    /// `cir.tan`
    /// Computes the floating-point tangent
    ///
    /// `cir.tan` computes the tangent of a floating-point operand and returns
    /// a result of the same type.
    ///
    /// Floating-point exceptions are ignored, and it does not set `errno`.
    Tan(arithmetic::Tan),
    /// `cir.tanh`
    /// Computes the floating-point hyperbolic tangent
    ///
    /// `cir.tanh` computes the hyperbolic tangent of a floating-point operand and
    /// returns a result of the same type.
    ///
    /// Floating-point exceptions are ignored, and it does not set `errno`.
    Tanh(arithmetic::Tanh),
    /// `cir.ternary`
    /// The `cond ? a : b` C/C++ ternary operation
    ///
    /// The `cir.ternary` operation represents C/C++ ternary, much like a `select`
    /// operation. The first argument is a `cir.bool` condition to evaluate, followed
    /// by two regions to execute (true or false). This is different from `cir.if`
    /// since each region is one block sized and the `cir.yield` closing the block
    /// scope should have one argument.
    ///
    /// `cir.ternary` also represents the GNU binary conditional operator ?: which
    /// reuses the parent operation for both the condition and the true branch to
    /// evaluate it only once.
    ///
    /// Example:
    ///
    /// ```
    /// // cond = a && b;
    ///
    /// %x = cir.ternary (%cond, true_region {
    ///   ...
    ///   cir.yield %a : i32
    /// }, false_region {
    ///   ...
    ///   cir.yield %b : i32
    /// }) -> i32
    /// ```
    Ternary(control_flow::Ternary),
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
    Throw(exceptions::Throw),
    /// `cir.token.none`
    /// Produces an empty token value.
    ///
    /// Produces a `none` token value, mirroring LLVM IR's `none` token
    /// literal. Lowers to `llvm::ConstantTokenNone`.
    TokenNone(exceptions::TokenNone),
    /// `cir.trap`
    /// Exit the program abnormally
    ///
    /// The cir.trap operation causes the program to exit abnormally. The
    /// implementations may implement this operation with different mechanisms. For
    /// example, an implementation may implement this operation by calling abort,
    /// while another implementation may implement this operation by executing an
    /// illegal instruction.
    Trap(control_flow::Trap),
    /// `cir.trunc`
    /// Truncates floating-point value to integer
    ///
    /// `cir.trunc` truncates a floating-point operand to an integer value
    /// and returns a result of the same type.
    ///
    /// Floating-point exceptions are ignored, and it does not set `errno`.
    Trunc(arithmetic::Trunc),
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
    Try(exceptions::Try),
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
    TryCall(misc::TryCall),
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
    TryThrow(exceptions::TryThrow),
    /// `cir.unreachable`
    /// invoke immediate undefined behavior
    ///
    /// If the program control flow reaches a `cir.unreachable` operation, the
    /// program exhibits undefined behavior immediately. This operation is useful
    /// in cases where the unreachability of a program point needs to be explicitly
    /// marked.
    Unreachable(control_flow::Unreachable),
    /// `cir.va_arg`
    /// Fetches next variadic element as a given type
    ///
    /// The `cir.va_arg` operation models the C/C++ `va_arg` macro by reading the
    /// next argument from an active variable argument list and producing it as a
    /// value of a specified result type.
    ///
    /// The operand must be a pointer to the target's `va_list` representation.
    /// The operation advances the `va_list` state as a side effect and returns
    /// the fetched value as the result, whose type is chosen by the user of the
    /// operation.
    ///
    /// A `cir.va_arg` must only be used on a `va_list` that has been initialized
    /// with `cir.va.start` and not yet finalized by `cir.va.end`. The semantics
    /// (including alignment and promotion rules) follow the platform ABI; the
    /// frontend is responsible for providing a `va_list` pointer that matches the
    /// target representation.
    ///
    /// Example:
    /// ```
    /// // %args : !cir.ptr<!cir.array<!rec___va_list_tag x 1>>
    /// %p = cir.cast array_to_ptrdecay %args
    ///         : !cir.ptr<!cir.array<!rec___va_list_tag x 1>>
    ///         -> !cir.ptr<!rec___va_list_tag>
    /// cir.va.start %p : !cir.ptr<!rec___va_list_tag>
    ///
    /// // Fetch an `int` from the vararg list.
    /// %v = cir.va_arg %p : (!cir.ptr<!rec___va_list_tag>) -> !s32i
    ///
    /// cir.va.end %p : !cir.ptr<!rec___va_list_tag>
    /// ```
    VaArg(varargs::VaArg),
    /// `cir.va_copy`
    /// Copied a variable argument list
    ///
    /// The `cir.copy` operation models the C/C++ va_copy macro.
    /// The variable argument list passed as the `$src_list` is copied to an
    /// unitialized `va_list` in the destination operand. The next argument that
    /// can be extracted from the copied list is the same as the next argument in
    /// the source list. The copied list must be destroyed with `va_end`.
    ///
    /// Example:
    ///
    /// ```
    /// // %args : !cir.ptr<!cir.array<!rec___va_list_tag x 1>>
    /// %p = cir.cast array_to_ptrdecay %args
    ///       : !cir.ptr<!cir.array<!rec___va_list_tag x 1>>
    ///       -> !cir.ptr<!rec___va_list_tag>
    /// cir.va_copy %p to %dst
    ///       : (!cir.ptr<!rec___va_list_tag>, !cir.ptr<!rec___va_list_tag>)
    /// ```
    VaCopy(varargs::VaCopy),
    /// `cir.va_end`
    /// Ends a variable argument list
    ///
    /// The `cir.va_end` operation models the C/C++ va_end macro by finalizing
    /// and cleaning up a variable argument list previously initialized with
    /// `cir.va_start`.
    ///
    /// The operand must be a pointer to the target's `va_list` representation.
    /// This operation has no results and produces its effect by mutating the
    /// storage referenced by the pointer operand.
    ///
    /// `cir.va_end` must only be called after a matching `cir.va_start` on the
    /// same `va_list` along all control-flow paths. After `cir.va_end`, the
    /// `va_list` is invalid and must not be accessed unless reinitialized.
    ///
    /// Lowering typically maps this to the LLVM intrinsic `llvm.va_end`,
    /// passing the appropriately decayed pointer to the underlying `va_list`
    /// storage.
    ///
    /// Example:
    /// ```
    /// // %args : !cir.ptr<!cir.array<!rec___va_list_tag x 1>>
    /// %p = cir.cast array_to_ptrdecay %args
    ///       : !cir.ptr<!cir.array<!rec___va_list_tag x 1>>
    ///       -> !cir.ptr<!rec___va_list_tag>
    /// cir.va_end %p : !cir.ptr<!rec___va_list_tag>
    /// ```
    VaEnd(varargs::VaEnd),
    /// `cir.va_start`
    /// Starts a variable argument list
    ///
    /// The cir.va_start operation models the C/C++ va_start macro by
    /// initializing a variable argument list at the given va_list storage
    /// location.
    ///
    /// The operand must be a pointer to the target's `va_list` representation.
    /// This operation has no results and produces its effect by mutating the
    /// storage referenced by the pointer operand.
    ///
    /// Each `cir.va_start` must be paired with a corresponding `cir.va_end`
    /// on the same logical `va_list` object along all control-flow paths. After
    /// `cir.va_end`, the `va_list` must not be accessed unless reinitialized
    /// with another `cir.va_start`.
    ///
    /// Lowering maps this to the LLVM intrinsic `llvm.va_start`, passing the
    /// appropriately decayed pointer to the underlying `va_list` storage.
    ///
    /// Example:
    ///
    /// ```
    /// // %args : !cir.ptr<!cir.array<!rec___va_list_tag x 1>>
    /// %p = cir.cast array_to_ptrdecay %args
    ///       : !cir.ptr<!cir.array<!rec___va_list_tag x 1>>)
    ///       -> !cir.ptr<!rec___va_list_tag>
    /// cir.va_start %p : !cir.ptr<!rec___va_list_tag>
    /// ```
    VaStart(varargs::VaStart),
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
    /// %gt = cir.vec.cmp(gt, %va, %vb) : !cir.vector<4 x !cir.float>, !cir.vector<4 x !s32i> {
    ///   fenv = #cir.fenv<>
    /// }
    /// ```
    VecCmp(vectors::VecCmp),
    /// `cir.vec.create`
    /// Create a vector value
    ///
    /// The `cir.vec.create` operation creates a vector value with the given element
    /// values. The number of element arguments must match the number of elements
    /// in the vector type.
    VecCreate(vectors::VecCreate),
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
    VecExtract(vectors::VecExtract),
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
    VecInsert(vectors::VecInsert),
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
    VecMaskedLoad(vectors::VecMaskedLoad),
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
    VecShuffle(vectors::VecShuffle),
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
    VecShuffleDynamic(vectors::VecShuffleDynamic),
    /// `cir.vec.splat`
    /// Convert a scalar into a vector
    ///
    /// The `cir.vec.splat` operation creates a vector value from a scalar value.
    /// All elements of the vector have the same value, that of the given scalar.
    ///
    /// It's a separate operation from `cir.vec.create` because more
    /// efficient LLVM IR can be generated for it, and because some optimization and
    /// analysis passes can benefit from knowing that all elements of the vector
    /// have the same value.
    ///
    /// ```
    /// %value = cir.const #cir.int<3> : !s32i
    /// %value_vec = cir.vec.splat %value : !s32i, !cir.vector<4 x !s32i>
    /// ```
    VecSplat(vectors::VecSplat),
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
    VecTernary(vectors::VecTernary),
    /// `cir.vtable.address_point`
    /// Get the vtable (global variable) address point
    ///
    /// The `vtable.address_point` operation retrieves the "effective" address
    /// (address point) of a C++ virtual table. An object internal `__vptr`
    /// gets initializated on top of the value returned by this operation.
    ///
    /// `address_point.index` (vtable index) provides the appropriate vtable within
    /// the vtable group (as specified by Itanium ABI), and `address_point.offset`
    /// (address point index) the actual address point within that vtable.
    ///
    /// The return type is always `!cir.vptr`.
    ///
    /// Example:
    /// ```
    /// cir.global linkonce_odr @_ZTV1B = ...
    /// ...
    /// %3 = cir.vtable.address_point(@_ZTV1B,
    ///           address_point = <index = 0, offset = 2>) : !cir.vptr
    /// ```
    VtableAddressPoint(vtables::VtableAddressPoint),
    /// `cir.vtable.get_type_info`
    /// Get the address of the type_info from the vtable
    ///
    /// The `vtable.get_type_info` operation retreives the address of the dynamic
    /// type_info/rtti object from an object's vtable. This is an ABI independent
    /// abstraction of this operation.
    ///
    /// The `vptr` operand must be a `!cir.vptr` value, which would have been
    /// returned by a previous call to `cir.vtable.get_vptr`.
    ///
    /// The return type is a loadable pointer to a `type_info` struct.
    ///
    /// Example:
    /// ```
    /// %5 = cir.vtable.get_vptr %2 : !cir.ptr<!rec_A> -> !cir.ptr<!cir.vptr>
    /// %6 = cir.load align(8) %5 : !cir.ptr<!cir.vptr>, !cir.vptr
    /// %7 = cir.vtable.get_type_info %6 : !cir.vptr -> !cir.ptr<!cir.ptr<!rec_std3A3Atype_info>>
    /// %8 = cir.load align(8) %7 : !cir.ptr<!cir.ptr<!rec_std3A3Atype_info>>, !cir.ptr<!rec_std3A3Atype_info>
    ///
    /// ```
    VtableGetTypeInfo(vtables::VtableGetTypeInfo),
    /// `cir.vtable.get_virtual_fn_addr`
    /// Get a the address of a virtual function pointer
    ///
    /// The `vtable.get_virtual_fn_addr` operation retrieves the address of a
    /// virtual function pointer from an object's vtable (__vptr).
    /// This is an abstraction to perform the basic pointer arithmetic to get
    /// the address of the virtual function pointer, which can then be loaded and
    /// called.
    ///
    /// The `vptr` operand must be a `!cir.ptr<!cir.vptr>` value, which would
    /// have been returned by a previous call to `cir.vtable.get_vptr`. The
    /// `index` operand is an index of the virtual function in the vtable.
    ///
    /// The return type is a pointer-to-pointer to the function type.
    ///
    /// Example:
    /// ```
    /// %2 = cir.load %0 : !cir.ptr<!cir.ptr<!rec_C>>, !cir.ptr<!rec_C>
    /// %3 = cir.vtable.get_vptr %2 : !cir.ptr<!rec_C> -> !cir.ptr<!cir.vptr>
    /// %4 = cir.load %3 : !cir.ptr<!cir.vptr>, !cir.vptr
    /// %5 = cir.vtable.get_virtual_fn_addr %4[2] : !cir.vptr
    ///               -> !cir.ptr<!cir.ptr<!cir.func<(!cir.ptr<!rec_C>) -> !s32i>>>
    /// %6 = cir.load align(8) %5 : !cir.ptr<!cir.ptr<!cir.func<(!cir.ptr<!rec_C>)
    ///                                                              -> !s32i>>>,
    ///                             !cir.ptr<!cir.func<(!cir.ptr<!rec_C>) -> !s32i>>
    /// %7 = cir.call %6(%2) : (!cir.ptr<!cir.func<(!cir.ptr<!rec_C>) -> !s32i>>,
    ///                         !cir.ptr<!rec_C>) -> !s32i
    /// ```
    VtableGetVirtualFnAddr(vtables::VtableGetVirtualFnAddr),
    /// `cir.vtable.get_vptr`
    /// Get a the address of the vtable pointer for an object
    ///
    /// The `vtable.get_vptr` operation retrieves the address of the vptr for a
    /// C++ object. This operation requires that the object pointer points to
    /// the start of a complete object. (TODO: Describe how we get that).
    /// The vptr will always be at offset zero in the object, but this operation
    /// is more explicit about what is being retrieved than a direct bitcast.
    ///
    /// The return type is always `!cir.ptr<!cir.vptr>`.
    ///
    /// Example:
    /// ```
    /// %2 = cir.load %0 : !cir.ptr<!cir.ptr<!rec_C>>, !cir.ptr<!rec_C>
    /// %3 = cir.vtable.get_vptr %2 : !cir.ptr<!rec_C> -> !cir.ptr<!cir.vptr>
    /// ```
    VtableGetVptr(vtables::VtableGetVptr),
    /// `cir.vtt.address_point`
    /// Get the VTT address point
    ///
    /// The `vtt.address_point` operation retrieves an element from the virtual
    /// table table (VTT), which is the address point of a C++ vtable. In virtual
    /// inheritance, a set of internal `__vptr` members for an object are
    /// initialized by this operation, which assigns an element from the VTT. The
    /// initialization order is as follows:
    ///
    /// The complete object constructors and destructors find the VTT,
    /// via the mangled name of the VTT global variable. They pass the address of
    /// the subobject's sub-VTT entry in the VTT as a second parameter
    /// when calling the base object constructors and destructors.
    /// The base object constructors and destructors use the address passed to
    /// initialize the primary virtual pointer and virtual pointers that point to
    /// the classes which either have virtual bases or override virtual functions
    /// with a virtual step.
    ///
    /// The first parameter is either the mangled name of VTT global variable
    /// or the address of the subobject's sub-VTT entry in the VTT.
    /// The second parameter `offset` provides a virtual step to adjust to
    /// the actual address point of the vtable.
    ///
    /// The return type is always a `!cir.ptr<!cir.ptr<void>>`.
    ///
    /// Example:
    /// ```
    /// cir.global linkonce_odr @_ZTV1B = ...
    /// ...
    /// %3 = cir.base_class_addr(%1 : !cir.ptr<!rec_D> nonnull) [0]
    ///          -> !cir.ptr<!rec_B>
    /// %4 = cir.vtt.address_point @_ZTT1D, offset = 1
    ///          -> !cir.ptr<!cir.ptr<!void>>
    /// cir.call @_ZN1BC2Ev(%3, %4)
    /// ```
    /// Or:
    /// ```
    /// %7 = cir.vtt.address_point %3 : !cir.ptr<!cir.ptr<!void>>, offset = 1
    ///          -> !cir.ptr<!cir.ptr<!void>>
    /// ```
    VttAddressPoint(vtables::VttAddressPoint),
    /// `cir.while`
    /// C/C++ while loop
    ///
    /// Represents a C/C++ while loop. It consists of two or three regions:
    ///
    ///  - `cond`: single block region with the loop's condition. Should be
    ///  terminated with a `cir.condition` operation.
    ///  - `body`: contains the loop body and an arbitrary number of blocks.
    ///  - `cleanup`: optional region that runs on every per-iteration exit edge
    ///  (condition-false exit, end-of-iteration, break/continue, and EH unwinding
    ///  when the cleanup kind includes EH). This is used to destroy a condition
    ///  variable whose lifetime is a single iteration. When present, it carries a
    ///  cleanup kind matching `cir.cleanup.scope` (`normal` or `all`). Note that
    ///  a `DoWhileOp` cannot contain a `cleanup` region.
    ///
    /// Example:
    ///
    /// ```
    /// cir.while {
    ///   cir.condition(%cond)
    /// } do {
    ///   cir.yield
    /// }
    ///
    /// cir.while {
    ///   cir.condition(%cond)
    /// } do {
    ///   cir.yield
    /// } cleanup all {
    ///   cir.yield
    /// }
    /// ```
    While(control_flow::While),
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
    Xor(arithmetic::Xor),
    /// `cir.yield`
    /// Represents the default branching behaviour of a region
    ///
    /// The `cir.yield` operation terminates regions on different CIR operations,
    /// and it is used to represent the default branching behaviour of a region.
    /// Said branching behaviour is determinted by the parent operation. For
    /// example, a yield in a `switch-case` region implies a fallthrough, while
    /// a yield in a `cir.if` region implies a branch to the exit block, and so
    /// on.
    ///
    /// In some cases, it might yield an SSA value and the semantics of how the
    /// values are yielded is defined by the parent operation. For example, a
    /// `cir.ternary` operation yields a value from one of its regions.
    ///
    /// As a general rule, `cir.yield` must be explicitly used whenever a region has
    /// more than one block and no terminator, or within `cir.switch` regions not
    /// `cir.return` terminated.
    ///
    /// Examples:
    /// ```
    /// cir.if %4 {
    ///   ...
    ///   cir.yield
    /// }
    ///
    /// cir.switch (%5) [
    ///   case (equal, 3) {
    ///     ...
    ///     cir.yield
    ///   }, ...
    /// ]
    ///
    /// cir.scope {
    ///   ...
    ///   cir.yield
    /// }
    ///
    /// %x = cir.scope {
    ///   ...
    ///   cir.yield %val
    /// }
    ///
    /// %y = cir.ternary {
    ///   ...
    ///   cir.yield %val : i32
    /// } : i32
    /// ```
    Yield(control_flow::Yield),
    Other(crate::ast::Operation),
}
impl Op {
    /// Builds the typed operation for `op.mnemonic()`, or returns
    /// `None` when this op is not a known CIR op or its generic form
    /// doesn't match the generated schema.
    pub fn from_operation(op: &crate::ast::Operation) -> Option<Self> {
        match op.mnemonic() {
            "abs" => lower_abs(op),
            "acos" => lower_acos(op),
            "add" => lower_add(op),
            "add.overflow" => lower_add_overflow(op),
            "address_of_return_address" => lower_address_of_return_address(op),
            "alloc.exception" => lower_alloc_exception(op),
            "alloca" => lower_alloca(op),
            "and" => lower_and(op),
            "array.ctor" => lower_array_ctor(op),
            "array.dtor" => lower_array_dtor(op),
            "asin" => lower_asin(op),
            "asm" => lower_asm(op),
            "assume" => lower_assume(op),
            "atan" => lower_atan(op),
            "atan2" => lower_atan2(op),
            "atomic.clear" => lower_atomic_clear(op),
            "atomic.cmpxchg" => lower_atomic_cmpxchg(op),
            "atomic.fence" => lower_atomic_fence(op),
            "atomic.fetch" => lower_atomic_fetch(op),
            "atomic.test_and_set" => lower_atomic_test_and_set(op),
            "atomic.xchg" => lower_atomic_xchg(op),
            "await" => lower_await(op),
            "base_class_addr" => lower_base_class_addr(op),
            "base_data_member" => lower_base_data_member(op),
            "base_method" => lower_base_method(op),
            "begin_catch" => lower_begin_catch(op),
            "begin_cleanup" => lower_begin_cleanup(op),
            "bitreverse" => lower_bitreverse(op),
            "block_address" => lower_block_address(op),
            "br" => lower_br(op),
            "brcond" => lower_brcond(op),
            "break" => lower_break(op),
            "builtin_int_cast" => lower_builtin_int_cast(op),
            "byte_swap" => lower_byte_swap(op),
            "call" => lower_call(op),
            "call_llvm_intrinsic" => lower_call_llvm_intrinsic(op),
            "case" => lower_case(op),
            "cast" => lower_cast(op),
            "catch_param" => lower_catch_param(op),
            "ceil" => lower_ceil(op),
            "cleanup.scope" => lower_cleanup_scope(op),
            "clear_cache" => lower_clear_cache(op),
            "clear_padding" => lower_clear_padding(op),
            "clrsb" => lower_clrsb(op),
            "clz" => lower_clz(op),
            "cmp" => lower_cmp(op),
            "cmp3way" => lower_cmp3way(op),
            "co_return" => lower_co_return(op),
            "complex.add" => lower_complex_add(op),
            "complex.conj" => lower_complex_conj(op),
            "complex.create" => lower_complex_create(op),
            "complex.div" => lower_complex_div(op),
            "complex.imag" => lower_complex_imag(op),
            "complex.imag_ptr" => lower_complex_imag_ptr(op),
            "complex.mul" => lower_complex_mul(op),
            "complex.real" => lower_complex_real(op),
            "complex.real_ptr" => lower_complex_real_ptr(op),
            "complex.sub" => lower_complex_sub(op),
            "condition" => lower_condition(op),
            "const" => lower_const(op),
            "construct_catch_param" => lower_construct_catch_param(op),
            "continue" => lower_continue(op),
            "copy" => lower_copy(op),
            "copysign" => lower_copysign(op),
            "coro.body" => lower_coro_body(op),
            "coro.intrinsic.alloc" => lower_coro_intrinsic_alloc(op),
            "coro.intrinsic.begin" => lower_coro_intrinsic_begin(op),
            "coro.intrinsic.end" => lower_coro_intrinsic_end(op),
            "coro.intrinsic.free" => lower_coro_intrinsic_free(op),
            "coro.intrinsic.id" => lower_coro_intrinsic_id(op),
            "coro.intrinsic.size" => lower_coro_intrinsic_size(op),
            "cos" => lower_cos(op),
            "cosh" => lower_cosh(op),
            "cpuid" => lower_cpuid(op),
            "ctz" => lower_ctz(op),
            "dec" => lower_dec(op),
            "delete_array" => lower_delete_array(op),
            "derived_class_addr" => lower_derived_class_addr(op),
            "derived_data_member" => lower_derived_data_member(op),
            "derived_method" => lower_derived_method(op),
            "div" => lower_div(op),
            "do" => lower_do(op),
            "dyn_cast" => lower_dyn_cast(op),
            "eh.dispatch" => lower_eh_dispatch(op),
            "eh.inflight_exception" => lower_eh_inflight_exception(op),
            "eh.initiate" => lower_eh_initiate(op),
            "eh.longjmp" => lower_eh_longjmp(op),
            "eh.setjmp" => lower_eh_setjmp(op),
            "eh.terminate" => lower_eh_terminate(op),
            "eh.typeid" => lower_eh_typeid(op),
            "end_catch" => lower_end_catch(op),
            "end_cleanup" => lower_end_cleanup(op),
            "exp" => lower_exp(op),
            "exp10" => lower_exp10(op),
            "exp2" => lower_exp2(op),
            "expect" => lower_expect(op),
            "extract_member" => lower_extract_member(op),
            "fabs" => lower_fabs(op),
            "fadd" => lower_fadd(op),
            "fdiv" => lower_fdiv(op),
            "ffs" => lower_ffs(op),
            "floor" => lower_floor(op),
            "fma" => lower_fma(op),
            "fmaximum" => lower_fmaximum(op),
            "fmaxnum" => lower_fmaxnum(op),
            "fminimum" => lower_fminimum(op),
            "fminnum" => lower_fminnum(op),
            "fmod" => lower_fmod(op),
            "fmul" => lower_fmul(op),
            "fmuladd" => lower_fmuladd(op),
            "fneg" => lower_fneg(op),
            "for" => lower_for(op),
            "frame_address" => lower_frame_address(op),
            "freeze" => lower_freeze(op),
            "frem" => lower_frem(op),
            "frexp" => lower_frexp(op),
            "fsub" => lower_fsub(op),
            "func" => lower_func(op),
            "get_bitfield" => lower_get_bitfield(op),
            "get_element" => lower_get_element(op),
            "get_global" => lower_get_global(op),
            "get_member" => lower_get_member(op),
            "get_method" => lower_get_method(op),
            "get_runtime_member" => lower_get_runtime_member(op),
            "global" => lower_global(op),
            "goto" => lower_goto(op),
            "if" => lower_if(op),
            "inc" => lower_inc(op),
            "indirect_br" => lower_indirect_br(op),
            "indirect_goto" => lower_indirect_goto(op),
            "init_catch_param" => lower_init_catch_param(op),
            "insert_member" => lower_insert_member(op),
            "is_constant" => lower_is_constant(op),
            "is_fp_class" => lower_is_fp_class(op),
            "label" => lower_label(op),
            "launder" => lower_launder(op),
            "libc.memchr" => lower_libc_memchr(op),
            "libc.memcpy" => lower_libc_memcpy(op),
            "libc.memmove" => lower_libc_memmove(op),
            "libc.memset" => lower_libc_memset(op),
            "lifetime.end" => lower_lifetime_end(op),
            "lifetime.start" => lower_lifetime_start(op),
            "llrint" => lower_llrint(op),
            "llround" => lower_llround(op),
            "load" => lower_load(op),
            "local_init" => lower_local_init(op),
            "log" => lower_log(op),
            "log10" => lower_log10(op),
            "log2" => lower_log2(op),
            "lrint" => lower_lrint(op),
            "lround" => lower_lround(op),
            "max" => lower_max(op),
            "min" => lower_min(op),
            "minus" => lower_minus(op),
            "modf" => lower_modf(op),
            "mul" => lower_mul(op),
            "mul.overflow" => lower_mul_overflow(op),
            "nearbyint" => lower_nearbyint(op),
            "not" => lower_not(op),
            "objsize" => lower_objsize(op),
            "or" => lower_or(op),
            "parity" => lower_parity(op),
            "popcount" => lower_popcount(op),
            "pow" => lower_pow(op),
            "prefetch" => lower_prefetch(op),
            "ptr_diff" => lower_ptr_diff(op),
            "ptr_stride" => lower_ptr_stride(op),
            "rem" => lower_rem(op),
            "resume" => lower_resume(op),
            "resume.flat" => lower_resume_flat(op),
            "return" => lower_return(op),
            "return_address" => lower_return_address(op),
            "rint" => lower_rint(op),
            "rotate" => lower_rotate(op),
            "round" => lower_round(op),
            "roundeven" => lower_roundeven(op),
            "scope" => lower_scope(op),
            "select" => lower_select(op),
            "set_bitfield" => lower_set_bitfield(op),
            "shift" => lower_shift(op),
            "signbit" => lower_signbit(op),
            "sin" => lower_sin(op),
            "sinh" => lower_sinh(op),
            "sqrt" => lower_sqrt(op),
            "stackrestore" => lower_stackrestore(op),
            "stacksave" => lower_stacksave(op),
            "std.find" => lower_std_find(op),
            "std.strlen" => lower_std_strlen(op),
            "store" => lower_store(op),
            "sub" => lower_sub(op),
            "sub.overflow" => lower_sub_overflow(op),
            "switch" => lower_switch(op),
            "switch.flat" => lower_switch_flat(op),
            "tan" => lower_tan(op),
            "tanh" => lower_tanh(op),
            "ternary" => lower_ternary(op),
            "throw" => lower_throw(op),
            "token.none" => lower_token_none(op),
            "trap" => lower_trap(op),
            "trunc" => lower_trunc(op),
            "try" => lower_try(op),
            "try_call" => lower_try_call(op),
            "try_throw" => lower_try_throw(op),
            "unreachable" => lower_unreachable(op),
            "va_arg" => lower_va_arg(op),
            "va_copy" => lower_va_copy(op),
            "va_end" => lower_va_end(op),
            "va_start" => lower_va_start(op),
            "vec.cmp" => lower_vec_cmp(op),
            "vec.create" => lower_vec_create(op),
            "vec.extract" => lower_vec_extract(op),
            "vec.insert" => lower_vec_insert(op),
            "vec.masked_load" => lower_vec_masked_load(op),
            "vec.shuffle" => lower_vec_shuffle(op),
            "vec.shuffle.dynamic" => lower_vec_shuffle_dynamic(op),
            "vec.splat" => lower_vec_splat(op),
            "vec.ternary" => lower_vec_ternary(op),
            "vtable.address_point" => lower_vtable_address_point(op),
            "vtable.get_type_info" => lower_vtable_get_type_info(op),
            "vtable.get_virtual_fn_addr" => lower_vtable_get_virtual_fn_addr(op),
            "vtable.get_vptr" => lower_vtable_get_vptr(op),
            "vtt.address_point" => lower_vtt_address_point(op),
            "while" => lower_while(op),
            "xor" => lower_xor(op),
            "yield" => lower_yield(op),
            _ => None,
        }
    }
    pub fn for_each_result(&self, mut visit: impl FnMut(&ValueId, &crate::types::Type)) {
        match self {
            Op::Abs(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Acos(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Add(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::AddOverflow(value) => {
                visit(&value.result, &value.result_ty);
                visit(&value.overflow, &value.overflow_ty);
            }
            Op::AddressOfReturnAddress(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::AllocException(value) => {
                visit(&value.addr, &value.addr_ty);
            }
            Op::Alloca(value) => {
                visit(&value.addr, &value.addr_ty);
            }
            Op::And(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::ArrayCtor(_value) => {}
            Op::ArrayDtor(_value) => {}
            Op::Asin(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Asm(value) => {
                if let (Some(id), Some(ty)) = (&value.res, &value.res_ty) {
                    visit(id, ty);
                }
            }
            Op::Assume(_value) => {}
            Op::Atan(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Atan2(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::AtomicClear(_value) => {}
            Op::AtomicCmpxchg(value) => {
                visit(&value.old, &value.old_ty);
                visit(&value.success, &value.success_ty);
            }
            Op::AtomicFence(_value) => {}
            Op::AtomicFetch(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::AtomicTestAndSet(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::AtomicXchg(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Await(_value) => {}
            Op::BaseClassAddr(value) => {
                visit(&value.base_addr, &value.base_addr_ty);
            }
            Op::BaseDataMember(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::BaseMethod(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::BeginCatch(value) => {
                visit(&value.catch_token, &value.catch_token_ty);
                visit(&value.exn_ptr, &value.exn_ptr_ty);
            }
            Op::BeginCleanup(value) => {
                visit(&value.cleanup_token, &value.cleanup_token_ty);
            }
            Op::Bitreverse(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::BlockAddress(value) => {
                visit(&value.addr, &value.addr_ty);
            }
            Op::Br(_value) => {}
            Op::Brcond(_value) => {}
            Op::Break(_value) => {}
            Op::BuiltinIntCast(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::ByteSwap(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Call(value) => {
                if let (Some(id), Some(ty)) = (&value.result, &value.result_ty) {
                    visit(id, ty);
                }
            }
            Op::CallLlvmIntrinsic(value) => {
                if let (Some(id), Some(ty)) = (&value.result, &value.result_ty) {
                    visit(id, ty);
                }
            }
            Op::Case(_value) => {}
            Op::Cast(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::CatchParam(value) => {
                if let (Some(id), Some(ty)) = (&value.param, &value.param_ty) {
                    visit(id, ty);
                }
            }
            Op::Ceil(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::CleanupScope(_value) => {}
            Op::ClearCache(_value) => {}
            Op::ClearPadding(_value) => {}
            Op::Clrsb(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Clz(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Cmp(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Cmp3way(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::CoReturn(_value) => {}
            Op::ComplexAdd(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::ComplexConj(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::ComplexCreate(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::ComplexDiv(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::ComplexImag(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::ComplexImagPtr(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::ComplexMul(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::ComplexReal(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::ComplexRealPtr(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::ComplexSub(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Condition(_value) => {}
            Op::Const(value) => {
                visit(&value.res, &value.res_ty);
            }
            Op::ConstructCatchParam(_value) => {}
            Op::Continue(_value) => {}
            Op::Copy(_value) => {}
            Op::Copysign(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::CoroBody(_value) => {}
            Op::CoroIntrinsicAlloc(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::CoroIntrinsicBegin(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::CoroIntrinsicEnd(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::CoroIntrinsicFree(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::CoroIntrinsicId(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::CoroIntrinsicSize(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Cos(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Cosh(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Cpuid(_value) => {}
            Op::Ctz(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Dec(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::DeleteArray(_value) => {}
            Op::DerivedClassAddr(value) => {
                visit(&value.derived_addr, &value.derived_addr_ty);
            }
            Op::DerivedDataMember(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::DerivedMethod(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Div(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Do(_value) => {}
            Op::DynCast(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::EhDispatch(_value) => {}
            Op::EhInflightException(value) => {
                visit(&value.exception_ptr, &value.exception_ptr_ty);
                visit(&value.type_id, &value.type_id_ty);
            }
            Op::EhInitiate(value) => {
                visit(&value.eh_token, &value.eh_token_ty);
            }
            Op::EhLongjmp(_value) => {}
            Op::EhSetjmp(value) => {
                visit(&value.res, &value.res_ty);
            }
            Op::EhTerminate(_value) => {}
            Op::EhTypeid(value) => {
                visit(&value.type_id, &value.type_id_ty);
            }
            Op::EndCatch(_value) => {}
            Op::EndCleanup(_value) => {}
            Op::Exp(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Exp10(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Exp2(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Expect(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::ExtractMember(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Fabs(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Fadd(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Fdiv(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Ffs(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Floor(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Fma(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Fmaximum(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Fmaxnum(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Fminimum(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Fminnum(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Fmod(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Fmul(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Fmuladd(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Fneg(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::For(_value) => {}
            Op::FrameAddress(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Freeze(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Frem(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Frexp(value) => {
                visit(&value.result, &value.result_ty);
                visit(&value.exp, &value.exp_ty);
            }
            Op::Fsub(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Func(_value) => {}
            Op::GetBitfield(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::GetElement(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::GetGlobal(value) => {
                visit(&value.addr, &value.addr_ty);
            }
            Op::GetMember(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::GetMethod(value) => {
                visit(&value.callee, &value.callee_ty);
                visit(&value.adjusted_this, &value.adjusted_this_ty);
            }
            Op::GetRuntimeMember(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Global(_value) => {}
            Op::Goto(_value) => {}
            Op::If(_value) => {}
            Op::Inc(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::IndirectBr(_value) => {}
            Op::IndirectGoto(_value) => {}
            Op::InitCatchParam(_value) => {}
            Op::InsertMember(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::IsConstant(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::IsFpClass(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Label(_value) => {}
            Op::Launder(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::LibcMemchr(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::LibcMemcpy(_value) => {}
            Op::LibcMemmove(_value) => {}
            Op::LibcMemset(_value) => {}
            Op::LifetimeEnd(_value) => {}
            Op::LifetimeStart(_value) => {}
            Op::Llrint(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Llround(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Load(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::LocalInit(_value) => {}
            Op::Log(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Log10(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Log2(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Lrint(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Lround(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Max(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Min(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Minus(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Modf(value) => {
                visit(&value.fractional, &value.fractional_ty);
                visit(&value.integral, &value.integral_ty);
            }
            Op::Mul(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::MulOverflow(value) => {
                visit(&value.result, &value.result_ty);
                visit(&value.overflow, &value.overflow_ty);
            }
            Op::Nearbyint(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Not(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Objsize(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Or(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Parity(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Popcount(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Pow(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Prefetch(_value) => {}
            Op::PtrDiff(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::PtrStride(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Rem(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Resume(_value) => {}
            Op::ResumeFlat(_value) => {}
            Op::Return(_value) => {}
            Op::ReturnAddress(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Rint(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Rotate(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Round(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Roundeven(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Scope(value) => {
                if let (Some(id), Some(ty)) = (&value.results, &value.results_ty) {
                    visit(id, ty);
                }
            }
            Op::Select(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::SetBitfield(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Shift(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Signbit(value) => {
                visit(&value.res, &value.res_ty);
            }
            Op::Sin(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Sinh(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Sqrt(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Stackrestore(_value) => {}
            Op::Stacksave(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::StdFind(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::StdStrlen(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Store(_value) => {}
            Op::Sub(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::SubOverflow(value) => {
                visit(&value.result, &value.result_ty);
                visit(&value.overflow, &value.overflow_ty);
            }
            Op::Switch(_value) => {}
            Op::SwitchFlat(_value) => {}
            Op::Tan(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Tanh(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Ternary(value) => {
                if let (Some(id), Some(ty)) = (&value.result, &value.result_ty) {
                    visit(id, ty);
                }
            }
            Op::Throw(_value) => {}
            Op::TokenNone(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Trap(_value) => {}
            Op::Trunc(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Try(_value) => {}
            Op::TryCall(value) => {
                if let (Some(id), Some(ty)) = (&value.result, &value.result_ty) {
                    visit(id, ty);
                }
            }
            Op::TryThrow(_value) => {}
            Op::Unreachable(_value) => {}
            Op::VaArg(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::VaCopy(_value) => {}
            Op::VaEnd(_value) => {}
            Op::VaStart(_value) => {}
            Op::VecCmp(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::VecCreate(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::VecExtract(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::VecInsert(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::VecMaskedLoad(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::VecShuffle(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::VecShuffleDynamic(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::VecSplat(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::VecTernary(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::VtableAddressPoint(value) => {
                visit(&value.addr, &value.addr_ty);
            }
            Op::VtableGetTypeInfo(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::VtableGetVirtualFnAddr(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::VtableGetVptr(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::VttAddressPoint(value) => {
                visit(&value.addr, &value.addr_ty);
            }
            Op::While(_value) => {}
            Op::Xor(value) => {
                visit(&value.result, &value.result_ty);
            }
            Op::Yield(_value) => {}
            Op::Other(op) => {
                for (id, ty) in &op.results {
                    visit(id, ty);
                }
            }
        }
    }
}
fn lower_abs(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Abs(arithmetic::Abs {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            min_is_poison: unit_attr(op, "min_is_poison"),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_acos(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Acos(arithmetic::Acos {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_add(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Add(arithmetic::Add {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            no_signed_wrap: unit_attr(op, "no_signed_wrap"),
            no_unsigned_wrap: unit_attr(op, "no_unsigned_wrap"),
            saturated: unit_attr(op, "saturated"),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_add_overflow(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::AddOverflow(misc::AddOverflow {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            overflow: op.results.get(1)?.0.clone(),
            overflow_ty: op.results.get(1)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_address_of_return_address(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::AddressOfReturnAddress(misc::AddressOfReturnAddress {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_alloc_exception(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::AllocException(exceptions::AllocException {
            addr: op.results.get(0)?.0.clone(),
            addr_ty: op.results.get(0)?.1.clone(),
            size: op_attr(op, "size")?.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_alloca(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Alloca(memory::Alloca {
            addr: op.results.get(0)?.0.clone(),
            addr_ty: op.results.get(0)?.1.clone(),
            dyn_alloc_size: take_optional_operand(op, &mut __operand_index, 0),
            name: attr_str(op, "name")?,
            init: unit_attr(op, "init"),
            constant: unit_attr(op, "constant"),
            cleanup_dest_slot: unit_attr(op, "cleanup_dest_slot"),
            alignment: attr_u64(op, "alignment")?,
            annotations: op.attr("annotations").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_and(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::And(arithmetic::And {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_array_ctor(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::ArrayCtor(arrays::ArrayCtor {
            addr: take_single_operand(op, &mut __operand_index, 0)?,
            num_elements: take_optional_operand(op, &mut __operand_index, 1),
            body: lower_region(op.regions.get(0)?),
            partial_dtor: lower_region(op.regions.get(1)?),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_array_dtor(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::ArrayDtor(arrays::ArrayDtor {
            addr: take_single_operand(op, &mut __operand_index, 0)?,
            num_elements: take_optional_operand(op, &mut __operand_index, 1),
            dtor_may_throw: unit_attr(op, "dtor_may_throw"),
            body: lower_region(op.regions.get(0)?),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_asin(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Asin(arithmetic::Asin {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_asm(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Asm(misc::Asm {
            res: op.results.get(0).map(|(id, _)| id.clone()),
            res_ty: op.results.get(0).map(|(_, ty)| ty.clone()),
            asm_operands: take_variadic_of_variadic(
                op,
                &mut __operand_index,
                "operands_segments",
            )?,
            asm_string: attr_str(op, "asm_string")?,
            constraints: attr_str(op, "constraints")?,
            side_effects: unit_attr(op, "side_effects"),
            asm_flavor: crate::enums::AsmFlavor::try_from(op_attr(op, "asm_flavor")?)
                .ok()?,
            operand_attrs: op_attr(op, "operand_attrs")?.clone(),
            operands_segments: op_attr(op, "operands_segments")?.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_assume(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Assume(misc::Assume {
            predicate: take_single_operand(op, &mut __operand_index, 0)?,
            bundle_kind: op_attr(op, "bundle_kind")?.clone(),
            bundle_args: take_variadic_operand(op, &mut __operand_index, 1)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_atan(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Atan(arithmetic::Atan {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_atan2(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Atan2(arithmetic::Atan2 {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_atomic_clear(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::AtomicClear(atomics::AtomicClear {
            ptr: take_single_operand(op, &mut __operand_index, 0)?,
            mem_order: crate::enums::MemOrder::try_from(op_attr(op, "mem_order")?).ok()?,
            alignment: op.attr("alignment").cloned(),
            is_volatile: unit_attr(op, "is_volatile"),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_atomic_cmpxchg(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::AtomicCmpxchg(atomics::AtomicCmpxchg {
            old: op.results.get(0)?.0.clone(),
            old_ty: op.results.get(0)?.1.clone(),
            success: op.results.get(1)?.0.clone(),
            success_ty: op.results.get(1)?.1.clone(),
            ptr: take_single_operand(op, &mut __operand_index, 0)?,
            expected: take_single_operand(op, &mut __operand_index, 1)?,
            desired: take_single_operand(op, &mut __operand_index, 2)?,
            succ_order: crate::enums::MemOrder::try_from(op_attr(op, "succ_order")?)
                .ok()?,
            fail_order: crate::enums::MemOrder::try_from(op_attr(op, "fail_order")?)
                .ok()?,
            sync_scope: crate::enums::SyncScopeKind::try_from(op_attr(op, "sync_scope")?)
                .ok()?,
            alignment: op.attr("alignment").cloned(),
            weak: unit_attr(op, "weak"),
            is_volatile: unit_attr(op, "is_volatile"),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_atomic_fence(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::AtomicFence(atomics::AtomicFence {
            ordering: crate::enums::MemOrder::try_from(op_attr(op, "ordering")?).ok()?,
            syncscope: op
                .attr("syncscope")
                .and_then(|a| crate::enums::SyncScopeKind::try_from(a).ok()),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_atomic_fetch(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::AtomicFetch(atomics::AtomicFetch {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            ptr: take_single_operand(op, &mut __operand_index, 0)?,
            val: take_single_operand(op, &mut __operand_index, 1)?,
            binop: crate::enums::AtomicFetchKind::try_from(op_attr(op, "binop")?).ok()?,
            mem_order: crate::enums::MemOrder::try_from(op_attr(op, "mem_order")?).ok()?,
            sync_scope: crate::enums::SyncScopeKind::try_from(op_attr(op, "sync_scope")?)
                .ok()?,
            is_volatile: unit_attr(op, "is_volatile"),
            fetch_first: unit_attr(op, "fetch_first"),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_atomic_test_and_set(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::AtomicTestAndSet(atomics::AtomicTestAndSet {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            ptr: take_single_operand(op, &mut __operand_index, 0)?,
            mem_order: crate::enums::MemOrder::try_from(op_attr(op, "mem_order")?).ok()?,
            alignment: op.attr("alignment").cloned(),
            is_volatile: unit_attr(op, "is_volatile"),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_atomic_xchg(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::AtomicXchg(atomics::AtomicXchg {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            ptr: take_single_operand(op, &mut __operand_index, 0)?,
            val: take_single_operand(op, &mut __operand_index, 1)?,
            mem_order: crate::enums::MemOrder::try_from(op_attr(op, "mem_order")?).ok()?,
            sync_scope: crate::enums::SyncScopeKind::try_from(op_attr(op, "sync_scope")?)
                .ok()?,
            is_volatile: unit_attr(op, "is_volatile"),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_await(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Await(misc::Await {
            kind: crate::enums::AwaitKind::try_from(op_attr(op, "kind")?).ok()?,
            ready: lower_region(op.regions.get(0)?),
            suspend: lower_region(op.regions.get(1)?),
            resume: lower_region(op.regions.get(2)?),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_base_class_addr(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::BaseClassAddr(memory::BaseClassAddr {
            base_addr: op.results.get(0)?.0.clone(),
            base_addr_ty: op.results.get(0)?.1.clone(),
            derived_addr: take_single_operand(op, &mut __operand_index, 0)?,
            offset: op_attr(op, "offset")?.clone(),
            assume_not_null: unit_attr(op, "assume_not_null"),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_base_data_member(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::BaseDataMember(memory::BaseDataMember {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            offset: op_attr(op, "offset")?.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_base_method(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::BaseMethod(memory::BaseMethod {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            offset: op_attr(op, "offset")?.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_begin_catch(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::BeginCatch(exceptions::BeginCatch {
            catch_token: op.results.get(0)?.0.clone(),
            catch_token_ty: op.results.get(0)?.1.clone(),
            exn_ptr: op.results.get(1)?.0.clone(),
            exn_ptr_ty: op.results.get(1)?.1.clone(),
            eh_token: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_begin_cleanup(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::BeginCleanup(exceptions::BeginCleanup {
            cleanup_token: op.results.get(0)?.0.clone(),
            cleanup_token_ty: op.results.get(0)?.1.clone(),
            eh_token: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_bitreverse(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Bitreverse(misc::Bitreverse {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            input: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_block_address(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::BlockAddress(misc::BlockAddress {
            addr: op.results.get(0)?.0.clone(),
            addr_ty: op.results.get(0)?.1.clone(),
            block_addr_info: op_attr(op, "block_addr_info")?.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_br(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Br(control_flow::Br {
            dest_operands: take_variadic_operand(op, &mut __operand_index, 0)?,
            successors: op.successors.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_brcond(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Brcond(control_flow::Brcond {
            cond: take_single_operand(op, &mut __operand_index, 0)?,
            dest_operands_true: take_variadic_operand(op, &mut __operand_index, 1)?,
            dest_operands_false: take_variadic_operand(op, &mut __operand_index, 2)?,
            successors: op.successors.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_break(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Break(control_flow::Break {
            loc: op.loc.clone(),
        }),
    )
}
fn lower_builtin_int_cast(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::BuiltinIntCast(misc::BuiltinIntCast {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_byte_swap(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::ByteSwap(arithmetic::ByteSwap {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            input: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_call(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Call(calls::Call {
            result: op.results.get(0).map(|(id, _)| id.clone()),
            result_ty: op.results.get(0).map(|(_, ty)| ty.clone()),
            callee: op.attr("callee").cloned(),
            args: if op.attr("callee").is_some() {
                take_variadic_operand(op, &mut __operand_index, 0)?
            } else {
                let args = op.operands.get(1..).unwrap_or(&[]).to_vec();
                __operand_index = op.operands.len();
                args
            },
            nothrow: unit_attr(op, "nothrow"),
            inline_kind: op
                .attr("inline_kind")
                .and_then(|a| crate::enums::InlineKind::try_from(a).ok()),
            musttail: unit_attr(op, "musttail"),
            side_effect: op_attr(op, "side_effect")?.clone(),
            arg_attrs: op.attr("arg_attrs").cloned(),
            res_attrs: op.attr("res_attrs").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_call_llvm_intrinsic(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::CallLlvmIntrinsic(calls::CallLlvmIntrinsic {
            result: op.results.get(0).map(|(id, _)| id.clone()),
            result_ty: op.results.get(0).map(|(_, ty)| ty.clone()),
            intrinsic_name: attr_str(op, "intrinsic_name")?,
            arg_ops: take_variadic_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_case(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Case(control_flow::Case {
            value: op_attr(op, "value")?.clone(),
            kind: crate::enums::CaseOpKind::try_from(op_attr(op, "kind")?).ok()?,
            case_region: lower_region(op.regions.get(0)?),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_cast(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Cast(misc::Cast {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            kind: crate::enums::CastKind::try_from(op_attr(op, "kind")?).ok()?,
            src: take_single_operand(op, &mut __operand_index, 0)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_catch_param(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::CatchParam(exceptions::CatchParam {
            param: op.results.get(0).map(|(id, _)| id.clone()),
            param_ty: op.results.get(0).map(|(_, ty)| ty.clone()),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_ceil(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Ceil(arithmetic::Ceil {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_cleanup_scope(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::CleanupScope(control_flow::CleanupScope {
            cleanup_kind: crate::enums::CleanupKind::try_from(
                    op_attr(op, "cleanupKind")?,
                )
                .ok()?,
            body_region: lower_region(op.regions.get(0)?),
            cleanup_region: lower_region(op.regions.get(1)?),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_clear_cache(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::ClearCache(memory::ClearCache {
            begin: take_single_operand(op, &mut __operand_index, 0)?,
            end: take_single_operand(op, &mut __operand_index, 1)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_clear_padding(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::ClearPadding(memory::ClearPadding {
            arg: take_single_operand(op, &mut __operand_index, 0)?,
            alignment: attr_u64(op, "alignment")?,
            padding: op_attr(op, "padding")?.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_clrsb(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Clrsb(misc::Clrsb {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            input: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_clz(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Clz(misc::Clz {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            input: take_single_operand(op, &mut __operand_index, 0)?,
            poison_zero: unit_attr(op, "poison_zero"),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_cmp(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Cmp(arithmetic::Cmp {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            kind: crate::enums::CmpOpKind::try_from(op_attr(op, "kind")?).ok()?,
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_cmp3way(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Cmp3way(arithmetic::Cmp3way {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            info: op_attr(op, "info")?.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_co_return(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::CoReturn(misc::CoReturn {
            loc: op.loc.clone(),
        }),
    )
}
fn lower_complex_add(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::ComplexAdd(complex::ComplexAdd {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_complex_conj(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::ComplexConj(complex::ComplexConj {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            operand: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_complex_create(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::ComplexCreate(complex::ComplexCreate {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            real: take_single_operand(op, &mut __operand_index, 0)?,
            imag: take_single_operand(op, &mut __operand_index, 1)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_complex_div(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::ComplexDiv(complex::ComplexDiv {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            range: crate::enums::ComplexRangeKind::try_from(op_attr(op, "range")?).ok()?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_complex_imag(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::ComplexImag(complex::ComplexImag {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            operand: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_complex_imag_ptr(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::ComplexImagPtr(complex::ComplexImagPtr {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            operand: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_complex_mul(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::ComplexMul(complex::ComplexMul {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            range: crate::enums::ComplexRangeKind::try_from(op_attr(op, "range")?).ok()?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_complex_real(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::ComplexReal(complex::ComplexReal {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            operand: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_complex_real_ptr(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::ComplexRealPtr(complex::ComplexRealPtr {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            operand: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_complex_sub(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::ComplexSub(complex::ComplexSub {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_condition(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Condition(control_flow::Condition {
            condition: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_const(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Const(globals::Const {
            res: op.results.get(0)?.0.clone(),
            res_ty: op.results.get(0)?.1.clone(),
            value: op_attr(op, "value")?.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_construct_catch_param(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::ConstructCatchParam(exceptions::ConstructCatchParam {
            eh_token: take_single_operand(op, &mut __operand_index, 0)?,
            param_addr: take_single_operand(op, &mut __operand_index, 1)?,
            kind: crate::enums::InitCatchKind::try_from(op_attr(op, "kind")?).ok()?,
            copy_fn: op.attr("copy_fn").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_continue(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Continue(control_flow::Continue {
            loc: op.loc.clone(),
        }),
    )
}
fn lower_copy(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Copy(memory::Copy {
            dst: take_single_operand(op, &mut __operand_index, 0)?,
            src: take_single_operand(op, &mut __operand_index, 1)?,
            dst_alignment: op.attr("dst_alignment").cloned(),
            src_alignment: op.attr("src_alignment").cloned(),
            is_volatile: unit_attr(op, "is_volatile"),
            skip_tail_padding: unit_attr(op, "skip_tail_padding"),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_copysign(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Copysign(arithmetic::Copysign {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_coro_body(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::CoroBody(misc::CoroBody {
            body: lower_region(op.regions.get(0)?),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_coro_intrinsic_alloc(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::CoroIntrinsicAlloc(misc::CoroIntrinsicAlloc {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            id: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_coro_intrinsic_begin(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::CoroIntrinsicBegin(misc::CoroIntrinsicBegin {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            id: take_single_operand(op, &mut __operand_index, 0)?,
            coroframe_addr: take_single_operand(op, &mut __operand_index, 1)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_coro_intrinsic_end(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::CoroIntrinsicEnd(misc::CoroIntrinsicEnd {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            handle: take_single_operand(op, &mut __operand_index, 0)?,
            unwind: take_single_operand(op, &mut __operand_index, 1)?,
            result_token: take_single_operand(op, &mut __operand_index, 2)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_coro_intrinsic_free(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::CoroIntrinsicFree(misc::CoroIntrinsicFree {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            id: take_single_operand(op, &mut __operand_index, 0)?,
            coroframe: take_single_operand(op, &mut __operand_index, 1)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_coro_intrinsic_id(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::CoroIntrinsicId(misc::CoroIntrinsicId {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            align: take_single_operand(op, &mut __operand_index, 0)?,
            promise: take_single_operand(op, &mut __operand_index, 1)?,
            coroaddr: take_single_operand(op, &mut __operand_index, 2)?,
            fnaddrs: take_single_operand(op, &mut __operand_index, 3)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_coro_intrinsic_size(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::CoroIntrinsicSize(misc::CoroIntrinsicSize {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_cos(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Cos(arithmetic::Cos {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_cosh(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Cosh(arithmetic::Cosh {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_cpuid(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Cpuid(misc::Cpuid {
            cpu_info: take_single_operand(op, &mut __operand_index, 0)?,
            function_id: take_single_operand(op, &mut __operand_index, 1)?,
            sub_function_id: take_single_operand(op, &mut __operand_index, 2)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_ctz(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Ctz(misc::Ctz {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            input: take_single_operand(op, &mut __operand_index, 0)?,
            poison_zero: unit_attr(op, "poison_zero"),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_dec(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Dec(misc::Dec {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            input: take_single_operand(op, &mut __operand_index, 0)?,
            no_signed_wrap: unit_attr(op, "no_signed_wrap"),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_delete_array(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::DeleteArray(misc::DeleteArray {
            address: take_single_operand(op, &mut __operand_index, 0)?,
            delete_fn: op_attr(op, "delete_fn")?.clone(),
            delete_params: op_attr(op, "delete_params")?.clone(),
            element_dtor: op.attr("element_dtor").cloned(),
            dtor_may_throw: unit_attr(op, "dtor_may_throw"),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_derived_class_addr(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::DerivedClassAddr(memory::DerivedClassAddr {
            derived_addr: op.results.get(0)?.0.clone(),
            derived_addr_ty: op.results.get(0)?.1.clone(),
            base_addr: take_single_operand(op, &mut __operand_index, 0)?,
            offset: op_attr(op, "offset")?.clone(),
            assume_not_null: unit_attr(op, "assume_not_null"),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_derived_data_member(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::DerivedDataMember(memory::DerivedDataMember {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            offset: op_attr(op, "offset")?.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_derived_method(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::DerivedMethod(memory::DerivedMethod {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            offset: op_attr(op, "offset")?.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_div(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Div(arithmetic::Div {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_do(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Do(control_flow::Do {
            body: lower_region(op.regions.get(0)?),
            cond: lower_region(op.regions.get(1)?),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_dyn_cast(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::DynCast(misc::DynCast {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            kind: crate::enums::DynamicCastKind::try_from(op_attr(op, "kind")?).ok()?,
            src: take_single_operand(op, &mut __operand_index, 0)?,
            info: op.attr("info").cloned(),
            relative_layout: unit_attr(op, "relative_layout"),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_eh_dispatch(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::EhDispatch(exceptions::EhDispatch {
            eh_token: take_single_operand(op, &mut __operand_index, 0)?,
            catch_types: op.attr("catch_types").cloned(),
            default_is_catch_all: unit_attr(op, "default_is_catch_all"),
            successors: op.successors.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_eh_inflight_exception(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::EhInflightException(exceptions::EhInflightException {
            exception_ptr: op.results.get(0)?.0.clone(),
            exception_ptr_ty: op.results.get(0)?.1.clone(),
            type_id: op.results.get(1)?.0.clone(),
            type_id_ty: op.results.get(1)?.1.clone(),
            cleanup: unit_attr(op, "cleanup"),
            catch_all: unit_attr(op, "catch_all"),
            catch_type_list: op.attr("catch_type_list").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_eh_initiate(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::EhInitiate(exceptions::EhInitiate {
            eh_token: op.results.get(0)?.0.clone(),
            eh_token_ty: op.results.get(0)?.1.clone(),
            cleanup: unit_attr(op, "cleanup"),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_eh_longjmp(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::EhLongjmp(exceptions::EhLongjmp {
            env: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_eh_setjmp(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::EhSetjmp(exceptions::EhSetjmp {
            res: op.results.get(0)?.0.clone(),
            res_ty: op.results.get(0)?.1.clone(),
            env: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_eh_terminate(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::EhTerminate(exceptions::EhTerminate {
            eh_token: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_eh_typeid(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::EhTypeid(exceptions::EhTypeid {
            type_id: op.results.get(0)?.0.clone(),
            type_id_ty: op.results.get(0)?.1.clone(),
            type_sym: op_attr(op, "type_sym")?.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_end_catch(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::EndCatch(exceptions::EndCatch {
            catch_token: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_end_cleanup(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::EndCleanup(exceptions::EndCleanup {
            cleanup_token: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_exp(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Exp(arithmetic::Exp {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_exp10(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Exp10(arithmetic::Exp10 {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_exp2(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Exp2(arithmetic::Exp2 {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_expect(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Expect(misc::Expect {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            val: take_single_operand(op, &mut __operand_index, 0)?,
            expected: take_single_operand(op, &mut __operand_index, 1)?,
            prob: op.attr("prob").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_extract_member(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::ExtractMember(memory::ExtractMember {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            record: take_single_operand(op, &mut __operand_index, 0)?,
            index: op_attr(op, "index")?.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_fabs(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Fabs(arithmetic::Fabs {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_fadd(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Fadd(arithmetic::Fadd {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_fdiv(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Fdiv(arithmetic::Fdiv {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_ffs(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Ffs(misc::Ffs {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            input: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_floor(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Floor(arithmetic::Floor {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_fma(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Fma(arithmetic::Fma {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            a: take_single_operand(op, &mut __operand_index, 0)?,
            b: take_single_operand(op, &mut __operand_index, 1)?,
            c: take_single_operand(op, &mut __operand_index, 2)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_fmaximum(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Fmaximum(arithmetic::Fmaximum {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_fmaxnum(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Fmaxnum(arithmetic::Fmaxnum {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_fminimum(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Fminimum(arithmetic::Fminimum {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_fminnum(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Fminnum(arithmetic::Fminnum {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_fmod(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Fmod(arithmetic::Fmod {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_fmul(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Fmul(arithmetic::Fmul {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_fmuladd(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Fmuladd(arithmetic::Fmuladd {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            a: take_single_operand(op, &mut __operand_index, 0)?,
            b: take_single_operand(op, &mut __operand_index, 1)?,
            c: take_single_operand(op, &mut __operand_index, 2)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_fneg(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Fneg(arithmetic::Fneg {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            input: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_for(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::For(control_flow::For {
            cleanup_kind: op
                .attr("cleanupKind")
                .and_then(|a| crate::enums::CleanupKind::try_from(a).ok()),
            cond: lower_region(op.regions.get(0)?),
            body: lower_region(op.regions.get(1)?),
            step: lower_region(op.regions.get(2)?),
            cleanup: lower_region(op.regions.get(3)?),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_frame_address(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::FrameAddress(misc::FrameAddress {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            level: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_freeze(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Freeze(arithmetic::Freeze {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            input: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_frem(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Frem(arithmetic::Frem {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_frexp(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Frexp(arithmetic::Frexp {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            exp: op.results.get(1)?.0.clone(),
            exp_ty: op.results.get(1)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_fsub(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Fsub(arithmetic::Fsub {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_func(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Func(globals::Func {
            sym_name: attr_str(op, "sym_name")?,
            global_visibility: unit_attr(op, "global_visibility"),
            function_type: attr_type(op, "function_type")?,
            builtin: unit_attr(op, "builtin"),
            coroutine: unit_attr(op, "coroutine"),
            inline_kind: op
                .attr("inline_kind")
                .and_then(|a| crate::enums::InlineKind::try_from(a).ok()),
            lambda: unit_attr(op, "lambda"),
            no_proto: unit_attr(op, "no_proto"),
            dso_local: unit_attr(op, "dso_local"),
            linkage: op_attr(op, "linkage")?.clone(),
            calling_conv: op_attr(op, "calling_conv")?.clone(),
            sym_visibility: op
                .attr("sym_visibility")
                .and_then(|a| a.as_str().map(str::to_string)),
            comdat: unit_attr(op, "comdat"),
            arg_attrs: op.attr("arg_attrs").cloned(),
            res_attrs: op.attr("res_attrs").cloned(),
            aliasee: op.attr("aliasee").cloned(),
            side_effect: op
                .attr("side_effect")
                .and_then(|a| crate::enums::SideEffect::try_from(a).ok()),
            personality: op.attr("personality").cloned(),
            global_ctor_priority: op.attr("global_ctor_priority").cloned(),
            global_dtor_priority: op.attr("global_dtor_priority").cloned(),
            func_info: op.attr("func_info").cloned(),
            annotations: op.attr("annotations").cloned(),
            body: lower_region(op.regions.get(0)?),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_get_bitfield(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::GetBitfield(memory::GetBitfield {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            addr: take_single_operand(op, &mut __operand_index, 0)?,
            bitfield_info: op_attr(op, "bitfield_info")?.clone(),
            alignment: op_attr(op, "alignment")?.clone(),
            is_volatile: unit_attr(op, "is_volatile"),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_get_element(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::GetElement(memory::GetElement {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            base: take_single_operand(op, &mut __operand_index, 0)?,
            index: take_single_operand(op, &mut __operand_index, 1)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_get_global(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::GetGlobal(globals::GetGlobal {
            addr: op.results.get(0)?.0.clone(),
            addr_ty: op.results.get(0)?.1.clone(),
            name: op_attr(op, "name")?.clone(),
            tls: unit_attr(op, "tls"),
            static_local: unit_attr(op, "static_local"),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_get_member(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::GetMember(memory::GetMember {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            addr: take_single_operand(op, &mut __operand_index, 0)?,
            name: attr_str(op, "name")?,
            index_attr: op_attr(op, "index_attr")?.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_get_method(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::GetMethod(memory::GetMethod {
            callee: op.results.get(0)?.0.clone(),
            callee_ty: op.results.get(0)?.1.clone(),
            adjusted_this: op.results.get(1)?.0.clone(),
            adjusted_this_ty: op.results.get(1)?.1.clone(),
            method: take_single_operand(op, &mut __operand_index, 0)?,
            object: take_single_operand(op, &mut __operand_index, 1)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_get_runtime_member(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::GetRuntimeMember(memory::GetRuntimeMember {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            addr: take_single_operand(op, &mut __operand_index, 0)?,
            member: take_single_operand(op, &mut __operand_index, 1)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_global(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Global(globals::Global {
            sym_name: attr_str(op, "sym_name")?,
            global_visibility: unit_attr(op, "global_visibility"),
            sym_visibility: op
                .attr("sym_visibility")
                .and_then(|a| a.as_str().map(str::to_string)),
            sym_type: attr_type(op, "sym_type")?,
            linkage: crate::enums::GlobalLinkageKind::try_from(op_attr(op, "linkage")?)
                .ok()?,
            addr_space: op.attr("addr_space").cloned(),
            tls_model: op
                .attr("tls_model")
                .and_then(|a| crate::enums::TlsModel::try_from(a).ok()),
            tls_refs: op.attr("tls_refs").cloned(),
            initial_value: op.attr("initial_value").cloned(),
            comdat: unit_attr(op, "comdat"),
            constant: unit_attr(op, "constant"),
            dso_local: unit_attr(op, "dso_local"),
            static_local_guard: op.attr("static_local_guard").cloned(),
            alignment: op.attr("alignment").cloned(),
            ast: op.attr("ast").cloned(),
            section: op.attr("section").and_then(|a| a.as_str().map(str::to_string)),
            annotations: op.attr("annotations").cloned(),
            aliasee: op.attr("aliasee").cloned(),
            strictfp: unit_attr(op, "strictfp"),
            init_priority: op.attr("init_priority").cloned(),
            ctor_region: lower_region(op.regions.get(0)?),
            dtor_region: lower_region(op.regions.get(1)?),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_goto(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Goto(control_flow::Goto {
            label: attr_str(op, "label")?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_if(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::If(control_flow::If {
            condition: take_single_operand(op, &mut __operand_index, 0)?,
            then_region: lower_region(op.regions.get(0)?),
            else_region: lower_region(op.regions.get(1)?),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_inc(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Inc(misc::Inc {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            input: take_single_operand(op, &mut __operand_index, 0)?,
            no_signed_wrap: unit_attr(op, "no_signed_wrap"),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_indirect_br(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::IndirectBr(misc::IndirectBr {
            addr: take_single_operand(op, &mut __operand_index, 0)?,
            poison: unit_attr(op, "poison"),
            succ_operands: take_variadic_of_variadic(
                op,
                &mut __operand_index,
                "operand_segments",
            )?,
            operand_segments: op_attr(op, "operand_segments")?.clone(),
            successors: op.successors.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_indirect_goto(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::IndirectGoto(control_flow::IndirectGoto {
            addr: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_init_catch_param(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::InitCatchParam(exceptions::InitCatchParam {
            exn_ptr: take_single_operand(op, &mut __operand_index, 0)?,
            param_addr: take_single_operand(op, &mut __operand_index, 1)?,
            kind: crate::enums::InitCatchKind::try_from(op_attr(op, "kind")?).ok()?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_insert_member(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::InsertMember(memory::InsertMember {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            record: take_single_operand(op, &mut __operand_index, 0)?,
            index: op_attr(op, "index")?.clone(),
            value: take_single_operand(op, &mut __operand_index, 1)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_is_constant(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::IsConstant(arithmetic::IsConstant {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            val: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_is_fp_class(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::IsFpClass(arithmetic::IsFpClass {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            flags: crate::enums::FpClassTest(attr_u64(op, "flags")?),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_label(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Label(control_flow::Label {
            label: attr_str(op, "label")?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_launder(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Launder(misc::Launder {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            arg: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_libc_memchr(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::LibcMemchr(stdlib::LibcMemchr {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            pattern: take_single_operand(op, &mut __operand_index, 1)?,
            len: take_single_operand(op, &mut __operand_index, 2)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_libc_memcpy(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::LibcMemcpy(stdlib::LibcMemcpy {
            dst: take_single_operand(op, &mut __operand_index, 0)?,
            src: take_single_operand(op, &mut __operand_index, 1)?,
            len: take_single_operand(op, &mut __operand_index, 2)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_libc_memmove(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::LibcMemmove(stdlib::LibcMemmove {
            dst: take_single_operand(op, &mut __operand_index, 0)?,
            src: take_single_operand(op, &mut __operand_index, 1)?,
            len: take_single_operand(op, &mut __operand_index, 2)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_libc_memset(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::LibcMemset(stdlib::LibcMemset {
            dst: take_single_operand(op, &mut __operand_index, 0)?,
            alignment: op.attr("alignment").cloned(),
            val: take_single_operand(op, &mut __operand_index, 1)?,
            len: take_single_operand(op, &mut __operand_index, 2)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_lifetime_end(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::LifetimeEnd(memory::LifetimeEnd {
            ptr: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_lifetime_start(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::LifetimeStart(memory::LifetimeStart {
            ptr: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_llrint(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Llrint(arithmetic::Llrint {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_llround(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Llround(arithmetic::Llround {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_load(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Load(memory::Load {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            addr: take_single_operand(op, &mut __operand_index, 0)?,
            is_deref: unit_attr(op, "isDeref"),
            is_volatile: unit_attr(op, "is_volatile"),
            is_nontemporal: unit_attr(op, "is_nontemporal"),
            alignment: op.attr("alignment").cloned(),
            sync_scope: op
                .attr("sync_scope")
                .and_then(|a| crate::enums::SyncScopeKind::try_from(a).ok()),
            mem_order: op
                .attr("mem_order")
                .and_then(|a| crate::enums::MemOrder::try_from(a).ok()),
            invariant: unit_attr(op, "invariant"),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_local_init(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::LocalInit(globals::LocalInit {
            global_name: op_attr(op, "globalName")?.clone(),
            tls: unit_attr(op, "tls"),
            ctor_region: lower_region(op.regions.get(0)?),
            dtor_region: lower_region(op.regions.get(1)?),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_log(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Log(arithmetic::Log {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_log10(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Log10(arithmetic::Log10 {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_log2(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Log2(arithmetic::Log2 {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_lrint(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Lrint(arithmetic::Lrint {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_lround(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Lround(arithmetic::Lround {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_max(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Max(arithmetic::Max {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_min(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Min(arithmetic::Min {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_minus(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Minus(misc::Minus {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            input: take_single_operand(op, &mut __operand_index, 0)?,
            no_signed_wrap: unit_attr(op, "no_signed_wrap"),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_modf(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Modf(arithmetic::Modf {
            fractional: op.results.get(0)?.0.clone(),
            fractional_ty: op.results.get(0)?.1.clone(),
            integral: op.results.get(1)?.0.clone(),
            integral_ty: op.results.get(1)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_mul(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Mul(arithmetic::Mul {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            no_signed_wrap: unit_attr(op, "no_signed_wrap"),
            no_unsigned_wrap: unit_attr(op, "no_unsigned_wrap"),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_mul_overflow(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::MulOverflow(misc::MulOverflow {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            overflow: op.results.get(1)?.0.clone(),
            overflow_ty: op.results.get(1)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_nearbyint(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Nearbyint(arithmetic::Nearbyint {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_not(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Not(misc::Not {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            input: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_objsize(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Objsize(arithmetic::Objsize {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            ptr: take_single_operand(op, &mut __operand_index, 0)?,
            min: unit_attr(op, "min"),
            nullunknown: unit_attr(op, "nullunknown"),
            dynamic: unit_attr(op, "dynamic"),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_or(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Or(arithmetic::Or {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_parity(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Parity(misc::Parity {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            input: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_popcount(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Popcount(misc::Popcount {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            input: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_pow(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Pow(arithmetic::Pow {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_prefetch(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Prefetch(misc::Prefetch {
            addr: take_single_operand(op, &mut __operand_index, 0)?,
            locality: op_attr(op, "locality")?.clone(),
            is_write: unit_attr(op, "isWrite"),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_ptr_diff(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::PtrDiff(memory::PtrDiff {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_ptr_stride(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::PtrStride(memory::PtrStride {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            base: take_single_operand(op, &mut __operand_index, 0)?,
            stride: take_single_operand(op, &mut __operand_index, 1)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_rem(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Rem(arithmetic::Rem {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_resume(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Resume(exceptions::Resume {
            eh_token: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_resume_flat(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::ResumeFlat(exceptions::ResumeFlat {
            exception_ptr: take_single_operand(op, &mut __operand_index, 0)?,
            type_id: take_single_operand(op, &mut __operand_index, 1)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_return(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Return(control_flow::Return {
            input: take_variadic_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_return_address(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::ReturnAddress(misc::ReturnAddress {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            level: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_rint(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Rint(arithmetic::Rint {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_rotate(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Rotate(arithmetic::Rotate {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            input: take_single_operand(op, &mut __operand_index, 0)?,
            amount: take_single_operand(op, &mut __operand_index, 1)?,
            rotate_left: unit_attr(op, "rotateLeft"),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_round(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Round(arithmetic::Round {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_roundeven(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Roundeven(arithmetic::Roundeven {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_scope(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Scope(control_flow::Scope {
            results: op.results.get(0).map(|(id, _)| id.clone()),
            results_ty: op.results.get(0).map(|(_, ty)| ty.clone()),
            scope_region: lower_region(op.regions.get(0)?),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_select(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Select(arithmetic::Select {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            condition: take_single_operand(op, &mut __operand_index, 0)?,
            true_value: take_single_operand(op, &mut __operand_index, 1)?,
            false_value: take_single_operand(op, &mut __operand_index, 2)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_set_bitfield(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::SetBitfield(memory::SetBitfield {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            addr: take_single_operand(op, &mut __operand_index, 0)?,
            src: take_single_operand(op, &mut __operand_index, 1)?,
            bitfield_info: op_attr(op, "bitfield_info")?.clone(),
            alignment: op_attr(op, "alignment")?.clone(),
            is_volatile: unit_attr(op, "is_volatile"),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_shift(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Shift(arithmetic::Shift {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            value: take_single_operand(op, &mut __operand_index, 0)?,
            amount: take_single_operand(op, &mut __operand_index, 1)?,
            is_shiftleft: unit_attr(op, "isShiftleft"),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_signbit(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Signbit(misc::Signbit {
            res: op.results.get(0)?.0.clone(),
            res_ty: op.results.get(0)?.1.clone(),
            input: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_sin(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Sin(arithmetic::Sin {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_sinh(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Sinh(arithmetic::Sinh {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_sqrt(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Sqrt(arithmetic::Sqrt {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_stackrestore(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Stackrestore(misc::Stackrestore {
            ptr: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_stacksave(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Stacksave(misc::Stacksave {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_std_find(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::StdFind(stdlib::StdFind {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            first: take_single_operand(op, &mut __operand_index, 0)?,
            last: take_single_operand(op, &mut __operand_index, 1)?,
            pattern: take_single_operand(op, &mut __operand_index, 2)?,
            original_fn: op_attr(op, "original_fn")?.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_std_strlen(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::StdStrlen(stdlib::StdStrlen {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            string: take_single_operand(op, &mut __operand_index, 0)?,
            original_fn: op_attr(op, "original_fn")?.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_store(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Store(memory::Store {
            value: take_single_operand(op, &mut __operand_index, 0)?,
            addr: take_single_operand(op, &mut __operand_index, 1)?,
            is_volatile: unit_attr(op, "is_volatile"),
            is_nontemporal: unit_attr(op, "is_nontemporal"),
            alignment: op.attr("alignment").cloned(),
            sync_scope: op
                .attr("sync_scope")
                .and_then(|a| crate::enums::SyncScopeKind::try_from(a).ok()),
            mem_order: op
                .attr("mem_order")
                .and_then(|a| crate::enums::MemOrder::try_from(a).ok()),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_sub(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Sub(arithmetic::Sub {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            no_signed_wrap: unit_attr(op, "no_signed_wrap"),
            no_unsigned_wrap: unit_attr(op, "no_unsigned_wrap"),
            saturated: unit_attr(op, "saturated"),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_sub_overflow(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::SubOverflow(misc::SubOverflow {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            overflow: op.results.get(1)?.0.clone(),
            overflow_ty: op.results.get(1)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_switch(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Switch(control_flow::Switch {
            condition: take_single_operand(op, &mut __operand_index, 0)?,
            all_enum_cases_covered: unit_attr(op, "all_enum_cases_covered"),
            body: lower_region(op.regions.get(0)?),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_switch_flat(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::SwitchFlat(control_flow::SwitchFlat {
            condition: take_single_operand(op, &mut __operand_index, 0)?,
            default_operands: take_variadic_operand(op, &mut __operand_index, 1)?,
            case_operands: take_variadic_of_variadic(
                op,
                &mut __operand_index,
                "operand_segments",
            )?,
            case_values: op_attr(op, "caseValues")?.clone(),
            case_operand_segments: op_attr(op, "case_operand_segments")?.clone(),
            successors: op.successors.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_tan(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Tan(arithmetic::Tan {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_tanh(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Tanh(arithmetic::Tanh {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_ternary(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Ternary(control_flow::Ternary {
            result: op.results.get(0).map(|(id, _)| id.clone()),
            result_ty: op.results.get(0).map(|(_, ty)| ty.clone()),
            cond: take_single_operand(op, &mut __operand_index, 0)?,
            true_region: lower_region(op.regions.get(0)?),
            false_region: lower_region(op.regions.get(1)?),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_throw(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Throw(exceptions::Throw {
            exception_ptr: take_optional_operand(op, &mut __operand_index, 0),
            type_info: op.attr("type_info").cloned(),
            dtor: op.attr("dtor").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_token_none(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::TokenNone(exceptions::TokenNone {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_trap(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Trap(control_flow::Trap {
            loc: op.loc.clone(),
        }),
    )
}
fn lower_trunc(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Trunc(arithmetic::Trunc {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_try(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Try(exceptions::Try {
            cleanup: unit_attr(op, "cleanup"),
            handler_types: op_attr(op, "handler_types")?.clone(),
            try_region: lower_region(op.regions.get(0)?),
            handler_regions: op.regions.iter().skip(1).map(lower_region).collect(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_try_call(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::TryCall(misc::TryCall {
            result: op.results.get(0).map(|(id, _)| id.clone()),
            result_ty: op.results.get(0).map(|(_, ty)| ty.clone()),
            callee: op.attr("callee").cloned(),
            args: take_variadic_operand(op, &mut __operand_index, 0)?,
            nothrow: unit_attr(op, "nothrow"),
            inline_kind: op
                .attr("inline_kind")
                .and_then(|a| crate::enums::InlineKind::try_from(a).ok()),
            musttail: unit_attr(op, "musttail"),
            side_effect: op_attr(op, "side_effect")?.clone(),
            arg_attrs: op.attr("arg_attrs").cloned(),
            res_attrs: op.attr("res_attrs").cloned(),
            successors: op.successors.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_try_throw(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::TryThrow(exceptions::TryThrow {
            exception_ptr: take_optional_operand(op, &mut __operand_index, 0),
            type_info: op.attr("type_info").cloned(),
            dtor: op.attr("dtor").cloned(),
            successors: op.successors.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_unreachable(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Unreachable(control_flow::Unreachable {
            loc: op.loc.clone(),
        }),
    )
}
fn lower_va_arg(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::VaArg(varargs::VaArg {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            arg_list: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_va_copy(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::VaCopy(varargs::VaCopy {
            dst_list: take_single_operand(op, &mut __operand_index, 0)?,
            src_list: take_single_operand(op, &mut __operand_index, 1)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_va_end(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::VaEnd(varargs::VaEnd {
            arg_list: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_va_start(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::VaStart(varargs::VaStart {
            arg_list: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_vec_cmp(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::VecCmp(vectors::VecCmp {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            kind: crate::enums::CmpOpKind::try_from(op_attr(op, "kind")?).ok()?,
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            fenv: op.attr("fenv").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_vec_create(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::VecCreate(vectors::VecCreate {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            elements: take_variadic_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_vec_extract(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::VecExtract(vectors::VecExtract {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            vec: take_single_operand(op, &mut __operand_index, 0)?,
            index: take_single_operand(op, &mut __operand_index, 1)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_vec_insert(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::VecInsert(vectors::VecInsert {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            vec: take_single_operand(op, &mut __operand_index, 0)?,
            value: take_single_operand(op, &mut __operand_index, 1)?,
            index: take_single_operand(op, &mut __operand_index, 2)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_vec_masked_load(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::VecMaskedLoad(vectors::VecMaskedLoad {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            addr: take_single_operand(op, &mut __operand_index, 0)?,
            mask: take_single_operand(op, &mut __operand_index, 1)?,
            pass_thru: take_single_operand(op, &mut __operand_index, 2)?,
            alignment: op.attr("alignment").cloned(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_vec_shuffle(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::VecShuffle(vectors::VecShuffle {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            vec1: take_single_operand(op, &mut __operand_index, 0)?,
            vec2: take_single_operand(op, &mut __operand_index, 1)?,
            indices: op_attr(op, "indices")?.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_vec_shuffle_dynamic(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::VecShuffleDynamic(vectors::VecShuffleDynamic {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            vec: take_single_operand(op, &mut __operand_index, 0)?,
            indices: take_single_operand(op, &mut __operand_index, 1)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_vec_splat(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::VecSplat(vectors::VecSplat {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            value: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_vec_ternary(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::VecTernary(vectors::VecTernary {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            cond: take_single_operand(op, &mut __operand_index, 0)?,
            lhs: take_single_operand(op, &mut __operand_index, 1)?,
            rhs: take_single_operand(op, &mut __operand_index, 2)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_vtable_address_point(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::VtableAddressPoint(vtables::VtableAddressPoint {
            addr: op.results.get(0)?.0.clone(),
            addr_ty: op.results.get(0)?.1.clone(),
            name: op_attr(op, "name")?.clone(),
            address_point: op_attr(op, "address_point")?.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_vtable_get_type_info(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::VtableGetTypeInfo(vtables::VtableGetTypeInfo {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            vptr: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_vtable_get_virtual_fn_addr(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::VtableGetVirtualFnAddr(vtables::VtableGetVirtualFnAddr {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            vptr: take_single_operand(op, &mut __operand_index, 0)?,
            index: op_attr(op, "index")?.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_vtable_get_vptr(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::VtableGetVptr(vtables::VtableGetVptr {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            src: take_single_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_vtt_address_point(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::VttAddressPoint(vtables::VttAddressPoint {
            addr: op.results.get(0)?.0.clone(),
            addr_ty: op.results.get(0)?.1.clone(),
            name: op.attr("name").cloned(),
            sym_addr: take_optional_operand(op, &mut __operand_index, 0),
            offset: op_attr(op, "offset")?.clone(),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_while(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::While(control_flow::While {
            cleanup_kind: op
                .attr("cleanupKind")
                .and_then(|a| crate::enums::CleanupKind::try_from(a).ok()),
            cond: lower_region(op.regions.get(0)?),
            body: lower_region(op.regions.get(1)?),
            cleanup: lower_region(op.regions.get(2)?),
            loc: op.loc.clone(),
        }),
    )
}
fn lower_xor(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Xor(arithmetic::Xor {
            result: op.results.get(0)?.0.clone(),
            result_ty: op.results.get(0)?.1.clone(),
            lhs: take_single_operand(op, &mut __operand_index, 0)?,
            rhs: take_single_operand(op, &mut __operand_index, 1)?,
            loc: op.loc.clone(),
        }),
    )
}
fn lower_yield(op: &crate::ast::Operation) -> Option<Op> {
    let mut __operand_index = 0usize;
    Some(
        Op::Yield(control_flow::Yield {
            args: take_variadic_operand(op, &mut __operand_index, 0)?,
            loc: op.loc.clone(),
        }),
    )
}
pub fn lower_op(op: &crate::ast::Operation) -> Op {
    Op::from_operation(op).unwrap_or_else(|| Op::Other(op.clone()))
}
pub fn lower_block(block: &crate::ast::Block) -> Block {
    Block {
        label: block.label.clone(),
        args: block.args.clone(),
        ops: block.ops.iter().map(lower_op).collect(),
    }
}
pub fn lower_region(region: &crate::ast::Region) -> Region {
    Region {
        blocks: region.blocks.iter().map(lower_block).collect(),
    }
}
pub fn write_indent(f: &mut std::fmt::Formatter<'_>, level: usize) -> std::fmt::Result {
    for _ in 0..level {
        write!(f, "    ")?;
    }
    Ok(())
}
pub fn write_value_list(
    f: &mut std::fmt::Formatter<'_>,
    ids: &[ValueId],
) -> std::fmt::Result {
    for (i, id) in ids.iter().enumerate() {
        if i > 0 {
            write!(f, ", ")?;
        }
        write!(f, "%{id}")?;
    }
    Ok(())
}
pub fn write_flags(
    f: &mut std::fmt::Formatter<'_>,
    flags: &[(&str, bool)],
) -> std::fmt::Result {
    let set: Vec<&str> = flags
        .iter()
        .filter(|(_, v)| *v)
        .map(|(name, _)| *name)
        .collect();
    if !set.is_empty() {
        write!(f, " [{}]", set.join(", "))?;
    }
    Ok(())
}
pub fn write_block(
    f: &mut std::fmt::Formatter<'_>,
    block: &Block,
    level: usize,
) -> std::fmt::Result {
    if let Some(label) = &block.label {
        write_indent(f, level)?;
        write!(f, "^{label}")?;
        if !block.args.is_empty() {
            write!(f, "(")?;
            for (i, (id, ty)) in block.args.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "%{id}: {ty}")?;
            }
            write!(f, ")")?;
        }
        writeln!(f, ":")?;
    }
    for op in &block.ops {
        write_op(f, op, level + 1)?;
    }
    Ok(())
}
pub fn write_region(
    f: &mut std::fmt::Formatter<'_>,
    region: &Region,
    level: usize,
) -> std::fmt::Result {
    for block in &region.blocks {
        write_block(f, block, level)?;
    }
    Ok(())
}
pub fn write_op(
    f: &mut std::fmt::Formatter<'_>,
    op: &Op,
    level: usize,
) -> std::fmt::Result {
    match op {
        Op::Abs(_) => {
            write_indent(f, level)?;
            writeln!(f, "abs")
        }
        Op::Acos(_) => {
            write_indent(f, level)?;
            writeln!(f, "acos")
        }
        Op::Add(
            arithmetic::Add {
                result,
                result_ty,
                lhs,
                rhs,
                no_signed_wrap,
                no_unsigned_wrap,
                saturated,
                ..
            },
        ) => {
            write_indent(f, level)?;
            write!(f, "%{result} = add %{lhs}, %{rhs} : {result_ty}")?;
            write_flags(
                f,
                &[
                    ("nsw", *no_signed_wrap),
                    ("nuw", *no_unsigned_wrap),
                    ("sat", *saturated),
                ],
            )?;
            writeln!(f)
        }
        Op::AddOverflow(_) => {
            write_indent(f, level)?;
            writeln!(f, "add.overflow")
        }
        Op::AddressOfReturnAddress(_) => {
            write_indent(f, level)?;
            writeln!(f, "address_of_return_address")
        }
        Op::AllocException(_) => {
            write_indent(f, level)?;
            writeln!(f, "alloc.exception")
        }
        Op::Alloca(
            memory::Alloca { addr, addr_ty, dyn_alloc_size, name, alignment, .. },
        ) => {
            write_indent(f, level)?;
            write!(f, "%{addr} = alloca ")?;
            if let crate::types::Type::Pointer { pointee, .. } = addr_ty {
                write!(f, "{pointee}")?;
            } else {
                write!(f, "{addr_ty}")?;
            }
            write!(f, ", {name:?}")?;
            if let Some(size) = dyn_alloc_size {
                write!(f, ", size(%{size})")?;
            }
            write!(f, ", align {alignment}")?;
            writeln!(f)
        }
        Op::And(_) => {
            write_indent(f, level)?;
            writeln!(f, "and")
        }
        Op::ArrayCtor(_) => {
            write_indent(f, level)?;
            writeln!(f, "array.ctor")
        }
        Op::ArrayDtor(_) => {
            write_indent(f, level)?;
            writeln!(f, "array.dtor")
        }
        Op::Asin(_) => {
            write_indent(f, level)?;
            writeln!(f, "asin")
        }
        Op::Asm(_) => {
            write_indent(f, level)?;
            writeln!(f, "asm")
        }
        Op::Assume(_) => {
            write_indent(f, level)?;
            writeln!(f, "assume")
        }
        Op::Atan(_) => {
            write_indent(f, level)?;
            writeln!(f, "atan")
        }
        Op::Atan2(_) => {
            write_indent(f, level)?;
            writeln!(f, "atan2")
        }
        Op::AtomicClear(_) => {
            write_indent(f, level)?;
            writeln!(f, "atomic.clear")
        }
        Op::AtomicCmpxchg(_) => {
            write_indent(f, level)?;
            writeln!(f, "atomic.cmpxchg")
        }
        Op::AtomicFence(_) => {
            write_indent(f, level)?;
            writeln!(f, "atomic.fence")
        }
        Op::AtomicFetch(_) => {
            write_indent(f, level)?;
            writeln!(f, "atomic.fetch")
        }
        Op::AtomicTestAndSet(_) => {
            write_indent(f, level)?;
            writeln!(f, "atomic.test_and_set")
        }
        Op::AtomicXchg(_) => {
            write_indent(f, level)?;
            writeln!(f, "atomic.xchg")
        }
        Op::Await(_) => {
            write_indent(f, level)?;
            writeln!(f, "await")
        }
        Op::BaseClassAddr(_) => {
            write_indent(f, level)?;
            writeln!(f, "base_class_addr")
        }
        Op::BaseDataMember(_) => {
            write_indent(f, level)?;
            writeln!(f, "base_data_member")
        }
        Op::BaseMethod(_) => {
            write_indent(f, level)?;
            writeln!(f, "base_method")
        }
        Op::BeginCatch(
            exceptions::BeginCatch { catch_token, exn_ptr, exn_ptr_ty, eh_token, .. },
        ) => {
            write_indent(f, level)?;
            writeln!(
                f, "%{catch_token}, %{exn_ptr} = begin_catch %{eh_token} : {exn_ptr_ty}"
            )
        }
        Op::BeginCleanup(_) => {
            write_indent(f, level)?;
            writeln!(f, "begin_cleanup")
        }
        Op::Bitreverse(_) => {
            write_indent(f, level)?;
            writeln!(f, "bitreverse")
        }
        Op::BlockAddress(_) => {
            write_indent(f, level)?;
            writeln!(f, "block_address")
        }
        Op::Br(control_flow::Br { successors, .. }) => {
            write_indent(f, level)?;
            if let Some(dest) = successors.first() {
                writeln!(f, "br ^{dest}")
            } else {
                writeln!(f, "br")
            }
        }
        Op::Brcond(control_flow::Brcond { cond, successors, .. }) => {
            write_indent(f, level)?;
            write!(f, "brcond %{cond}")?;
            if let Some(dest) = successors.first() {
                write!(f, ", ^{dest}")?;
            }
            if let Some(dest) = successors.get(1) {
                write!(f, ", ^{dest}")?;
            }
            writeln!(f)
        }
        Op::Break(control_flow::Break { .. }) => {
            write_indent(f, level)?;
            writeln!(f, "break")
        }
        Op::BuiltinIntCast(_) => {
            write_indent(f, level)?;
            writeln!(f, "builtin_int_cast")
        }
        Op::ByteSwap(_) => {
            write_indent(f, level)?;
            writeln!(f, "byte_swap")
        }
        Op::Call(calls::Call { result, result_ty, callee, args, side_effect, .. }) => {
            write_indent(f, level)?;
            if let Some(result) = result {
                write!(f, "%{result} = ")?;
            }
            write!(f, "call ")?;
            if let Some(callee) = callee {
                write!(f, "{callee}")?;
            }
            write!(f, "(")?;
            write_value_list(f, args)?;
            write!(f, ")")?;
            if let Some(ty) = result_ty {
                write!(f, " : {ty}")?;
            }
            if let Ok(se) = crate::enums::SideEffect::try_from(side_effect) {
                write!(f, " [{se}]")?;
            }
            writeln!(f)
        }
        Op::CallLlvmIntrinsic(_) => {
            write_indent(f, level)?;
            writeln!(f, "call_llvm_intrinsic")
        }
        Op::Case(control_flow::Case { value, kind, case_region, .. }) => {
            write_indent(f, level)?;
            write!(f, "case {kind} {value} {{")?;
            writeln!(f)?;
            write_region(f, case_region, level + 1)?;
            write_indent(f, level)?;
            writeln!(f, "}}")
        }
        Op::Cast(misc::Cast { result, result_ty, kind, src, .. }) => {
            write_indent(f, level)?;
            writeln!(f, "%{result} = cast({kind}) %{src} : {result_ty}")
        }
        Op::CatchParam(_) => {
            write_indent(f, level)?;
            writeln!(f, "catch_param")
        }
        Op::Ceil(_) => {
            write_indent(f, level)?;
            writeln!(f, "ceil")
        }
        Op::CleanupScope(
            control_flow::CleanupScope { cleanup_kind, body_region, cleanup_region, .. },
        ) => {
            write_indent(f, level)?;
            writeln!(f, "cleanup.scope {{")?;
            write_region(f, body_region, level + 1)?;
            write_indent(f, level)?;
            writeln!(f, "}} cleanup {cleanup_kind} {{")?;
            write_region(f, cleanup_region, level + 1)?;
            write_indent(f, level)?;
            writeln!(f, "}}")
        }
        Op::ClearCache(_) => {
            write_indent(f, level)?;
            writeln!(f, "clear_cache")
        }
        Op::ClearPadding(_) => {
            write_indent(f, level)?;
            writeln!(f, "clear_padding")
        }
        Op::Clrsb(_) => {
            write_indent(f, level)?;
            writeln!(f, "clrsb")
        }
        Op::Clz(_) => {
            write_indent(f, level)?;
            writeln!(f, "clz")
        }
        Op::Cmp(_) => {
            write_indent(f, level)?;
            writeln!(f, "cmp")
        }
        Op::Cmp3way(_) => {
            write_indent(f, level)?;
            writeln!(f, "cmp3way")
        }
        Op::CoReturn(_) => {
            write_indent(f, level)?;
            writeln!(f, "co_return")
        }
        Op::ComplexAdd(_) => {
            write_indent(f, level)?;
            writeln!(f, "complex.add")
        }
        Op::ComplexConj(_) => {
            write_indent(f, level)?;
            writeln!(f, "complex.conj")
        }
        Op::ComplexCreate(_) => {
            write_indent(f, level)?;
            writeln!(f, "complex.create")
        }
        Op::ComplexDiv(_) => {
            write_indent(f, level)?;
            writeln!(f, "complex.div")
        }
        Op::ComplexImag(_) => {
            write_indent(f, level)?;
            writeln!(f, "complex.imag")
        }
        Op::ComplexImagPtr(_) => {
            write_indent(f, level)?;
            writeln!(f, "complex.imag_ptr")
        }
        Op::ComplexMul(_) => {
            write_indent(f, level)?;
            writeln!(f, "complex.mul")
        }
        Op::ComplexReal(_) => {
            write_indent(f, level)?;
            writeln!(f, "complex.real")
        }
        Op::ComplexRealPtr(_) => {
            write_indent(f, level)?;
            writeln!(f, "complex.real_ptr")
        }
        Op::ComplexSub(_) => {
            write_indent(f, level)?;
            writeln!(f, "complex.sub")
        }
        Op::Condition(control_flow::Condition { condition, .. }) => {
            write_indent(f, level)?;
            writeln!(f, "condition %{condition}")
        }
        Op::Const(globals::Const { res, res_ty, value, .. }) => {
            write_indent(f, level)?;
            writeln!(f, "%{res} = const {value} : {res_ty}")
        }
        Op::ConstructCatchParam(_) => {
            write_indent(f, level)?;
            writeln!(f, "construct_catch_param")
        }
        Op::Continue(control_flow::Continue { .. }) => {
            write_indent(f, level)?;
            writeln!(f, "continue")
        }
        Op::Copy(_) => {
            write_indent(f, level)?;
            writeln!(f, "copy")
        }
        Op::Copysign(_) => {
            write_indent(f, level)?;
            writeln!(f, "copysign")
        }
        Op::CoroBody(_) => {
            write_indent(f, level)?;
            writeln!(f, "coro.body")
        }
        Op::CoroIntrinsicAlloc(_) => {
            write_indent(f, level)?;
            writeln!(f, "coro.intrinsic.alloc")
        }
        Op::CoroIntrinsicBegin(_) => {
            write_indent(f, level)?;
            writeln!(f, "coro.intrinsic.begin")
        }
        Op::CoroIntrinsicEnd(_) => {
            write_indent(f, level)?;
            writeln!(f, "coro.intrinsic.end")
        }
        Op::CoroIntrinsicFree(_) => {
            write_indent(f, level)?;
            writeln!(f, "coro.intrinsic.free")
        }
        Op::CoroIntrinsicId(_) => {
            write_indent(f, level)?;
            writeln!(f, "coro.intrinsic.id")
        }
        Op::CoroIntrinsicSize(_) => {
            write_indent(f, level)?;
            writeln!(f, "coro.intrinsic.size")
        }
        Op::Cos(_) => {
            write_indent(f, level)?;
            writeln!(f, "cos")
        }
        Op::Cosh(_) => {
            write_indent(f, level)?;
            writeln!(f, "cosh")
        }
        Op::Cpuid(_) => {
            write_indent(f, level)?;
            writeln!(f, "cpuid")
        }
        Op::Ctz(_) => {
            write_indent(f, level)?;
            writeln!(f, "ctz")
        }
        Op::Dec(_) => {
            write_indent(f, level)?;
            writeln!(f, "dec")
        }
        Op::DeleteArray(_) => {
            write_indent(f, level)?;
            writeln!(f, "delete_array")
        }
        Op::DerivedClassAddr(_) => {
            write_indent(f, level)?;
            writeln!(f, "derived_class_addr")
        }
        Op::DerivedDataMember(_) => {
            write_indent(f, level)?;
            writeln!(f, "derived_data_member")
        }
        Op::DerivedMethod(_) => {
            write_indent(f, level)?;
            writeln!(f, "derived_method")
        }
        Op::Div(_) => {
            write_indent(f, level)?;
            writeln!(f, "div")
        }
        Op::Do(control_flow::Do { body, cond, .. }) => {
            write_indent(f, level)?;
            writeln!(f, "do {{")?;
            write_region(f, body, level + 1)?;
            write_indent(f, level)?;
            writeln!(f, "}} while {{")?;
            write_region(f, cond, level + 1)?;
            write_indent(f, level)?;
            writeln!(f, "}}")
        }
        Op::DynCast(_) => {
            write_indent(f, level)?;
            writeln!(f, "dyn_cast")
        }
        Op::EhDispatch(_) => {
            write_indent(f, level)?;
            writeln!(f, "eh.dispatch")
        }
        Op::EhInflightException(_) => {
            write_indent(f, level)?;
            writeln!(f, "eh.inflight_exception")
        }
        Op::EhInitiate(_) => {
            write_indent(f, level)?;
            writeln!(f, "eh.initiate")
        }
        Op::EhLongjmp(_) => {
            write_indent(f, level)?;
            writeln!(f, "eh.longjmp")
        }
        Op::EhSetjmp(_) => {
            write_indent(f, level)?;
            writeln!(f, "eh.setjmp")
        }
        Op::EhTerminate(_) => {
            write_indent(f, level)?;
            writeln!(f, "eh.terminate")
        }
        Op::EhTypeid(_) => {
            write_indent(f, level)?;
            writeln!(f, "eh.typeid")
        }
        Op::EndCatch(exceptions::EndCatch { catch_token, .. }) => {
            write_indent(f, level)?;
            writeln!(f, "end_catch %{catch_token}")
        }
        Op::EndCleanup(_) => {
            write_indent(f, level)?;
            writeln!(f, "end_cleanup")
        }
        Op::Exp(_) => {
            write_indent(f, level)?;
            writeln!(f, "exp")
        }
        Op::Exp10(_) => {
            write_indent(f, level)?;
            writeln!(f, "exp10")
        }
        Op::Exp2(_) => {
            write_indent(f, level)?;
            writeln!(f, "exp2")
        }
        Op::Expect(_) => {
            write_indent(f, level)?;
            writeln!(f, "expect")
        }
        Op::ExtractMember(_) => {
            write_indent(f, level)?;
            writeln!(f, "extract_member")
        }
        Op::Fabs(_) => {
            write_indent(f, level)?;
            writeln!(f, "fabs")
        }
        Op::Fadd(_) => {
            write_indent(f, level)?;
            writeln!(f, "fadd")
        }
        Op::Fdiv(_) => {
            write_indent(f, level)?;
            writeln!(f, "fdiv")
        }
        Op::Ffs(_) => {
            write_indent(f, level)?;
            writeln!(f, "ffs")
        }
        Op::Floor(_) => {
            write_indent(f, level)?;
            writeln!(f, "floor")
        }
        Op::Fma(_) => {
            write_indent(f, level)?;
            writeln!(f, "fma")
        }
        Op::Fmaximum(_) => {
            write_indent(f, level)?;
            writeln!(f, "fmaximum")
        }
        Op::Fmaxnum(_) => {
            write_indent(f, level)?;
            writeln!(f, "fmaxnum")
        }
        Op::Fminimum(_) => {
            write_indent(f, level)?;
            writeln!(f, "fminimum")
        }
        Op::Fminnum(_) => {
            write_indent(f, level)?;
            writeln!(f, "fminnum")
        }
        Op::Fmod(_) => {
            write_indent(f, level)?;
            writeln!(f, "fmod")
        }
        Op::Fmul(_) => {
            write_indent(f, level)?;
            writeln!(f, "fmul")
        }
        Op::Fmuladd(_) => {
            write_indent(f, level)?;
            writeln!(f, "fmuladd")
        }
        Op::Fneg(_) => {
            write_indent(f, level)?;
            writeln!(f, "fneg")
        }
        Op::For(control_flow::For { cond, body, step, .. }) => {
            write_indent(f, level)?;
            writeln!(f, "for cond {{")?;
            write_region(f, cond, level + 1)?;
            write_indent(f, level)?;
            writeln!(f, "}} step {{")?;
            write_region(f, step, level + 1)?;
            write_indent(f, level)?;
            writeln!(f, "}} body {{")?;
            write_region(f, body, level + 1)?;
            write_indent(f, level)?;
            writeln!(f, "}}")
        }
        Op::FrameAddress(_) => {
            write_indent(f, level)?;
            writeln!(f, "frame_address")
        }
        Op::Freeze(_) => {
            write_indent(f, level)?;
            writeln!(f, "freeze")
        }
        Op::Frem(_) => {
            write_indent(f, level)?;
            writeln!(f, "frem")
        }
        Op::Frexp(_) => {
            write_indent(f, level)?;
            writeln!(f, "frexp")
        }
        Op::Fsub(_) => {
            write_indent(f, level)?;
            writeln!(f, "fsub")
        }
        Op::Func(_) => {
            write_indent(f, level)?;
            writeln!(f, "func")
        }
        Op::GetBitfield(_) => {
            write_indent(f, level)?;
            writeln!(f, "get_bitfield")
        }
        Op::GetElement(_) => {
            write_indent(f, level)?;
            writeln!(f, "get_element")
        }
        Op::GetGlobal(globals::GetGlobal { addr, addr_ty, name, .. }) => {
            write_indent(f, level)?;
            writeln!(f, "%{addr} = get_global {name} : {addr_ty}")
        }
        Op::GetMember(_) => {
            write_indent(f, level)?;
            writeln!(f, "get_member")
        }
        Op::GetMethod(_) => {
            write_indent(f, level)?;
            writeln!(f, "get_method")
        }
        Op::GetRuntimeMember(_) => {
            write_indent(f, level)?;
            writeln!(f, "get_runtime_member")
        }
        Op::Global(_) => {
            write_indent(f, level)?;
            writeln!(f, "global")
        }
        Op::Goto(control_flow::Goto { label, .. }) => {
            write_indent(f, level)?;
            writeln!(f, "goto {label}")
        }
        Op::If(control_flow::If { condition, then_region, else_region, .. }) => {
            write_indent(f, level)?;
            writeln!(f, "if %{condition} {{")?;
            write_region(f, then_region, level + 1)?;
            if !else_region.blocks.iter().all(|b| b.ops.is_empty()) {
                write_indent(f, level)?;
                writeln!(f, "}} else {{")?;
                write_region(f, else_region, level + 1)?;
            }
            write_indent(f, level)?;
            writeln!(f, "}}")
        }
        Op::Inc(_) => {
            write_indent(f, level)?;
            writeln!(f, "inc")
        }
        Op::IndirectBr(_) => {
            write_indent(f, level)?;
            writeln!(f, "indirect_br")
        }
        Op::IndirectGoto(control_flow::IndirectGoto { addr, .. }) => {
            write_indent(f, level)?;
            writeln!(f, "indirect_goto %{addr}")
        }
        Op::InitCatchParam(
            exceptions::InitCatchParam { exn_ptr, param_addr, kind, .. },
        ) => {
            write_indent(f, level)?;
            writeln!(f, "init_catch_param {kind} %{exn_ptr} to %{param_addr}")
        }
        Op::InsertMember(_) => {
            write_indent(f, level)?;
            writeln!(f, "insert_member")
        }
        Op::IsConstant(_) => {
            write_indent(f, level)?;
            writeln!(f, "is_constant")
        }
        Op::IsFpClass(_) => {
            write_indent(f, level)?;
            writeln!(f, "is_fp_class")
        }
        Op::Label(control_flow::Label { label, .. }) => {
            write_indent(f, level)?;
            writeln!(f, "label {label}:")
        }
        Op::Launder(_) => {
            write_indent(f, level)?;
            writeln!(f, "launder")
        }
        Op::LibcMemchr(_) => {
            write_indent(f, level)?;
            writeln!(f, "libc.memchr")
        }
        Op::LibcMemcpy(_) => {
            write_indent(f, level)?;
            writeln!(f, "libc.memcpy")
        }
        Op::LibcMemmove(_) => {
            write_indent(f, level)?;
            writeln!(f, "libc.memmove")
        }
        Op::LibcMemset(_) => {
            write_indent(f, level)?;
            writeln!(f, "libc.memset")
        }
        Op::LifetimeEnd(_) => {
            write_indent(f, level)?;
            writeln!(f, "lifetime.end")
        }
        Op::LifetimeStart(_) => {
            write_indent(f, level)?;
            writeln!(f, "lifetime.start")
        }
        Op::Llrint(_) => {
            write_indent(f, level)?;
            writeln!(f, "llrint")
        }
        Op::Llround(_) => {
            write_indent(f, level)?;
            writeln!(f, "llround")
        }
        Op::Load(memory::Load { result, result_ty, addr, alignment, .. }) => {
            write_indent(f, level)?;
            write!(f, "%{result} = load %{addr} : {result_ty}")?;
            if let Some(a) = alignment {
                if let Some(v) = a.as_int() {
                    write!(f, ", align {v}")?;
                }
            }
            writeln!(f)
        }
        Op::LocalInit(_) => {
            write_indent(f, level)?;
            writeln!(f, "local_init")
        }
        Op::Log(_) => {
            write_indent(f, level)?;
            writeln!(f, "log")
        }
        Op::Log10(_) => {
            write_indent(f, level)?;
            writeln!(f, "log10")
        }
        Op::Log2(_) => {
            write_indent(f, level)?;
            writeln!(f, "log2")
        }
        Op::Lrint(_) => {
            write_indent(f, level)?;
            writeln!(f, "lrint")
        }
        Op::Lround(_) => {
            write_indent(f, level)?;
            writeln!(f, "lround")
        }
        Op::Max(_) => {
            write_indent(f, level)?;
            writeln!(f, "max")
        }
        Op::Min(_) => {
            write_indent(f, level)?;
            writeln!(f, "min")
        }
        Op::Minus(_) => {
            write_indent(f, level)?;
            writeln!(f, "minus")
        }
        Op::Modf(_) => {
            write_indent(f, level)?;
            writeln!(f, "modf")
        }
        Op::Mul(_) => {
            write_indent(f, level)?;
            writeln!(f, "mul")
        }
        Op::MulOverflow(_) => {
            write_indent(f, level)?;
            writeln!(f, "mul.overflow")
        }
        Op::Nearbyint(_) => {
            write_indent(f, level)?;
            writeln!(f, "nearbyint")
        }
        Op::Not(_) => {
            write_indent(f, level)?;
            writeln!(f, "not")
        }
        Op::Objsize(_) => {
            write_indent(f, level)?;
            writeln!(f, "objsize")
        }
        Op::Or(_) => {
            write_indent(f, level)?;
            writeln!(f, "or")
        }
        Op::Parity(_) => {
            write_indent(f, level)?;
            writeln!(f, "parity")
        }
        Op::Popcount(_) => {
            write_indent(f, level)?;
            writeln!(f, "popcount")
        }
        Op::Pow(_) => {
            write_indent(f, level)?;
            writeln!(f, "pow")
        }
        Op::Prefetch(_) => {
            write_indent(f, level)?;
            writeln!(f, "prefetch")
        }
        Op::PtrDiff(_) => {
            write_indent(f, level)?;
            writeln!(f, "ptr_diff")
        }
        Op::PtrStride(_) => {
            write_indent(f, level)?;
            writeln!(f, "ptr_stride")
        }
        Op::Rem(_) => {
            write_indent(f, level)?;
            writeln!(f, "rem")
        }
        Op::Resume(exceptions::Resume { eh_token, .. }) => {
            write_indent(f, level)?;
            writeln!(f, "resume %{eh_token}")
        }
        Op::ResumeFlat(_) => {
            write_indent(f, level)?;
            writeln!(f, "resume.flat")
        }
        Op::Return(control_flow::Return { input, .. }) => {
            write_indent(f, level)?;
            write!(f, "return")?;
            if let Some(v) = input.first() {
                write!(f, " %{v}")?;
            }
            writeln!(f)
        }
        Op::ReturnAddress(_) => {
            write_indent(f, level)?;
            writeln!(f, "return_address")
        }
        Op::Rint(_) => {
            write_indent(f, level)?;
            writeln!(f, "rint")
        }
        Op::Rotate(_) => {
            write_indent(f, level)?;
            writeln!(f, "rotate")
        }
        Op::Round(_) => {
            write_indent(f, level)?;
            writeln!(f, "round")
        }
        Op::Roundeven(_) => {
            write_indent(f, level)?;
            writeln!(f, "roundeven")
        }
        Op::Scope(control_flow::Scope { results, scope_region, .. }) => {
            write_indent(f, level)?;
            if let Some(results) = results {
                write!(f, "%{results} = ")?;
            }
            writeln!(f, "scope {{")?;
            write_region(f, scope_region, level + 1)?;
            write_indent(f, level)?;
            writeln!(f, "}}")
        }
        Op::Select(_) => {
            write_indent(f, level)?;
            writeln!(f, "select")
        }
        Op::SetBitfield(_) => {
            write_indent(f, level)?;
            writeln!(f, "set_bitfield")
        }
        Op::Shift(_) => {
            write_indent(f, level)?;
            writeln!(f, "shift")
        }
        Op::Signbit(_) => {
            write_indent(f, level)?;
            writeln!(f, "signbit")
        }
        Op::Sin(_) => {
            write_indent(f, level)?;
            writeln!(f, "sin")
        }
        Op::Sinh(_) => {
            write_indent(f, level)?;
            writeln!(f, "sinh")
        }
        Op::Sqrt(_) => {
            write_indent(f, level)?;
            writeln!(f, "sqrt")
        }
        Op::Stackrestore(_) => {
            write_indent(f, level)?;
            writeln!(f, "stackrestore")
        }
        Op::Stacksave(_) => {
            write_indent(f, level)?;
            writeln!(f, "stacksave")
        }
        Op::StdFind(_) => {
            write_indent(f, level)?;
            writeln!(f, "std.find")
        }
        Op::StdStrlen(_) => {
            write_indent(f, level)?;
            writeln!(f, "std.strlen")
        }
        Op::Store(memory::Store { value, addr, alignment, .. }) => {
            write_indent(f, level)?;
            write!(f, "store %{value}, %{addr}")?;
            if let Some(a) = alignment {
                if let Some(v) = a.as_int() {
                    write!(f, ", align {v}")?;
                }
            }
            writeln!(f)
        }
        Op::Sub(_) => {
            write_indent(f, level)?;
            writeln!(f, "sub")
        }
        Op::SubOverflow(_) => {
            write_indent(f, level)?;
            writeln!(f, "sub.overflow")
        }
        Op::Switch(control_flow::Switch { condition, body, .. }) => {
            write_indent(f, level)?;
            writeln!(f, "switch %{condition} {{")?;
            write_region(f, body, level + 1)?;
            write_indent(f, level)?;
            writeln!(f, "}}")
        }
        Op::SwitchFlat(_) => {
            write_indent(f, level)?;
            writeln!(f, "switch.flat")
        }
        Op::Tan(_) => {
            write_indent(f, level)?;
            writeln!(f, "tan")
        }
        Op::Tanh(_) => {
            write_indent(f, level)?;
            writeln!(f, "tanh")
        }
        Op::Ternary(
            control_flow::Ternary {
                result,
                result_ty,
                cond,
                true_region,
                false_region,
                ..
            },
        ) => {
            write_indent(f, level)?;
            if let Some(result) = result {
                write!(f, "%{result} = ")?;
            }
            write!(f, "ternary %{cond} ? {{")?;
            if let Some(ty) = result_ty {
                write!(f, " : {ty}")?;
            }
            writeln!(f)?;
            write_region(f, true_region, level + 1)?;
            write_indent(f, level)?;
            writeln!(f, "}} : {{")?;
            write_region(f, false_region, level + 1)?;
            write_indent(f, level)?;
            writeln!(f, "}}")
        }
        Op::Throw(_) => {
            write_indent(f, level)?;
            writeln!(f, "throw")
        }
        Op::TokenNone(_) => {
            write_indent(f, level)?;
            writeln!(f, "token.none")
        }
        Op::Trap(control_flow::Trap { .. }) => {
            write_indent(f, level)?;
            writeln!(f, "trap")
        }
        Op::Trunc(_) => {
            write_indent(f, level)?;
            writeln!(f, "trunc")
        }
        Op::Try(exceptions::Try { cleanup, try_region, handler_regions, .. }) => {
            write_indent(f, level)?;
            write!(f, "try")?;
            write_flags(f, &[("cleanup", *cleanup)])?;
            writeln!(f, " {{")?;
            write_region(f, try_region, level + 1)?;
            write_indent(f, level)?;
            writeln!(f, "}}")?;
            for handler in handler_regions {
                write_indent(f, level)?;
                writeln!(f, "catch {{")?;
                write_region(f, handler, level + 1)?;
                write_indent(f, level)?;
                writeln!(f, "}}")?;
            }
            Ok(())
        }
        Op::TryCall(_) => {
            write_indent(f, level)?;
            writeln!(f, "try_call")
        }
        Op::TryThrow(_) => {
            write_indent(f, level)?;
            writeln!(f, "try_throw")
        }
        Op::Unreachable(control_flow::Unreachable { .. }) => {
            write_indent(f, level)?;
            writeln!(f, "unreachable")
        }
        Op::VaArg(_) => {
            write_indent(f, level)?;
            writeln!(f, "va_arg")
        }
        Op::VaCopy(_) => {
            write_indent(f, level)?;
            writeln!(f, "va_copy")
        }
        Op::VaEnd(_) => {
            write_indent(f, level)?;
            writeln!(f, "va_end")
        }
        Op::VaStart(_) => {
            write_indent(f, level)?;
            writeln!(f, "va_start")
        }
        Op::VecCmp(_) => {
            write_indent(f, level)?;
            writeln!(f, "vec.cmp")
        }
        Op::VecCreate(_) => {
            write_indent(f, level)?;
            writeln!(f, "vec.create")
        }
        Op::VecExtract(_) => {
            write_indent(f, level)?;
            writeln!(f, "vec.extract")
        }
        Op::VecInsert(_) => {
            write_indent(f, level)?;
            writeln!(f, "vec.insert")
        }
        Op::VecMaskedLoad(_) => {
            write_indent(f, level)?;
            writeln!(f, "vec.masked_load")
        }
        Op::VecShuffle(_) => {
            write_indent(f, level)?;
            writeln!(f, "vec.shuffle")
        }
        Op::VecShuffleDynamic(_) => {
            write_indent(f, level)?;
            writeln!(f, "vec.shuffle.dynamic")
        }
        Op::VecSplat(_) => {
            write_indent(f, level)?;
            writeln!(f, "vec.splat")
        }
        Op::VecTernary(_) => {
            write_indent(f, level)?;
            writeln!(f, "vec.ternary")
        }
        Op::VtableAddressPoint(_) => {
            write_indent(f, level)?;
            writeln!(f, "vtable.address_point")
        }
        Op::VtableGetTypeInfo(_) => {
            write_indent(f, level)?;
            writeln!(f, "vtable.get_type_info")
        }
        Op::VtableGetVirtualFnAddr(_) => {
            write_indent(f, level)?;
            writeln!(f, "vtable.get_virtual_fn_addr")
        }
        Op::VtableGetVptr(_) => {
            write_indent(f, level)?;
            writeln!(f, "vtable.get_vptr")
        }
        Op::VttAddressPoint(_) => {
            write_indent(f, level)?;
            writeln!(f, "vtt.address_point")
        }
        Op::While(control_flow::While { cond, body, .. }) => {
            write_indent(f, level)?;
            writeln!(f, "while {{")?;
            write_region(f, cond, level + 1)?;
            write_indent(f, level)?;
            writeln!(f, "}} do {{")?;
            write_region(f, body, level + 1)?;
            write_indent(f, level)?;
            writeln!(f, "}}")
        }
        Op::Xor(_) => {
            write_indent(f, level)?;
            writeln!(f, "xor")
        }
        Op::Yield(control_flow::Yield { args, .. }) => {
            write_indent(f, level)?;
            write!(f, "yield")?;
            if let Some(v) = args.first() {
                write!(f, " %{v}")?;
            }
            writeln!(f)
        }
        Op::Other(raw) => {
            write_indent(f, level)?;
            writeln!(f, "<unmodeled: {}>", raw.name)
        }
    }
}
fn op_attr<'a>(
    op: &'a crate::ast::Operation,
    key: &str,
) -> Option<&'a crate::attrs::Attribute> {
    op.attr(key)
}
fn unit_attr(op: &crate::ast::Operation, key: &str) -> bool {
    op.attr(key).is_some_and(|a| !matches!(a, crate ::attrs::Attribute::Bool(false)))
}
fn attr_str(op: &crate::ast::Operation, key: &str) -> Option<String> {
    op.attr(key).and_then(|a| a.as_str().map(str::to_string))
}
fn attr_type(op: &crate::ast::Operation, key: &str) -> Option<crate::types::Type> {
    op.attr(key).and_then(|a| a.as_type().cloned())
}
#[allow(dead_code)]
fn attr_i32(op: &crate::ast::Operation, key: &str) -> Option<i32> {
    op.attr(key).and_then(|a| a.as_int()).and_then(|v| i32::try_from(v).ok())
}
#[allow(dead_code)]
fn attr_i64(op: &crate::ast::Operation, key: &str) -> Option<i64> {
    op.attr(key).and_then(|a| a.as_int()).and_then(|v| i64::try_from(v).ok())
}
fn attr_u64(op: &crate::ast::Operation, key: &str) -> Option<u64> {
    op.attr(key).and_then(|a| a.as_int()).and_then(|v| u64::try_from(v).ok())
}
fn dense_i32_array(attr: &crate::attrs::Attribute) -> Option<Vec<usize>> {
    match attr {
        crate::attrs::Attribute::Dialect {
            dialect,
            mnemonic,
            raw: Some(raw),
            ..
        } if dialect == "builtin" && mnemonic == "array" => {
            let (_elem_ty, list) = raw.split_once(':')?;
            list.split(',').map(|v| v.trim().parse::<usize>().ok()).collect()
        }
        crate::attrs::Attribute::Array(items) => {
            items
                .iter()
                .map(|a| a.as_int().and_then(|v| usize::try_from(v).ok()))
                .collect()
        }
        _ => None,
    }
}
fn operand_segment_sizes(op: &crate::ast::Operation) -> Option<Vec<usize>> {
    op.attr("operandSegmentSizes").and_then(dense_i32_array)
}
fn take_operand_group(
    op: &crate::ast::Operation,
    index: &mut usize,
    group_index: usize,
) -> Option<Vec<ValueId>> {
    if let Some(sizes) = operand_segment_sizes(op) {
        let size = *sizes.get(group_index)?;
        let end = index.checked_add(size)?;
        let values = op.operands.get(*index..end)?.to_vec();
        *index = end;
        Some(values)
    } else {
        let value = op.operands.get(*index)?.clone();
        *index += 1;
        Some(vec![value])
    }
}
fn take_single_operand(
    op: &crate::ast::Operation,
    index: &mut usize,
    group_index: usize,
) -> Option<ValueId> {
    let mut values = take_operand_group(op, index, group_index)?;
    (values.len() == 1).then(|| values.remove(0))
}
fn take_optional_operand(
    op: &crate::ast::Operation,
    index: &mut usize,
    group_index: usize,
) -> Option<ValueId> {
    if let Some(sizes) = operand_segment_sizes(op) {
        let size = *sizes.get(group_index)?;
        match size {
            0 => None,
            1 => {
                let value = op.operands.get(*index)?.clone();
                *index += 1;
                Some(value)
            }
            _ => None,
        }
    } else if *index < op.operands.len() {
        let value = op.operands.get(*index)?.clone();
        *index += 1;
        Some(value)
    } else {
        None
    }
}
fn take_variadic_operand(
    op: &crate::ast::Operation,
    index: &mut usize,
    group_index: usize,
) -> Option<Vec<ValueId>> {
    if operand_segment_sizes(op).is_some() {
        take_operand_group(op, index, group_index)
    } else {
        let values = op.operands.get(*index..)?.to_vec();
        *index = op.operands.len();
        Some(values)
    }
}
fn take_variadic_of_variadic(
    op: &crate::ast::Operation,
    index: &mut usize,
    segments_key: &str,
) -> Option<Vec<Vec<ValueId>>> {
    let sizes = dense_i32_array(op.attr(segments_key)?)?;
    let mut groups = Vec::with_capacity(sizes.len());
    for size in sizes {
        let end = index.checked_add(size)?;
        groups.push(op.operands.get(*index..end)?.to_vec());
        *index = end;
    }
    Some(groups)
}
impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write_op(f, self, 0)
    }
}