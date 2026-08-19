//! Vtable and RTTI operations.

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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VtableAddressPoint {
    pub addr: super::ValueId,
    pub addr_ty: crate::types::Type,
    /// flat symbol reference attribute
    pub name: crate::attrs::Attribute,
    /// Address point attribute
    pub address_point: crate::attrs::Attribute,
    pub loc: Option<crate::ast::SourceLocation>,
}
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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VtableGetTypeInfo {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// CIR type that is used for the vptr member of C++ objects
    pub vptr: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VtableGetVirtualFnAddr {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// CIR type that is used for the vptr member of C++ objects
    pub vptr: super::ValueId,
    /// 64-bit signless integer attribute
    pub index: crate::attrs::Attribute,
    pub loc: Option<crate::ast::SourceLocation>,
}
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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VtableGetVptr {
    pub result: super::ValueId,
    pub result_ty: crate::types::Type,
    /// CIR pointer type
    pub src: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VttAddressPoint {
    pub addr: super::ValueId,
    pub addr_ty: crate::types::Type,
    /// flat symbol reference attribute
    pub name: Option<crate::attrs::Attribute>,
    /// CIR void type or CIR bool type or CIR array type or CIR vector type or Integer type with arbitrary precision up to a fixed limit or single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type or CIR pointer type or CIR function type or CIR struct/class type or CIR union type or CIR complex type or CIR type that is used for the vptr member of C++ objects or CIR type that represents a pointer-to-data-member in C++ or CIR type that represents C++ pointer-to-member-function type or CIR exception handling token type or CIR cleanup token type or CIR catch token type
    pub sym_addr: Option<super::ValueId>,
    /// 32-bit signless integer attribute
    pub offset: crate::attrs::Attribute,
    pub loc: Option<crate::ast::SourceLocation>,
}