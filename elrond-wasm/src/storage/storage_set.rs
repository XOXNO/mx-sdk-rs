use crate::api::{ErrorApi, ManagedTypeApi, StorageWriteApi};
use crate::types::{BigInt, BigUint, ManagedBuffer};
use crate::*;
use elrond_codec::*;

use super::StorageKey;

struct StorageSetOutput<'k, A>
where
    A: StorageWriteApi + ManagedTypeApi + ErrorApi + 'static,
{
    api: A,
    key: &'k StorageKey<A>,
}

impl<'k, A> StorageSetOutput<'k, A>
where
    A: StorageWriteApi + ManagedTypeApi + ErrorApi + 'static,
{
    #[inline]
    fn new(api: A, key: &'k StorageKey<A>) -> Self {
        StorageSetOutput { api, key }
    }

    fn set_managed_buffer(&self, managed_buffer: &ManagedBuffer<A>) {
        self.api.storage_store_managed_buffer_raw(
            self.key.buffer.get_raw_handle(),
            managed_buffer.handle,
        );
    }
}

impl<'k, A> TopEncodeOutput for StorageSetOutput<'k, A>
where
    A: StorageWriteApi + ManagedTypeApi + ErrorApi + 'static,
{
    type NestedBuffer = ManagedBuffer<A>;

    fn set_slice_u8(self, bytes: &[u8]) {
        let key_bytes = self.key.to_boxed_bytes();
        self.api.storage_store_slice_u8(key_bytes.as_slice(), bytes)
    }

    fn set_u64(self, value: u64) {
        let key_bytes = self.key.to_boxed_bytes();
        self.api.storage_store_u64(key_bytes.as_slice(), value);
    }

    fn set_i64(self, value: i64) {
        let key_bytes = self.key.to_boxed_bytes();
        self.api.storage_store_i64(key_bytes.as_slice(), value);
    }

    #[inline]
    fn set_specialized<T, F>(self, value: &T, else_serialization: F) -> Result<(), EncodeError>
    where
        T: TryStaticCast,
        F: FnOnce(Self) -> Result<(), EncodeError>,
    {
        if let Some(managed_buffer) = value.try_cast_ref::<ManagedBuffer<A>>() {
            self.set_managed_buffer(managed_buffer);
            Ok(())
        } else if let Some(big_uint) = value.try_cast_ref::<BigUint<A>>() {
            self.set_managed_buffer(&big_uint.to_bytes_be_buffer());
            Ok(())
        } else if let Some(big_int) = value.try_cast_ref::<BigInt<A>>() {
            self.set_managed_buffer(&big_int.to_signed_bytes_be_buffer());
            Ok(())
        } else {
            else_serialization(self)
        }
    }

    fn start_nested_encode(&self) -> Self::NestedBuffer {
        ManagedBuffer::new_empty(self.api.clone())
    }

    fn finalize_nested_encode(self, nb: Self::NestedBuffer) {
        self.set_managed_buffer(&nb);
    }
}

// #[inline]
pub fn storage_set<A, T>(api: A, key: &StorageKey<A>, value: &T)
where
    T: TopEncode,
    A: StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
{
    value.top_encode_or_exit(
        StorageSetOutput::new(api.clone(), key),
        api,
        storage_set_exit,
    );
}

/// Useful for storage mappers.
/// Also calls to it generated by macro.
pub fn storage_clear<A>(api: A, key: &StorageKey<A>)
where
    A: StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
{
    api.storage_store_managed_buffer_clear(key.buffer.get_raw_handle());
}

#[inline(always)]
fn storage_set_exit<A>(api: A, encode_err: EncodeError) -> !
where
    A: StorageWriteApi + ManagedTypeApi + ErrorApi + 'static,
{
    let mut message_buffer =
        ManagedBuffer::new_from_bytes(api.clone(), err_msg::STORAGE_ENCODE_ERROR);
    message_buffer.append_bytes(encode_err.message_bytes());
    api.signal_error_from_buffer(message_buffer.get_raw_handle())
}
