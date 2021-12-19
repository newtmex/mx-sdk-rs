use crate::{tx_mock::TxPanic, DebugApi};
use alloc::vec::Vec;
use elrond_wasm::{
    api::{EndpointArgumentApi, EndpointArgumentApiImpl, Handle},
    types::BoxedBytes,
};
use num_bigint::{BigInt, BigUint, Sign};
use num_traits::cast::ToPrimitive;

impl EndpointArgumentApi for DebugApi {
    type EndpointArgumentApiImpl = DebugApi;

    fn argument_api_impl() -> Self::EndpointArgumentApiImpl {
        DebugApi::new_from_static()
    }
}

/// Interface to only be used by code generated by the macros.
/// The smart contract code doesn't have access to these methods directly.
impl EndpointArgumentApiImpl for DebugApi {
    fn get_num_arguments(&self) -> i32 {
        self.input_ref().args.len() as i32
    }

    fn get_argument_len(&self, arg_index: i32) -> usize {
        let arg = self.get_argument_vec_u8(arg_index);
        arg.len()
    }

    fn copy_argument_to_slice(&self, _arg_index: i32, _slice: &mut [u8]) {
        panic!("copy_argument_to_slice not yet implemented")
    }

    fn get_argument_vec_u8(&self, arg_index: i32) -> Vec<u8> {
        let arg_idx_usize = arg_index as usize;
        assert!(
            arg_idx_usize < self.input_ref().args.len(),
            "Tx arg index out of range"
        );
        self.input_ref().args[arg_idx_usize].clone()
    }

    fn get_argument_boxed_bytes(&self, arg_index: i32) -> BoxedBytes {
        self.get_argument_vec_u8(arg_index).into()
    }

    fn get_argument_big_uint_raw(&self, arg_index: i32) -> Handle {
        let arg_bytes = self.get_argument_boxed_bytes(arg_index);
        let mut managed_types = self.m_types_borrow_mut();
        let result = BigInt::from_bytes_be(Sign::Plus, arg_bytes.as_slice());
        managed_types.big_int_map.insert_new_handle(result)
    }

    fn get_argument_big_int_raw(&self, arg_index: i32) -> i32 {
        let arg_bytes = self.get_argument_boxed_bytes(arg_index);
        let mut managed_types = self.m_types_borrow_mut();
        let result = BigInt::from_signed_bytes_be(arg_bytes.as_slice());
        managed_types.big_int_map.insert_new_handle(result)
    }

    fn get_argument_managed_buffer_raw(&self, arg_index: i32) -> Handle {
        let arg_bytes = self.get_argument_boxed_bytes(arg_index);
        let mut managed_types = self.m_types_borrow_mut();
        managed_types
            .managed_buffer_map
            .insert_new_handle(arg_bytes.as_slice().into())
    }

    fn get_argument_i64(&self, arg_index: i32) -> i64 {
        let bytes = self.get_argument_vec_u8(arg_index);
        let bi = BigInt::from_signed_bytes_be(&bytes);
        if let Some(v) = bi.to_i64() {
            v
        } else {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: b"argument out of range".to_vec(),
            })
        }
    }

    fn get_argument_u64(&self, arg_index: i32) -> u64 {
        let bytes = self.get_argument_vec_u8(arg_index);
        let bu = BigUint::from_bytes_be(&bytes);
        if let Some(v) = bu.to_u64() {
            v
        } else {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: b"argument out of range".to_vec(),
            })
        }
    }
}
