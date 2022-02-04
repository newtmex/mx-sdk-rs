use crate::{
    api::{ErrorApi, ErrorApiImpl, ManagedTypeApi, StorageWriteApi, StorageWriteApiImpl},
    err_msg,
    types::{BigInt, BigUint, ManagedBuffer, ManagedBufferCachedBuilder, ManagedRef, ManagedType},
};
use elrond_codec::*;

use super::StorageKey;

struct StorageSetOutput<'k, A>
where
    A: StorageWriteApi + ManagedTypeApi + ErrorApi + 'static,
{
    key: ManagedRef<'k, A, StorageKey<A>>,
}

impl<'k, A> StorageSetOutput<'k, A>
where
    A: StorageWriteApi + ManagedTypeApi + ErrorApi + 'static,
{
    #[inline]
    fn new(key: ManagedRef<'k, A, StorageKey<A>>) -> Self {
        StorageSetOutput { key }
    }

    fn set_managed_buffer(&self, managed_buffer: &ManagedBuffer<A>) {
        A::storage_write_api_impl().storage_store_managed_buffer_raw(
            self.key.buffer.get_raw_handle(),
            managed_buffer.handle,
        );
    }
}

impl<'k, A> TopEncodeOutput for StorageSetOutput<'k, A>
where
    A: StorageWriteApi + ManagedTypeApi + ErrorApi + 'static,
{
    type NestedBuffer = ManagedBufferCachedBuilder<A>;

    fn set_slice_u8(self, bytes: &[u8]) {
        self.set_managed_buffer(&bytes.into())
    }

    fn set_u64(self, value: u64) {
        using_encoded_number(value, 64, false, true, |bytes| {
            self.set_managed_buffer(&bytes.into())
        });
    }

    fn set_i64(self, value: i64) {
        using_encoded_number(value as u64, 64, true, true, |bytes| {
            self.set_managed_buffer(&bytes.into())
        });
    }

    #[inline]
    fn supports_specialized_type<T: TryStaticCast>() -> bool {
        T::type_eq::<ManagedBuffer<A>>() || T::type_eq::<BigUint<A>>() || T::type_eq::<BigInt<A>>()
    }

    #[inline]
    fn set_specialized<T, H>(self, value: &T, h: H) -> Result<(), H::HandledErr>
    where
        T: TryStaticCast,
        H: EncodeErrorHandler,
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
            Err(h.handle_error(EncodeError::UNSUPPORTED_OPERATION))
        }
    }

    fn start_nested_encode(&self) -> Self::NestedBuffer {
        ManagedBufferCachedBuilder::new_from_slice(&[])
    }

    fn finalize_nested_encode(self, nb: Self::NestedBuffer) {
        self.set_managed_buffer(&nb.into_managed_buffer());
    }
}

// #[inline]
pub fn storage_set<A, T>(key: ManagedRef<'_, A, StorageKey<A>>, value: &T)
where
    T: TopEncode,
    A: StorageWriteApi + ManagedTypeApi + ErrorApi,
{
    value.top_encode_or_exit(StorageSetOutput::new(key), (), storage_set_exit::<A>);
}

/// Useful for storage mappers.
/// Also calls to it generated by macro.
pub fn storage_clear<A>(key: ManagedRef<'_, A, StorageKey<A>>)
where
    A: StorageWriteApi + ManagedTypeApi + ErrorApi,
{
    A::storage_write_api_impl().storage_store_managed_buffer_clear(key.get_raw_handle());
}

#[inline(always)]
fn storage_set_exit<A>(_: (), encode_err: EncodeError) -> !
where
    A: StorageWriteApi + ManagedTypeApi + ErrorApi + 'static,
{
    let mut message_buffer = ManagedBuffer::<A>::new_from_bytes(err_msg::STORAGE_ENCODE_ERROR);
    message_buffer.append_bytes(encode_err.message_bytes());
    A::error_api_impl().signal_error_from_buffer(message_buffer.get_raw_handle())
}
