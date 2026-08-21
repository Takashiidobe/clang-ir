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
    /// `#cir.cu.binary_handle`
    /// Fat binary handle for device code.
    ///
    /// This attribute is attached to the ModuleOp and records the binary file
    /// name passed to host.
    ///
    /// CUDA first compiles device-side code into a fat binary file. The file
    /// name is then passed into host-side code, which is used to create a handle
    /// and then generate various registration functions.
    CudaBinaryHandle { name: String },
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
    /// `#cir.all`
    CatchAll,
    /// `#cir.cleanup_kind`
    /// Cleanup kind attribute
    ///
    /// Cleanup kind attributes.
    CleanupKind {
        /// an enum of type CleanupKind
        value: crate::enums::CleanupKind,
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
    /// `#cir.fp`
    /// An attribute containing a floating-point value
    ///
    /// An fp attribute is a literal attribute that represents a floating-point
    /// value of the specified floating-point type. Supporting only CIR FP types.
    CirFloat { ty: crate::types::Type, value: String },
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
    GlobalCtor { name: String, priority: i32 },
    /// `#cir.global_dtor`
    /// Marks a function as a global destructor
    ///
    /// Marks a function as a global destructor in the module dtors list.
    /// The function will be executed before the module unloading.
    GlobalDtor { name: String, priority: i32 },
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
    /// VTT for B in CIR:
    /// ```
    /// cir.global linkonce_odr @_ZTT1B = #cir.const_array<[
    ///           #cir.global_view<@_ZTV1B, [0 : i32, 3 : i32]> : !cir.ptr<!u8i>]>
    ///                : !cir.array<!cir.ptr<!u8i> x 1>
    /// ```
    /// VTT for B in LLVM IR:
    /// ```
    /// @_ZTT1B = linkonce_odr global [1 x ptr] [ptr getelementptr inbounds
    ///           ({ [3 x ptr] }, ptr @_ZTV1B, i32 0, i32 0, i32 3)], align 8
    /// ```
    GlobalView {
        ty: crate::types::Type,
        symbol: String,
        indices: Option<Box<Attribute>>,
    },
    /// `#cir.inline_kind`
    /// Inline kind attribute
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
    /// ```
    ///
    /// Module source language attribute name is `cir.lang` is defined by
    /// `getSourceLanguageAttrName` method in CIRDialect class.
    SourceLanguage {
        /// an enum of type SourceLanguage
        value: crate::enums::SourceLanguage,
    },
    /// `#cir.static_local_guard`
    /// Guard variable name for static local variables
    ///
    /// Contains the mangled guard variable name for static local variable
    /// initialization.
    ///
    /// Example:
    /// ```
    /// cir.global internal static_local_guard<"_ZGVZ3foovE1x"> @_ZZ3foovE1x = ...
    /// ```
    StaticLocalGuard { name: String },
    /// `#cir.tls_model`
    /// TLS Model attribute
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
    /// `#cir.visibility`
    /// Visibility attribute
    ///
    /// Visibility attributes.
    Visibility {
        /// an enum of type VisibilityKind
        value: crate::enums::VisibilityKind,
    },
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