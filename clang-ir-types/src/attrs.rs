//! CIR attributes generated from CIRAttrs.td and related files.

#![allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Attribute {
    /// `#cir.var.decl.ast`
    /// Wraps a 'const clang::VarDecl *' AST node.
    ///
    /// Operations optionally refer to this node, they could be available depending
    /// on the CIR lowering stage. Whether it's attached to the appropriate
    /// CIR operation is delegated to the operation verifier.
    ///
    /// Note: the AST pointer can be null when CIR is parsed from text, since
    /// there is no serialization support for AST nodes yet.
    AstVarDecl { ast: String },
    /// `#cir.address_point`
    /// Address point attribute
    ///
    /// Attribute specifying the address point within a C++ virtual table (vtable).
    ///
    /// The `index` (vtable index) parameter identifies which vtable to use within a
    /// vtable group, while the `offset` (address point index) specifies the offset
    /// within that vtable where the address begins.
    ///
    /// Example:
    /// ```
    /// cir.global linkonce_odr @_ZTV1B = ...
    /// ...
    /// %3 = cir.vtable.address_point(@_ZTV1B,
    ///                               address_point = <index = 0, offset = 2>)
    ///                              : !cir.vptr
    /// ```
    AddressPoint { index: i32, offset: i32 },
    /// `#cir.annotation`
    /// Annotation attribute for global variables and functions
    ///
    /// Represents a C/C++ `__attribute__((annotate(...)))` in CIR.
    ///
    /// Example C code:
    /// ```c
    /// int *a __attribute__((annotate("testptr", "21", 12)));
    /// ```
    ///
    /// In CIR, the attribute for above annotation looks like:
    /// ```
    /// [#cir.annotation<"testptr", ["21", 12 : i32]>]
    /// ```
    Annotation { name: String, args: Option<Box<Attribute>> },
    /// `#cir.asm_flavor`
    /// ATT or Intel
    /// Enum cases:
    /// * x86_att (`x86_att`)
    /// * x86_intel (`x86_intel`)
    AsmFlavor {
        /// an enum of type AsmFlavor
        value: crate::enums::AsmFlavor,
    },
    /// `#cir.assume_bundle`
    /// kind of cir.assume operand bundle
    /// Enum cases:
    /// * None (`None`)
    /// * align (`Align`)
    /// * separate_storage (`SeparateStorage`)
    /// * dereferenceable (`Dereferenceable`)
    AssumeBundleKind {
        /// an enum of type AssumeBundleKind
        value: crate::enums::AssumeBundleKind,
    },
    /// `#cir.atomic_fetch`
    /// Binary opcode for atomic fetch-and-update operations
    /// Enum cases:
    /// * add (`Add`)
    /// * sub (`Sub`)
    /// * and (`And`)
    /// * xor (`Xor`)
    /// * or (`Or`)
    /// * nand (`Nand`)
    /// * max (`Max`)
    /// * min (`Min`)
    /// * uinc_wrap (`UIncWrap`)
    /// * udec_wrap (`UDecWrap`)
    /// * maximum (`Maximum`)
    /// * minimum (`Minimum`)
    /// * maximum_num (`MaximumNum`)
    /// * minimum_num (`MinimumNum`)
    AtomicFetchKind {
        /// an enum of type AtomicFetchKind
        value: crate::enums::AtomicFetchKind,
    },
    /// `#cir.await`
    /// await kind
    /// Enum cases:
    /// * init (`Init`)
    /// * user (`User`)
    /// * yield (`Yield`)
    /// * final (`Final`)
    AwaitKind {
        /// an enum of type AwaitKind
        value: crate::enums::AwaitKind,
    },
    /// `#cir.bitfield_decl`
    /// How the source declared one bit-field
    ///
    /// Records a single bit-field as its declaration wrote it: the declared type,
    /// the number of bits the field holds, and whether the declaration named it.
    ///
    /// A run of bit-fields shares one access unit whose width the compiler picks,
    /// so none of this can be recovered from the unit.  The fields in a run need
    /// not even agree on a declared type: `struct { int a : 2; long long b : 30; }`
    /// packs an `int` and a `long long` into a single 32-bit unit.  This attribute
    /// is therefore per field, and `!cir.bitfield` carries one of them for every
    /// bit-field in the unit.
    ///
    /// A width of zero is a zero-width bit-field, `int : 0`, which holds no bits
    /// and exists only to end the run before it.
    ///
    /// `unnamed` marks a bit-field the source cannot read, `int : 3`.  It still
    /// occupies bits of the unit, and the x86-64 classifier reads it, but it is
    /// not a field any expression names.  Every zero-width bit-field is unnamed.
    ///
    /// Example:
    /// ```
    ///   #cir.bitfield_decl<!s32i, 4>
    ///   #cir.bitfield_decl<!s64i, 30>
    ///   #cir.bitfield_decl<!s32i, 3, unnamed>
    ///   #cir.bitfield_decl<!u32i, 0, unnamed>
    /// ```
    BitFieldDecl {
        /// integer or boolean type
        declared_type: crate::types::Type,
        width: u64,
        /// unnamed bit-field flag
        is_unnamed: Box<Attribute>,
    },
    /// `#cir.bitfield_info`
    /// Represents info for a bit-field member
    ///
    /// Holds the following information about bitfields: name, storage type, size
    /// and position in the storage, and signedness.
    /// Example:
    ///   Given the following struct with bitfields:
    ///     ```c++
    ///     typedef struct {
    ///       int a : 4;
    ///       int b : 27;
    ///       int c : 17;
    ///       int d : 2;
    ///       int e : 15;
    ///     } S;
    ///     ```
    ///
    ///   The CIR representation of the struct `S` might look like:
    ///   ```
    ///     !rec_S = !cir.struct<"S" packed {!u64i, !u16i,
    ///                                      pad !cir.array<!u8i x 2>}>
    ///   ```
    ///   And the bitfield info attribute for member `a` would be:
    ///   ```
    ///     #bfi_a = #cir.bitfield_info<name = "a", storage_type = !u64i,
    ///                                 size = 4, offset = 0, is_signed = true>
    ///   ```
    ///
    ///   This metadata describes that field `a` is stored in a 64-bit integer,
    ///   is 4 bits wide, starts at offset 0, and is signed.
    BitfieldInfo {
        name: String,
        storage_type: crate::types::Type,
        size: u64,
        offset: u64,
        is_signed: bool,
    },
    /// `#cir.block_addr_diff`
    /// Difference between two block addresses
    ///
    /// This attribute represents the constant difference between the addresses of
    /// two basic blocks within the same function. It is produced for GCC's "labels
    /// as values" extension when the difference of two label addresses appears in a
    /// constant context, e.g. `&&lhs - &&rhs`.
    ///
    /// Both labels belong to the function referenced by `func`. The value is the
    /// address of `lhs_label` minus the address of `rhs_label`, truncated to the
    /// attribute's integer type.
    ///
    /// Example:
    /// ```
    /// cir.global "private" internal @b.ar =
    ///     #cir.block_addr_diff<@b, "l2", "l1"> : !s32i
    /// ```
    BlockAddrDiff {
        ty: crate::types::Type,
        func: String,
        lhs_label: String,
        rhs_label: String,
    },
    /// `#cir.block_addr_info`
    /// Block address attribute
    ///
    /// This attribute is used to represent the address of a basic block
    /// within a function. It combines the symbol reference to a function
    /// with the name of a label inside that function.
    BlockAddrInfo { ty: crate::types::Type, func: String, label: String },
    /// `#cir.bool`
    /// Represent true/false for !cir.bool types
    ///
    /// The BoolAttr represents a 'true' or 'false' value.
    CirBool { ty: crate::types::Type, value: bool },
    /// `#cir.cu.externally_initialized`
    /// The marked variable is externally initialized.
    ///
    /// CUDA __device__ and __constant__ variables, along with surface and
    /// textures, might be initialized by host, hence "externally initialized".
    /// Therefore they must be emitted even if they are not referenced.
    ///
    /// The attribute corresponds to the attribute on LLVM with the same name.
    CudaExternallyInitialized,
    /// `#cir.cu.kernel_name`
    /// Device-side function name for this stub.
    ///
    /// This attribute is attached to function definitions and records the
    /// mangled name of the kernel function used on the device.
    ///
    /// In CUDA, global functions (kernels) are processed differently for host
    /// and device. On host, Clang generates device stubs; on device, they are
    /// treated as normal functions. As they probably have different mangled
    /// names, we must record the corresponding device-side name for a stub.
    /// Preserving the device-side kernel name is crucial for performing its
    /// respective function runtime registration on the host.
    CudaKernelName { kernel_name: String },
    /// `#cir.cu.var_registration`
    /// Device variable registration flags.
    CudaVarRegistrationInfo {
        device_side_name: String,
        kind: crate::enums::CudaDeviceVarKind,
        is_extern: bool,
        is_constant: bool,
        is_managed: bool,
    },
    /// `#cir.cxx_assign`
    /// Marks a function as a CXX assignment operator
    ///
    /// This attribute identifies a C++ assignment operator and classifies its kind:
    ///
    /// - `copy`: a copy assignment
    /// - `move`: a move assignment
    CxxAssign {
        ty: crate::types::Type,
        /// an enum of type AssignKind
        assign_kind: crate::enums::AssignKind,
        is_trivial: bool,
    },
    /// `#cir.cxx_ctor`
    /// Marks a function as a C++ constructor
    ///
    /// This attribute identifies a C++ constructor and classifies its kind:
    ///
    /// - `custom`: a user-defined constructor
    /// - `default`: a default constructor
    /// - `copy`: a copy constructor
    /// - `move`: a move constructor
    ///
    /// Example:
    /// ```
    /// #cir.cxx_ctor<!rec_a, copy>
    /// #cir.cxx_ctor<!rec_b, default, trivial>
    /// ```
    CxxCtor {
        ty: crate::types::Type,
        /// an enum of type CtorKind
        ctor_kind: crate::enums::CtorKind,
        is_trivial: bool,
    },
    /// `#cir.cxx_dtor`
    /// Marks a function as a CXX destructor
    ///
    /// This attribute identifies a C++ destructor.
    CxxDtor { ty: crate::types::Type, is_trivial: bool },
    /// `#cir.calling_conv`
    /// calling convention
    /// Enum cases:
    /// * c (`C`)
    /// * ptx_kernel (`PTXKernel`)
    /// * spir_function (`SpirFunction`)
    /// * spir_kernel (`SpirKernel`)
    /// * amdgpu_kernel (`AMDGPUKernel`)
    CallingConv {
        /// an enum of type CallingConv
        value: crate::enums::CallingConv,
    },
    /// `#cir.case`
    /// case kind
    /// Enum cases:
    /// * default (`Default`)
    /// * equal (`Equal`)
    /// * anyof (`Anyof`)
    /// * range (`Range`)
    CaseOpKind {
        /// an enum of type CaseOpKind
        value: crate::enums::CaseOpKind,
    },
    /// `#cir.cast`
    /// cast kind
    /// Enum cases:
    /// * bitcast (`bitcast`)
    /// * array_to_ptrdecay (`array_to_ptrdecay`)
    /// * member_ptr_to_bool (`member_ptr_to_bool`)
    /// * int_to_ptr (`int_to_ptr`)
    /// * ptr_to_int (`ptr_to_int`)
    /// * ptr_to_bool (`ptr_to_bool`)
    /// * integral (`integral`)
    /// * int_to_bool (`int_to_bool`)
    /// * int_to_float (`int_to_float`)
    /// * float_to_int (`float_to_int`)
    /// * float_to_bool (`float_to_bool`)
    /// * bool_to_int (`bool_to_int`)
    /// * floating (`floating`)
    /// * float_to_complex (`float_to_complex`)
    /// * float_complex_to_real (`float_complex_to_real`)
    /// * float_complex_to_bool (`float_complex_to_bool`)
    /// * float_complex (`float_complex`)
    /// * float_complex_to_int_complex (`float_complex_to_int_complex`)
    /// * int_to_complex (`int_to_complex`)
    /// * int_complex_to_real (`int_complex_to_real`)
    /// * int_complex_to_bool (`int_complex_to_bool`)
    /// * int_complex (`int_complex`)
    /// * int_complex_to_float_complex (`int_complex_to_float_complex`)
    /// * address_space (`address_space`)
    /// * bool_to_float (`bool_to_float`)
    CastKind {
        /// an enum of type CastKind
        value: crate::enums::CastKind,
    },
    /// `#cir.all`
    CatchAll,
    /// `#cir.cleanup`
    /// cleanup kind
    /// Enum cases:
    /// * normal (`Normal`)
    /// * eh (`EH`)
    /// * all (`All`)
    CleanupKind {
        /// an enum of type CleanupKind
        value: crate::enums::CleanupKind,
    },
    /// `#cir.cmp`
    /// compare operation kind
    /// Enum cases:
    /// * lt (`lt`)
    /// * le (`le`)
    /// * gt (`gt`)
    /// * ge (`ge`)
    /// * eq (`eq`)
    /// * ne (`ne`)
    /// * one (`one`)
    /// * uno (`uno`)
    CmpOpKind {
        /// an enum of type CmpOpKind
        value: crate::enums::CmpOpKind,
    },
    /// `#cir.cmp3way_info`
    /// Holds information about a three-way comparison operation
    ///
    /// The `#cmpinfo` attribute contains information about a three-way
    /// comparison operation `cir.cmp3way`.
    ///
    /// The `ordering` parameter gives the ordering kind of the three-way comparison
    /// operation: strong ordering, weak ordering, or partial ordering. Strong and
    /// weak orderings are both total orderings (i.e. every two elements are comparable),
    /// while partial orderings can have incomparable elements.
    ///
    /// Given the two input operands of the three-way comparison operation `lhs` and
    /// `rhs`, the `lt`, `eq`, `gt`, and `unordered` parameters gives the result
    /// value that should be produced by the three-way comparison operation when the
    /// ordering between `lhs` and `rhs` is `lhs < rhs`, `lhs == rhs`, `lhs > rhs`,
    /// or neither, respectively.
    ///
    /// Example:
    ///
    /// ```
    /// !s32i = !cir.int<s, 32>
    ///
    /// #cmpinfo_partial_ltn1eq0gt1unn127 = #cir.cmp3way_info<partial, lt = -1, eq = 0, gt = 1, unordered = -127>
    /// #cmpinfo_strong_ltn1eq0gt1 = #cir.cmp3way_info<strong, lt = -1, eq = 0, gt = 1>
    ///
    /// %0 = cir.const #cir.int<0> : !s32i
    /// %1 = cir.const #cir.int<1> : !s32i
    /// %2 = cir.cmp3way(%0 : !s32i, %1, #cmpinfo_strong_ltn1eq0gt1) : !s8i
    ///
    /// %3 = cir.const #cir.fp<0.0> : !cir.float
    /// %4 = cir.const #cir.fp<1.0> : !cir.float
    /// %5 = cir.cmp3way(%3 : !cir.float, %4, #cmpinfo_partial_ltn1eq0gt1unn127) : !s8
    /// ```
    CmpThreeWayInfo {
        /// an enum of type CmpOrdering
        ordering: crate::enums::CmpOrdering,
        lt: i64,
        eq: i64,
        gt: i64,
        unordered: Option<i64>,
    },
    /// `#cir.complex_range`
    /// complex multiplication and division implementation
    /// Enum cases:
    /// * full (`Full`)
    /// * improved (`Improved`)
    /// * promoted (`Promoted`)
    /// * basic (`Basic`)
    ComplexRangeKind {
        /// an enum of type ComplexRangeKind
        value: crate::enums::ComplexRangeKind,
    },
    /// `#cir.const_array`
    /// A constant array from ArrayAttr or StringRefAttr
    ///
    /// An CIR array attribute is an array of literals of the specified attr types.
    ConstArray { ty: crate::types::Type, elts: Box<Attribute>, trailing_zeros_num: i32 },
    /// `#cir.const_complex`
    /// An attribute that contains a constant complex value
    ///
    /// The `#cir.const_complex` attribute contains a constant value of complex
    /// number type. The `real` parameter gives the real part of the complex number
    /// and the `imag` parameter gives the imaginary part of the complex number.
    ///
    /// The `real` and `imag` parameters must both reference the same type and must
    /// be either IntAttr or FPAttr.
    ///
    /// ```
    /// %ci = #cir.const_complex<#cir.int<1> : !s32i, #cir.int<2> : !s32i>
    ///     : !cir.complex<!s32i>
    /// %cf = #cir.const_complex<#cir.fp<1.000000e+00> : !cir.float,
    ///     #cir.fp<2.000000e+00> : !cir.float> : !cir.complex<!cir.float>
    /// ```
    ConstComplex {
        ty: crate::types::Type,
        /// integer or floating point type
        real: Box<Attribute>,
        /// integer or floating point type
        imag: Box<Attribute>,
    },
    /// `#cir.ptr`
    /// Holds a constant pointer value
    ///
    /// A pointer attribute is a literal attribute that represents an integral
    /// value of a pointer type.
    ConstPtr { ty: crate::types::Type, value: Box<Attribute> },
    /// `#cir.const_record`
    /// Represents a constant record
    ///
    /// Effectively supports "struct-like" constants. It's must be built from
    /// an `mlir::ArrayAttr` instance where each element is a typed attribute
    /// (`mlir::TypedAttribute`).
    ///
    /// These must be initialized with a set of types that exactly match the type of
    /// the attribute, with one exception: flexible array members.  If the last
    /// member of the struct type is a zero-length array, the `ConstRecordAttr` is
    /// allowed to hold an unbounded number of elements. This is necessary to
    /// support global and static storage duration variables that use the flexible
    /// array member functionality.
    ///
    /// Example:
    /// ```
    /// cir.global external @rgb2 = #cir.const_record<{0 : i8,
    ///                                                5 : i64, #cir.null : !cir.ptr<i8>
    ///                                               }> : !cir.record<"", i8, i64, !cir.ptr<i8>>
    /// ```
    ConstRecord { ty: crate::types::Type, members: Box<Attribute> },
    /// `#cir.const_vector`
    /// A constant vector from ArrayAttr
    ///
    /// A CIR vector attribute is an array of literals of the specified attribute
    /// types.
    ConstVector { ty: crate::types::Type, elts: Box<Attribute> },
    /// `#cir.data_member`
    /// Holds a constant data member pointer value
    ///
    /// A data member attribute is a literal attribute that represents a constant
    /// pointer-to-data-member value.
    ///
    /// The `member_path` parameter is a GEP-like sequence of field indices
    /// navigating from `classTy` down to the pointed-to member.  An absent
    /// `member_path` represents a null pointer-to-data-member.
    ///
    /// Examples:
    /// ```
    /// // int Point::*p = &Point::z  (z is field 2)
    /// #cir.data_member<[2]> : !cir.data_member<!s32i in !rec_Point>
    ///
    /// // int Derived::*p = &Derived::x  (Base subobject at [0], x at [0])
    /// #cir.data_member<[0, 0]> : !cir.data_member<!s32i in !rec_Derived>
    ///
    /// // null
    /// #cir.data_member<null> : !cir.data_member<!s32i in !rec_Point>
    /// ```
    DataMember { ty: crate::types::Type, member_path: Option<String> },
    /// `#cir.data_member_offset`
    /// Constant pointer-to-data-member given by a byte offset
    ///
    /// Like `#cir.data_member`, this is a literal attribute representing a constant
    /// (non-null) pointer-to-data-member value.  It is used for members that are
    /// not laid out in the CIR record and therefore have no field-index path -- in
    /// particular `[[no_unique_address]]` empty fields.  The concrete byte offset
    /// of the member within the class is stored directly.
    ///
    /// Example:
    /// ```
    /// // Empty S::*p = &S::e  (e is a no_unique_address empty field at byte 1)
    /// #cir.data_member_offset<1> : !cir.data_member<!rec_Empty in !rec_S>
    /// ```
    DataMemberOffset { ty: crate::types::Type, offset: u64 },
    /// `#cir.dyn_cast_info`
    /// ABI specific information about a dynamic cast
    ///
    /// Provide ABI specific information about a dynamic cast operation.
    ///
    /// The `src_rtti` and the `dest_rtti` parameters give the RTTI of the source
    /// record type and the destination record type, respectively.
    ///
    /// The `runtime_func` parameter gives the `__dynamic_cast` function which is
    /// provided by the runtime. The `bad_cast_func` parameter gives the
    /// `__cxa_bad_cast` function which is also provided by the runtime.
    ///
    /// The `offset_hint` parameter gives the hint value that should be passed to
    /// the `__dynamic_cast` runtime function.
    DynamicCastInfo {
        /// Provides constant access to a global address
        src_rtti: Box<Attribute>,
        /// Provides constant access to a global address
        dest_rtti: Box<Attribute>,
        runtime_func: String,
        bad_cast_func: String,
        /// An attribute containing an integer value
        offset_hint: Box<Attribute>,
    },
    /// `#cir.dynamic_cast`
    /// dynamic cast kind
    /// Enum cases:
    /// * ptr (`Ptr`)
    /// * ref (`Ref`)
    DynamicCastKind {
        /// an enum of type DynamicCastKind
        value: crate::enums::DynamicCastKind,
    },
    /// `#cir.dynamic_init_guard`
    /// Guard variable name for guarded dynamic initialization
    ///
    /// Contains the mangled guard variable name for guarded dynamic
    /// initialization. This covers both function-local static variables and
    /// namespace-scope variables with vague linkage (e.g. weak/linkonce_odr
    /// inline variables), which both use the Itanium `__cxa_guard_*` protocol.
    /// See `DynamicInitInfoAttr` for the facts that distinguish the two.
    ///
    /// Example:
    /// ```
    /// cir.global internal dynamic_init_guard<"_ZGVZ3foovE1x"> @_ZZ3foovE1x = ...
    /// ```
    DynamicInitGuard { name: String },
    /// `#cir.dynamic_init_info`
    /// Variable facts needed by guarded dynamic initialization lowering
    ///
    /// Holds the subset of a guarded-dynamic-initialization variable's
    /// declaration facts that post-CIRGen lowering needs, as plain data (no
    /// `clang::VarDecl` pointer). Always accompanies a `DynamicInitGuardAttr`,
    /// and applies equally to function-local statics and to namespace-scope
    /// variables with vague linkage (e.g. weak/linkonce_odr inline variables).
    ///
    /// - `local`: whether the variable is a block-scope (local) declaration.
    ///   Mirrors `clang::VarDecl::isLocalVarDecl()`. The namespace-scope inline
    ///   variables that also reach guarded initialization are not local.
    ///
    /// - `tls`: the thread-local storage kind (`none`, `static`, or `dynamic`).
    ///
    /// - `is_inline`: whether the variable is inline. Mirrors
    ///   `clang::VarDecl::isInline()`.
    ///
    /// - `tsk`: the template-specialization kind (`undeclared`,
    ///   `implicit_instantiation`, `explicit_specialization`,
    ///   `explicit_instantiation_declaration`, or
    ///   `explicit_instantiation_definition`).
    ///
    /// Example:
    /// ```
    /// cir.global ... @x = ...
    ///   dynamic_init_info<local = true, tls = none, is_inline = false,
    ///                     tsk = undeclared>
    /// ```
    ///
    /// Because it stores plain data rather than a `clang::VarDecl` pointer, it
    /// round-trips through textual CIR and stays valid once the AST is gone.
    /// This is what lets LoweringPrepare consume these facts without a live
    /// `clang::ASTContext`; the `$ast` `ASTVarDeclAttr` remains a separate,
    /// live-AST handle for consumers that need arbitrary AST properties.
    DynamicInitInfo {
        local: bool,
        /// an enum of type TLSKind
        tls: crate::enums::TlsKind,
        is_inline: bool,
        /// an enum of type TemplateSpecializationKind
        tsk: crate::enums::TemplateSpecializationKind,
    },
    /// `#cir.eh_filter`
    /// Dynamic exception specification filter handler
    ///
    /// Identifies the handler of a `cir.try` operation that is reached when the
    /// in-flight exception is permitted by the dynamic exception specification of
    /// the enclosing function. The attribute carries the list of type info symbols
    /// naming the permitted types. An empty list represents `throw()`, which
    /// permits nothing.
    EhFilter { permitted_types: Box<Attribute> },
    /// `#cir.eh_unexpected`
    /// Dynamic exception specification unexpected exception handler
    ///
    /// Identifies the handler of a `cir.try` operation that is reached when the
    /// in-flight exception is not permitted by the dynamic exception specification
    /// of the enclosing function.
    EhUnexpected,
    /// `#cir.fp`
    /// An attribute containing a floating-point value
    ///
    /// An fp attribute is a literal attribute that represents a floating-point
    /// value of the specified floating-point type. Supporting only CIR FP types.
    CirFloat { ty: crate::types::Type, value: String },
    /// `#cir.fp_class`
    /// floating-point class test flags
    /// Enum cases:
    /// * fcNone (`None`)
    /// * fcSNan (`SignalingNaN`)
    /// * fcQNan (`QuietNaN`)
    /// * fcNegInf (`NegativeInfinity`)
    /// * fcNegNormal (`NegativeNormal`)
    /// * fcNegSubnormal (`NegativeSubnormal`)
    /// * fcNegZero (`NegativeZero`)
    /// * fcPosZero (`PositiveZero`)
    /// * fcPosSubnormal (`PositiveSubnormal`)
    /// * fcPosNormal (`PositiveNormal`)
    /// * fcPosInf (`PositiveInfinity`)
    /// * fcNan (`Nan`)
    /// * fcInf (`Infinity`)
    /// * fcNormal (`Normal`)
    /// * fcSubnormal (`Subnormal`)
    /// * fcZero (`Zero`)
    /// * fcPosFinite (`PositiveFinite`)
    /// * fcNegFinite (`NegativeFinite`)
    /// * fcFinite (`Finite`)
    /// * fcPositive (`Positive`)
    /// * fcNegative (`Negative`)
    /// * fcAllFlags (`All`)
    FpClassTest {
        /// an enum of type FPClassTest
        value: crate::enums::FpClassTest,
    },
    /// `#cir.fastmath`
    /// fast-math flags
    ///
    /// Describes fast-math flags for CIR operations. This attribute is shared by
    /// operations with floating-point semantics and is not specific to LLVM intrinsic
    /// calls.
    FastMathFlags {
        /// an enum of type FastMathFlags
        value: crate::enums::FastMathFlags,
    },
    /// `#cir.fenv`
    /// Describes floating-point environment constraints
    ///
    /// The `#cir.fenv` attribute describes constraints on the floating-point
    /// handling of a floating-point operation. It is attached to floating-point
    /// operations to capture rounding and exception behavior. All of its
    /// parameters are optional.
    ///
    /// - `dynamic_rounding_mode`: the known dynamic rounding mode at the point the
    ///   instruction is executed. Otherwise the behavior is undefined.
    /// - `except_mode`: the known exception mode at the point the instruction is
    ///   executed. Otherwise the behavior is undefined.
    /// - `strict_except`: if `false`, any case that would produce an FP exception
    ///   produces it non-deterministically instead (i.e. it may or may not occur).
    ///   This means the FP status is written non-deterministically, and, if
    ///   exceptions are unmasked, the instruction traps non-deterministically.
    ///
    /// An absent `dynamic_rounding_mode` defaults to `unknown`, an absent
    /// `except_mode` defaults to `masked`, and an absent `strict_except` defaults
    /// to `false`.
    ///
    /// Example:
    /// ```
    /// #cir.fenv<dynamic_rounding_mode = tonearest, except_mode = unknown,
    ///           strict_except = true>
    /// ```
    Fenv {
        dynamic_rounding_mode: Option<String>,
        except_mode: Option<String>,
        strict_except: Option<String>,
    },
    /// `#cir.func_identity`
    /// Identifies a function as a known standard library entity
    ///
    /// Names the standard library entity a function represents, so that
    /// transformations can recognize calls to well known library functions
    /// without decoding mangled symbol names.
    ///
    /// The tag names the whole entity. For `std::find` that is the free
    /// function named `find` in the `std` namespace, so a member function, a
    /// static member, or an operator can never carry the tag. Inline
    /// namespaces, such as the versioning namespace of libc++, count as part
    /// of `std`. The tag never encodes signatures, and a function that
    /// matches no known entity carries no attribute.
    ///
    /// Example:
    /// ```
    /// #cir.func_identity<"std::find">
    /// ```
    FuncIdentity {
        /// an enum of type KnownFuncKind
        kind: crate::enums::KnownFuncKind,
    },
    /// `#cir.global_ctor`
    /// Marks a function as a global constructor
    ///
    /// Marks the function as a global constructor in the module's constructor list.
    /// It will be executed before main() is called.
    GlobalCtor { name: String, priority: i32, associated: Option<String> },
    /// `#cir.global_dtor`
    /// Marks a function as a global destructor
    ///
    /// Marks a function as a global destructor in the module dtors list.
    /// The function will be executed before the module unloading.
    GlobalDtor { name: String, priority: i32, associated: Option<String> },
    /// `#cir.linkage`
    /// linkage kind
    /// Enum cases:
    /// * external (`ExternalLinkage`)
    /// * available_externally (`AvailableExternallyLinkage`)
    /// * linkonce (`LinkOnceAnyLinkage`)
    /// * linkonce_odr (`LinkOnceODRLinkage`)
    /// * weak (`WeakAnyLinkage`)
    /// * weak_odr (`WeakODRLinkage`)
    /// * appending (`AppendingLinkage`)
    /// * internal (`InternalLinkage`)
    /// * cir_private (`PrivateLinkage`)
    /// * extern_weak (`ExternalWeakLinkage`)
    /// * common (`CommonLinkage`)
    GlobalLinkageKind {
        /// an enum of type GlobalLinkageKind
        value: crate::enums::GlobalLinkageKind,
    },
    /// `#cir.global_offset`
    /// Provides constant access to a byte offset from a global
    ///
    /// Get the constant address that is `offset` bytes away from the address of
    /// global `symbol`. Like `#cir.global_view`, it provides a way to access
    /// globals from other globals and always produces a pointer, and the type of
    /// the referenced symbol may differ from the type of the attribute.
    ///
    /// Whereas `#cir.global_view` navigates the type of the symbol using
    /// subelement indices, this attribute describes a flat displacement in bytes.
    /// It is used for the addresses that cannot be described by subelement
    /// indices, namely addresses outside of the referenced object, such as the
    /// one-past-the-end address of a record, and addresses that do not fall on a
    /// subelement boundary. This mirrors the byte offset that classic codegen
    /// applies to every constant lvalue.
    ///
    /// Example:
    ///
    /// ```
    /// struct S { int x; };
    /// S s;
    /// S *p = &s + 1;
    /// ```
    ///
    /// produces
    ///
    /// ```
    /// cir.global external @p = #cir.global_offset<@s, 4> : !cir.ptr<!rec_S>
    /// ```
    ///
    /// which lowers to
    ///
    /// ```
    /// @p = global ptr getelementptr (i8, ptr @s, i64 4)
    /// ```
    GlobalOffset { ty: crate::types::Type, symbol: String, offset: i64 },
    /// `#cir.global_view`
    /// Provides constant access to a global address
    ///
    /// Get constant address of global `symbol` and optionally apply offsets to
    /// access existing subelements. It provides a way to access globals from other
    /// global and always produces a pointer.
    ///
    /// The type of the input symbol can be different from `#cir.global_view`
    /// output type, since a given view of the global might require a static
    /// cast for initializing other globals.
    ///
    /// A list of indices can be optionally passed and each element subsequently
    /// indexes underlying types. For `symbol` types like `!cir.array`
    /// and `!cir.record`, it leads to the constant address of sub-elements, while
    /// for `!cir.ptr`, an offset is applied. The first index is relative to the
    /// original symbol type, not the produced one.
    ///
    /// The result type of this attribute may be an integer type. In such a case,
    /// the pointer to the referenced global is casted to an integer and this
    /// attribute represents the casted result.
    ///
    /// Example:
    ///
    /// ```
    ///   cir.global external @s = @".str2": !cir.ptr<i8>
    ///   cir.global external @x = #cir.global_view<@s> : !cir.ptr<i8>
    ///   cir.global external @s_addr = #cir.global_view<@s> : !s64i
    ///
    ///   cir.global external @rgb = #cir.const_array<[0 : i8, -23 : i8, 33 : i8]
    ///                                                : !cir.array<i8 x 3>>
    ///   cir.global external @elt_ptr = #cir.global_view<@rgb, [1]> : !cir.ptr<i8>
    /// ```
    ///
    /// Note, that unlike LLVM IR's gep instruction, CIR doesn't add the leading
    /// zero index when it's known to be constant zero, e.g. for pointers, i.e. we
    /// use indexes exactly to access sub elements or for the offset. The leading
    /// zero index is added later in the lowering.
    ///
    /// Example:
    /// ```
    /// struct A {
    ///   int a;
    /// };
    ///
    /// struct B:  virtual A {
    ///   int b;
    /// };
    /// ```
    /// `address_point` marks a vtable address point: `symbol` is a vtable and
    /// `indices` are the vtable within the group and the address point within
    /// it. As in classic codegen, it lowers to an `inbounds` GEP whose `inrange`
    /// spans that one vtable.
    ///
    /// VTT for B in CIR:
    /// ```
    /// cir.global linkonce_odr @_ZTT1B = #cir.const_array<[
    ///           #cir.global_view<@_ZTV1B, [0 : i32, 3 : i32], address_point>
    ///                : !cir.ptr<!u8i>]> : !cir.array<!cir.ptr<!u8i> x 1>
    /// ```
    /// VTT for B in LLVM IR:
    /// ```
    /// @_ZTT1B = linkonce_odr global [1 x ptr] [ptr getelementptr inbounds
    ///           inrange(-24, 0) (i8, ptr @_ZTV1B, i64 24)], align 8
    /// ```
    GlobalView {
        ty: crate::types::Type,
        symbol: String,
        indices: Option<Box<Attribute>>,
        address_point: bool,
    },
    /// `#cir.init_catch`
    /// allowed i32 cases: 0, 1, 2, 3, 4, 5, 6, 7
    /// Enum cases:
    /// * reference (`Reference`)
    /// * pointer (`Pointer`)
    /// * scalar (`Scalar`)
    /// * objc (`Objc`)
    /// * trivial_copy (`TrivialCopy`)
    /// * non_trivial_copy (`NonTrivialCopy`)
    /// * reference_to_pointer (`ReferenceToPointer`)
    /// * reference_to_record_pointer (`ReferenceToRecordPointer`)
    InitCatchKind {
        /// an enum of type InitCatchKind
        value: crate::enums::InitCatchKind,
    },
    /// `#cir.inline`
    /// inline kind
    /// Inline Kind attributes.  `no_inline` and `always_inline`
    ///    spellings correspond to the attributes of the same name, and `inline_hint`
    ///      is the `inline` keyword in the language.
    InlineKind {
        /// an enum of type InlineKind
        value: crate::enums::InlineKind,
    },
    /// `#cir.int`
    /// An attribute containing an integer value
    ///
    /// An integer attribute is a literal attribute that represents an integral
    /// value of the specified integer type.
    CirInt { ty: crate::types::Type, value: String },
    /// `#cir.lang_address_space`
    /// Represents a language address space
    ///
    /// Encodes the semantic address spaces defined by the front-end language
    /// (e.g. `__shared__`, `__constant__`, `__local__`). Values are stored using the
    /// `cir::LangAddressSpace` enum, keeping the representation compact and
    /// preserving the qualifier until it is mapped onto target/LLVM address-space
    /// numbers.
    ///
    /// Example:
    /// ```
    /// !cir.ptr<!s32i, lang_address_space(offload_local)>
    /// cir.global constant external lang_address_space(offload_constant)
    /// ```
    LangAddressSpace {
        /// an enum of type LangAddressSpace
        value: crate::enums::LangAddressSpace,
    },
    /// `#cir.lowering_lang_options`
    /// LangOptions facts that post-CIRGen lowering consumes
    ///
    /// Serializes the subset of `clang::LangOptions` that post-CIRGen lowering
    /// reads, as plain data, so a reloaded `.cir` module lowers the same way it
    /// was compiled without a live `clang::LangOptions`. Only LangOpts flags with
    /// no triple/module representation live here; target-derived facts (size_t
    /// width, CXXABI, fp formats, etc.) are rebuilt from `cir.triple` instead.
    ///
    /// Each bool field mirrors the like-named `LangOptions` member: `exceptions`,
    /// `threadsafe_statics`, `cuda`, `cuda_is_device`, `hip`, `gpu_rdc`,
    /// `openmp` (`LangOpts.OpenMP != 0`; only its on/off state matters), and
    /// `openmp_is_target_device`. `clang_abi_compat` is an `i32` mirroring
    /// `LangOptions::ClangABI`; it is an `i32` rather than a CIR enum because
    /// `ClangABI` gains an enumerator every Clang release (see
    /// `clang/Basic/ABIVersions.def`), and the ordinal round-trips fine within a
    /// single toolchain.
    ///
    /// Example:
    /// ```
    /// module attributes {cir.lowering_lang_options = #cir.lowering_lang_options<
    ///   exceptions = false, threadsafe_statics = true, cuda = false,
    ///   cuda_is_device = false, hip = false, gpu_rdc = false, openmp = false,
    ///   openmp_is_target_device = false, clang_abi_compat = 17>} { }
    /// ```
    LoweringLangOptions {
        exceptions: bool,
        threadsafe_statics: bool,
        cuda: bool,
        cuda_is_device: bool,
        hip: bool,
        gpu_rdc: bool,
        openmp: bool,
        openmp_is_target_device: bool,
        clang_abi_compat: i32,
    },
    /// `#cir.max_work_group_size`
    /// Maximum kernel work-group size
    ///
    /// Stores the maximum work-group size of a kernel, emitted to LLVM IR as
    /// `max_work_group_size` function metadata.
    MaxWorkGroupSize { x: u32, y: u32, z: u32 },
    /// `#cir.mem_order`
    /// Memory order according to C++11 memory model
    /// Enum cases:
    /// * relaxed (`Relaxed`)
    /// * consume (`Consume`)
    /// * acquire (`Acquire`)
    /// * release (`Release`)
    /// * acq_rel (`AcquireRelease`)
    /// * seq_cst (`SequentiallyConsistent`)
    MemOrder {
        /// an enum of type MemOrder
        value: crate::enums::MemOrder,
    },
    /// `#cir.memory_effects`
    /// Describes the possible memory effects of a function or callee
    ///
    /// Specifies the possible memory effects of a function or callee.  Memory
    /// is divided into classes, each carrying its own access, so that a promise
    /// about one class need not be a promise about all of them:
    ///
    /// - `other`: the access for every location not named by one of the other
    ///   parameters, including locations with no class of their own such as
    ///   globals and captured pointers.
    /// - `arg_mem`: accesses based on the function's pointer arguments.
    /// - `inaccessible_mem`: accesses to memory the current module cannot reach,
    ///   often used to model the control dependencies of intrinsics.
    /// - `errno_mem`: accesses to the `errno` variable.
    /// - `target_mem0`, `target_mem1`: target-specific state.
    ///
    /// The classes and their meanings match the LLVM dialect's
    /// `#llvm.memory_effects`, which lowering converts this attribute to.
    ///
    /// An absent attribute means the effects are unknown, which is the
    /// conservative default and what a function with no memory annotation gets.
    /// Lowering leaves such a function with no memory attribute at all.
    ///
    /// Example:
    /// ```
    /// #cir.memory_effects<other = read, arg_mem = readwrite,
    ///                     inaccessible_mem = none, errno_mem = none,
    ///                     target_mem0 = none, target_mem1 = none>
    /// ```
    ///
    /// The attribute has no keyword syntax, so on an operation it is written in
    /// the attribute dictionary:
    ///
    /// ```
    /// cir.func private @f() attributes {memory_effects = #cir.memory_effects<...>}
    /// %0 = cir.call @f() {memory_effects = #cir.memory_effects<...>} : () -> !s32i
    /// ```
    MemoryEffects {
        other: String,
        arg_mem: String,
        inaccessible_mem: String,
        errno_mem: String,
        target_mem0: String,
        target_mem1: String,
    },
    /// `#cir.method`
    /// Holds a constant pointer-to-member-function value
    ///
    /// A method attribute is a literal attribute that represents a constant
    /// pointer-to-member-function value.
    ///
    /// If the member function is a non-virtual function, the `symbol` parameter
    /// gives the global symbol for the non-virtual member function.
    ///
    /// If the member function is a virtual function, the `vtable_offset` parameter
    /// gives the offset of the vtable entry corresponding to the virtual member
    /// function.
    ///
    /// `symbol` and `vtable_offset` cannot be present at the same time. If both of
    /// `symbol` and `vtable_offset` are not present, the attribute represents a
    /// null pointer constant.
    ///
    /// Examples:
    /// ```
    /// // Non-virtual method
    /// %0 = cir.const #cir.method<@_ZN1S2m1Ei> :
    ///          !cir.method<!cir.func<(!s32i)> in !rec_S>
    ///
    /// // Virtual method
    /// %1 = cir.const #cir.method<vtable_offset = 8> :
    ///          !cir.method<!cir.func<(!s32i)> in !rec_S>
    ///
    /// // Null method pointer
    /// %0 = cir.const #cir.method<null> :
    ///          !cir.method<!cir.func<(!s32i)> in !rec_S>
    /// ```
    Method {
        ty: crate::types::Type,
        symbol: Option<String>,
        vtable_offset: Option<u64>,
    },
    /// `#cir.offload`
    /// Offload kind of a module in an offload container
    ///
    /// Marks a module nested inside an offload container - identified by the
    /// `cir.offload.container` unit attribute - as either the host module (carrying
    /// `cir.offload.kind = #cir.offload<host>`) or one of the device modules
    /// (carrying `cir.offload.kind = #cir.offload<device>`) of the offload
    /// container. This creates a tiered module layout, where a singular offload
    /// container contains a single host module and one or more device modules,
    /// granting the offload merge/split pipeline passes the cross-boundary view.
    ///
    /// Example:
    /// ```mlir
    /// module attributes {cir.offload.container} {
    ///   module @host attributes {cir.offload.kind = #cir.offload<host>} {}
    ///   module @device1 attributes {cir.offload.kind = #cir.offload<device>} {}
    ///   module @device2 attributes {cir.offload.kind = #cir.offload<device>} {}
    /// }
    /// ```
    ///
    /// Note: In order to simplify offload passes, the offload container module
    /// requires that the first module inside it is the host module and the rest
    /// of the modules are device modules.
    ///
    /// The nested modules are plain `builtin.module` ops rather than a dedicated
    /// container op, so that the existing module infrastructure applies to them
    /// unchanged; the layout above is enforced by the CIR dialect attribute
    /// verifier, following the `gpu.container_module` precedent.
    ///
    /// The attribute names `cir.offload.container` and `cir.offload.kind` are
    /// defined by the `getOffloadContainerAttrName` and `getOffloadKindAttrName`
    /// methods in the CIRDialect class.
    OffloadKind {
        /// an enum of type OffloadKind
        value: crate::enums::OffloadKind,
    },
    /// `#cir.offset_pair`
    /// Offset Pair attribute
    ///
    /// An attribute that specifies a pair of positive integral values that
    /// represent a bit-offset range in an existing structure.  The range
    /// represented by this is [start, end).
    ///
    /// Example:
    /// ```
    /// cir.clear_padding(align(1) %1,
    ///                   [#cir.offset_pair<0, 2>, #cir.offset_pair<6, 8>])
    ///                   : <rec_Type> -> ()
    /// ```
    OffsetPair { start: u64, end: u64 },
    /// `#cir.cl.kernel_arg_metadata`
    /// OpenCL kernel argument metadata
    ///
    /// Stores the OpenCL kernel argument metadata emitted to LLVM IR as
    /// `kernel_arg_*` metadata.
    ///
    /// All parameters are arrays containing the argument information in source
    /// order. The `name` field is optional and is emitted only when requested by
    /// `-cl-kernel-arg-info`.
    OpenClKernelArgMetadata {
        /// language address space array attribute
        addr_space: Box<Attribute>,
        /// string array attribute
        access_qual: Box<Attribute>,
        /// string array attribute
        ty: Box<Attribute>,
        /// string array attribute
        base_type: Box<Attribute>,
        /// string array attribute
        type_qual: Box<Attribute>,
        /// string array attribute or null
        name: Box<Attribute>,
    },
    /// `#cir.cl.version`
    /// OpenCL version
    ///
    /// Represents an OpenCL language version preserved as module metadata.
    ///
    /// The `cir.cl.cxx.version` module attribute requires a companion
    /// `cir.cl.version` attribute specifying the compatible OpenCL version.
    ///
    /// Example:
    /// ```
    /// // Module compiled from OpenCL C 1.2.
    /// module attributes {cir.cl.version = #cir.cl.version<1, 2>} {}
    /// // Module compiled from C++ for OpenCL 2021.
    /// module attributes {
    ///   cir.cl.cxx.version = #cir.cl.version<2021, 0>,
    ///   cir.cl.version = #cir.cl.version<3, 0>
    /// } {}
    /// ```
    OpenClVersion { major: i32, minor: i32 },
    /// `#cir.opt_info`
    /// A module-level attribute that holds the optimization information
    ///
    /// The `#cir.opt_info` attribute holds optimization related information. For
    /// now this attribute is a module-level attribute that gets attached to the
    /// module operation during CIRGen.
    ///
    /// The `level` parameter gives the optimization level. It must be an integer
    /// between 0 and 3, inclusive. It corresponds to the `OptimizationLevel` field
    /// within the `clang::CodeGenOptions` structure.
    ///
    /// The `size` parameter gives the code size optimization level. It must be an
    /// integer between 0 and 2, inclusive. It corresponds to the `OptimizeSize`
    /// field within the `clang::CodeGenOptions` structure.
    ///
    /// The `level` and `size` parameters correspond to the optimization level
    /// command line options passed to clang driver. The table below lists the
    /// current correspondance relationship:
    ///
    /// | Flag             | `level` | `size` |
    /// |------------------|---------|--------|
    /// | `-O0` or nothing | 0       | 0      |
    /// | `-O1`            | 1       | 0      |
    /// | `-O2`            | 2       | 0      |
    /// | `-O3`            | 3       | 0      |
    /// | `-Os`            | 2       | 1      |
    /// | `-Oz`            | 2       | 2      |
    ///
    /// Examples:
    ///
    /// ```
    /// #cir.opt_info<level = 2, size = 0>  // -O2
    /// ```
    OptInfo { level: u32, size: u32 },
    /// `#cir.poison`
    /// Represent a typed poison constant
    ///
    /// The PoisonAttr represents a typed poison constant, corresponding to LLVM's
    /// notion of poison.
    Poison { ty: crate::types::Type },
    /// `#cir.ptr_spec`
    /// !cir.ptr data layout spec
    ///
    /// Pointer data layout for `!cir.ptr`: `size`, `abi` and `preferred` are
    /// required bitwidths; `index` is the optional bitwidth for index
    /// computations, defaulting to `size`. All present values must be divisible
    /// by 8, with `preferred` >= `abi`.
    ///
    /// This is the CIR-native analogue of the ptr dialect's `#ptr.spec`; a
    /// native attribute keeps CIR's data-layout queries free of any ptr-dialect
    /// dependency.
    ///
    /// Used as the value of the `!cir.ptr<!cir.void>`-keyed data-layout entry:
    ///
    /// ```mlir
    /// #dlti.dl_spec<!cir.ptr<!cir.void> =
    ///     #cir.ptr_spec<size = 64, abi = 64, preferred = 64>>
    /// ```
    PtrSpec { size: u32, abi: u32, preferred: u32, index: u32 },
    /// `#cir.record_layout`
    /// ABI layout metadata for a record type
    ///
    /// Holds AST-derived ABI metadata for a named record type.  These
    /// properties are translation-unit / target properties, not intrinsic
    /// to the type, so they live on the module rather than on RecordType.
    ///
    /// Fields:
    /// - `arg_passing_kind`: whether the record can be passed in registers
    ///   per the C++ ABI (mirrors `RecordDecl::getArgPassingRestrictions()`).
    /// - `has_trivial_destructor`: from `CXXRecordDecl::hasTrivialDestructor()`.
    /// - `record_align_in_bytes`: from `ASTRecordLayout::getAlignment()`.
    ///   Needed because CIR's DataLayout cannot account for
    ///   `__attribute__((aligned(N)))`.
    ///
    /// Example:
    /// ```
    /// module attributes {
    ///   cir.record_layouts = {
    ///     "Trivial" = #cir.record_layout<
    ///       arg_passing_kind = can_pass_in_regs,
    ///       has_trivial_dtor = true,
    ///       record_align = 4>,
    ///     "NonTrivialDtor" = #cir.record_layout<
    ///       arg_passing_kind = cannot_pass_in_regs,
    ///       has_trivial_dtor = false,
    ///       record_align = 4>
    ///   }
    /// }
    /// ```
    RecordLayout {
        /// an enum of type ArgPassingKind
        arg_passing_kind: crate::enums::ArgPassingKind,
        has_trivial_dtor: bool,
        record_align: u64,
    },
    /// `#cir.lang`
    /// Module source language
    ///
    /// Represents the source language used to generate the module.
    ///
    /// Example:
    /// ```
    /// // Module compiled from C.
    /// module attributes {cir.lang = cir.lang<c>} {}
    /// // Module compiled from C++.
    /// module attributes {cir.lang = cir.lang<cxx>} {}
    /// // Module compiled from OpenCL C.
    /// module attributes {cir.lang = cir.lang<opencl_c>} {}
    /// // Module compiled from C++ for OpenCL.
    /// module attributes {cir.lang = cir.lang<opencl_cxx>} {}
    /// ```
    ///
    /// Module source language attribute name is `cir.lang` is defined by
    /// `getSourceLanguageAttrName` method in CIRDialect class.
    SourceLanguage {
        /// an enum of type SourceLanguage
        value: crate::enums::SourceLanguage,
    },
    /// `#cir.sync_scope`
    /// sync scope kind
    /// Enum cases:
    /// * single_thread (`SingleThread`)
    /// * system (`System`)
    /// * device (`Device`)
    /// * workgroup (`Workgroup`)
    /// * wavefront (`Wavefront`)
    /// * cluster (`Cluster`)
    /// * hip_single_thread (`HIPSingleThread`)
    /// * hip_system (`HIPSystem`)
    /// * hip_agent (`HIPAgent`)
    /// * hip_workgroup (`HIPWorkgroup`)
    /// * hip_wavefront (`HIPWavefront`)
    /// * hip_cluster (`HIPCluster`)
    /// * opencl_work_group (`OpenCLWorkGroup`)
    /// * opencl_device (`OpenCLDevice`)
    /// * opencl_all_svm_devices (`OpenCLAllSVMDevices`)
    /// * opencl_sub_group (`OpenCLSubGroup`)
    SyncScopeKind {
        /// an enum of type SyncScopeKind
        value: crate::enums::SyncScopeKind,
    },
    /// `#cir.tls_model`
    /// TLS model
    ///
    ///  The TLS mode for the global, which comes from either the
    /// `tls_model` attribute, or `-ftls-model` flag.
    TlsModel {
        /// an enum of type TLSModel
        value: crate::enums::TlsModel,
    },
    /// `#cir.target_address_space`
    /// Represents a target-specific numeric address space
    ///
    /// The TargetAddressSpaceAttr represents a target-specific numeric address space,
    /// corresponding to the LLVM IR `addrspace` qualifier and the clang
    ///  `address_space` attribute.
    ///
    /// A value of zero represents the default address space. The semantics of non-zero
    /// address spaces are target-specific.
    ///
    /// Example:
    /// ```
    /// // Target-specific numeric address spaces
    /// !cir.ptr<!s32i, addrspace(target<1>)>
    /// !cir.ptr<!s32i, addrspace(target<10>)>
    /// ```
    TargetAddressSpace { value: u32 },
    /// `#cir.tls_wrapper_init`
    /// Wrapper and Init function names for thread local variables
    ///
    /// Contains the mangled name of the wrapper function, init function, and
    /// guard variable for a namespace/global scope thread local variable. The
    /// guard variable is optional, as it is only required for unordered thread
    /// local variables, as ordered thread local variables share a guard.
    ///
    /// Unordered global thread local variables (such as variable template
    /// instantiations) are individually initialized when first used on a thread.
    /// Ordered global thread local variables are ALL initialized together when
    /// any that require initialization are referenced.
    ///
    /// This is accomplished by rewriting all calls to these variables as calls to
    /// the wrapper.  If the variable requires initialization, the wrapper calls
    /// the init function, then returns the global variable reference.
    ///
    /// Throughout CIR though, these are just represented as normal `get_global`
    /// calls to `global`s with `ctor`/`dtor` regions (if necessary).  The
    /// lowering-prepare pass manages the generation of the wrapper,x
    /// initialization, and call rewrites.
    ///
    /// Example:
    /// ```
    /// cir.global tls_dyn dyn_tls_refs = <"_ZTW7tls_var", "_ZTH7tls_var"> @_ZZZ7tls_var = ...
    /// ...
    /// cir.get_global thread_local @ZZZ7tls_var : !cir.ptr<!s32i>
    /// ```
    ThreadLocalGlobalWrapperInit {
        wrapper_name: String,
        init_name: String,
        guard_name: Option<String>,
    },
    /// `#cir.typeinfo`
    /// Represents a typeinfo used for RTTI
    ///
    /// The typeinfo data for a given class is stored into an ArrayAttr. The
    /// layout is determined by the C++ ABI used (clang only implements
    /// itanium on CIRGen).
    ///
    /// The verifier enforces that the output type is always a `!cir.record`,
    /// and that the ArrayAttr element types match the equivalent member type
    /// for the resulting record, i.e, a GlobalViewAttr for symbol reference or
    /// an IntAttr for flags.
    ///
    /// Example:
    ///
    /// ```
    /// cir.global "private" external @_ZTVN10__cxxabiv120__si_class_type_infoE
    ///   : !cir.ptr<i32>
    ///
    /// !rec_anon_struct = !cir.record<struct  {!cir.ptr<!u8i>, !cir.ptr<!u8i>,
    ///   !cir.ptr<!u8i>}>
    ///
    /// cir.global constant external @type_info = #cir.typeinfo<{
    ///   #cir.global_view<@_ZTVN10__cxxabiv120__si_class_type_infoE, [2 : i32]>
    ///   : !cir.ptr<!u8i>, #cir.global_view<@_ZTS1B> : !cir.ptr<!u8i>,
    ///   #cir.global_view<@_ZTI1A> : !cir.ptr<!u8i>}> : !rec_anon_struct
    /// ```
    TypeInfo {
        ty: crate::types::Type,
        /// integer or global view array attribute
        data: Box<Attribute>,
    },
    /// `#cir.undef`
    /// Represent an undef constant
    ///
    /// The UndefAttr represents an undef constant, corresponding to LLVM's notion
    /// of undef.
    Undef { ty: crate::types::Type },
    /// `#cir.unwind`
    Unwind,
    /// `#cir.uwtable`
    /// Encodes the unwind table kind for a function.
    ///
    /// This mirrors the UWTableKindEnum in The LLVM dialect,
    /// and is passed through to the llvm dialect. According to the LLVM
    /// documentation:
    ///     The ‘uwtable’ attribute is about the ABI or the user mandating an /
    ///         entry in the unwind table.
    UnwindTableKind {
        /// an enum of type UnwindTableKind
        value: crate::enums::UnwindTableKind,
    },
    /// `#cir.usual_delete_params`
    /// Parameters describing the usual operator delete signature
    ///
    /// Captures the properties of the usual deallocation function associated with
    /// an operator delete. These mirror the fields of `clang::UsualDeleteParams`.
    UsualDeleteParams {
        size: bool,
        alignment: Option<u64>,
        type_aware_delete: bool,
        destroying_delete: bool,
    },
    /// `#cir.vtable`
    /// Represents a C++ vtable
    ///
    /// Wraps a #cir.const_record containing one or more vtable arrays.
    ///
    /// In most cases, the anonymous record type wrapped by this attribute will
    /// contain a single array corresponding to the vtable for one class. However,
    /// in the case of multiple inheritence, the anonymous structure may contain
    /// multiple arrays, each of which is a vtable.
    ///
    /// Example 1 (single vtable):
    /// ```
    /// cir.global linkonce_odr @_ZTV6Mother =
    ///   #cir.vtable<{
    ///     #cir.const_array<[
    ///       #cir.ptr<null> : !cir.ptr<!u8i>,
    ///       #cir.global_view<@_ZTI6Mother> : !cir.ptr<!u8i>,
    ///       #cir.global_view<@_ZN6Mother9MotherFooEv> : !cir.ptr<!u8i>,
    ///       #cir.global_view<@_ZN6Mother10MotherFoo2Ev> : !cir.ptr<!u8i>
    ///     ]> : !cir.array<!cir.ptr<!u8i> x 4>
    ///   }> : !rec_anon_struct1
    /// ```
    ///
    /// Example 2 (multiple vtables):
    /// ```
    /// cir.global linkonce_odr @_ZTV5Child =
    ///   #cir.vtable<{
    ///     #cir.const_array<[
    ///       #cir.ptr<null> : !cir.ptr<!u8i>,
    ///       #cir.global_view<@_ZTI5Child> : !cir.ptr<!u8i>,
    ///       #cir.global_view<@_ZN5Child9MotherFooEv> : !cir.ptr<!u8i>,
    ///       #cir.global_view<@_ZN6Mother10MotherFoo2Ev> : !cir.ptr<!u8i>
    ///     ]> : !cir.array<!cir.ptr<!u8i> x 4>,
    ///     #cir.const_array<[
    ///       #cir.ptr<-8 : i64> : !cir.ptr<!u8i>,
    ///       #cir.global_view<@_ZTI5Child> : !cir.ptr<!u8i>,
    ///       #cir.global_view<@_ZN6Father9FatherFooEv> : !cir.ptr<!u8i>
    ///     ]> : !cir.array<!cir.ptr<!u8i> x 3>
    ///   }> : !rec_anon_struct2
    /// ```
    VTable { ty: crate::types::Type, data: Box<Attribute> },
    /// `#cir.zero`
    /// Attribute to represent zero initialization
    ///
    /// The ZeroAttr is used to indicate zero initialization on structs.
    Zero { ty: crate::types::Type },
    Unit,
    Bool(bool),
    Int { value: i128, ty: Option<crate::types::Type> },
    Float { text: String, ty: Option<crate::types::Type> },
    Str(String),
    Array(Vec<Attribute>),
    Dict(Vec<(String, Attribute)>),
    SymbolRef(String),
    Type(crate::types::Type),
    Named(String),
    Dialect {
        dialect: String,
        mnemonic: String,
        raw: Option<String>,
        ty: Option<crate::types::Type>,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConstArrayData {
    Str(Vec<u8>),
    Elements(Vec<Attribute>),
}
impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unit => write!(f, "unit"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::Int { value, .. } => write!(f, "{value}"),
            Self::Float { text, .. } => write!(f, "{text}"),
            Self::Str(s) => write!(f, "{s:?}"),
            Self::Array(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{item}")?;
                }
                write!(f, "]")
            }
            Self::Dict(entries) => {
                write!(f, "{{")?;
                for (i, (k, v)) in entries.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{k} = {v}")?;
                }
                write!(f, "}}")
            }
            Self::SymbolRef(s) => write!(f, "@{s}"),
            Self::Type(t) => write!(f, "{t}"),
            Self::Named(n) => write!(f, "#{n}"),
            Self::CirInt { value, .. } | Self::CirFloat { value, .. } => {
                write!(f, "{value}")
            }
            Self::CirBool { value, .. } => write!(f, "{value}"),
            Self::ConstArray { elts, .. } => write!(f, "[{elts}]"),
            Self::ConstVector { elts, .. } => write!(f, "[{elts}]"),
            Self::ConstRecord { members, .. } => write!(f, "{{{members}}}"),
            Self::ConstComplex { real, imag, .. } => write!(f, "({real}, {imag})"),
            Self::GlobalView { symbol, indices, .. } => {
                write!(f, "@{symbol}")?;
                if let Some(indices) = indices {
                    write!(f, "[{indices}]")?;
                }
                Ok(())
            }
            Self::Zero { .. } => write!(f, "zero"),
            Self::Poison { .. } => write!(f, "poison"),
            Self::Dialect { dialect, mnemonic, raw, .. } => {
                write!(f, "#{dialect}.{mnemonic}")?;
                if let Some(raw) = raw {
                    write!(f, "<{raw}>")?;
                }
                Ok(())
            }
            other => write!(f, "{other:?}"),
        }
    }
}
impl Attribute {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Str(s) | Self::SymbolRef(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_type(&self) -> Option<&crate::types::Type> {
        match self {
            Self::Type(ty) => Some(ty),
            _ => None,
        }
    }
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) | Self::CirBool { value: b, .. } => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i128> {
        match self {
            Self::Int { value, .. } => Some(*value),
            Self::CirInt { value, .. } => value.parse().ok(),
            _ => None,
        }
    }
    pub fn as_dense_array_ints(&self) -> Option<Vec<i128>> {
        let Self::Dialect { dialect, mnemonic, raw: Some(raw), .. } = self else {
            return None;
        };
        if dialect != "builtin" || mnemonic != "array" {
            return None;
        }
        let digits = raw.split_once(':').map_or(raw.as_str(), |(_, rest)| rest);
        digits.split(',').map(|part| part.trim().parse::<i128>().ok()).collect()
    }
}