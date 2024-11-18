use crate::api::{HandleConstraints, ManagedTypeApi, RawHandle};

use super::ManagedRef;

/// Commonalities between all managed types.
pub trait ManagedType<M: ManagedTypeApi>: Sized {
    type OwnHandle: HandleConstraints;

    #[doc(hidden)]
    unsafe fn from_handle(handle: Self::OwnHandle) -> Self;

    fn get_handle(&self) -> Self::OwnHandle;

    /// Forgets current object (does not run destructor), but extracts the handle.
    /// 
    /// The handle remains an owned object, so the handle's destructor will run later, when dropped.
    unsafe fn forget_into_handle(self) -> Self::OwnHandle;

    #[doc(hidden)]
    unsafe fn from_raw_handle(handle: RawHandle) -> Self {
        Self::from_handle(Self::OwnHandle::new(handle))
    }

    fn get_raw_handle(&self) -> RawHandle {
        self.get_handle().cast_or_signal_error::<M, _>()
    }

    /// Implement carefully, since the underlying transmutation is an unsafe operation.
    /// For types that wrap a handle to some VM-managed data,
    /// make sure the type only contains the handle (plus ZSTs if necessary).
    /// For types that just wrap another managed type it is easier, call for the wrapped object.
    fn transmute_from_handle_ref(handle_ref: &Self::OwnHandle) -> &Self;

    fn transmute_from_handle_ref_mut(handle_ref: &mut Self::OwnHandle) -> &mut Self;

    fn as_ref(&self) -> ManagedRef<'_, M, Self> {
        ManagedRef::new(self)
    }
}
