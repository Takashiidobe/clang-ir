//! Array construction and destruction operations.

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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArrayCtor {
    /// array or element address
    pub addr: super::ValueId,
    /// integer type
    pub num_elements: Option<super::ValueId>,
    pub body: super::Region,
    pub partial_dtor: super::Region,
    pub loc: Option<crate::ast::SourceLocation>,
}
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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArrayDtor {
    /// array or element address
    pub addr: super::ValueId,
    /// integer type
    pub num_elements: Option<super::ValueId>,
    /// unit property
    pub dtor_may_throw: bool,
    pub body: super::Region,
    pub loc: Option<crate::ast::SourceLocation>,
}