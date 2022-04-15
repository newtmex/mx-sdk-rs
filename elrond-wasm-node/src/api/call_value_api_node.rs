use super::VmApiImpl;
use elrond_wasm::{
    api::{CallValueApi, CallValueApiImpl, Handle, StaticVarApiImpl},
    types::{EsdtTokenType, ManagedType, TokenIdentifier},
};

const MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH: usize = 32;

extern "C" {
    #[allow(dead_code)]
    fn bigIntNew(value: i64) -> i32;

    fn checkNoPayment();

    fn bigIntGetCallValue(dest: i32);

    #[cfg(not(feature = "ei-unmanaged"))]
    fn managedGetMultiESDTCallValue(resultHandle: i32);

    fn getNumESDTTransfers() -> i32;

    // single ESDT transfer
    fn bigIntGetESDTCallValue(dest: i32);
    fn getESDTTokenName(resultOffset: *const u8) -> i32;
    fn getESDTTokenNonce() -> i64;
    fn getESDTTokenType() -> i32;

    // ESDT by index
    fn bigIntGetESDTCallValueByIndex(dest: i32, index: i32);
    fn getESDTTokenNameByIndex(resultOffset: *const u8, index: i32) -> i32;
    fn getESDTTokenNonceByIndex(index: i32) -> i64;
    fn getESDTTokenTypeByIndex(index: i32) -> i32;
}

impl CallValueApi for VmApiImpl {
    type CallValueApiImpl = VmApiImpl;

    #[inline]
    fn call_value_api_impl() -> Self::CallValueApiImpl {
        VmApiImpl {}
    }
}

impl CallValueApiImpl for VmApiImpl {
    #[inline]
    fn check_not_payable(&self) {
        unsafe {
            checkNoPayment();
        }
    }

    fn load_egld_value(&self, dest: Handle) {
        unsafe {
            bigIntGetCallValue(dest);
        }
    }

    #[cfg(not(feature = "ei-unmanaged"))]
    fn load_all_esdt_transfers(&self, dest_handle: Handle) {
        unsafe {
            managedGetMultiESDTCallValue(dest_handle);
        }
    }

    fn esdt_num_transfers(&self) -> usize {
        unsafe { getNumESDTTransfers() as usize }
    }

    fn load_single_esdt_value(&self, dest: Handle) {
        unsafe {
            bigIntGetESDTCallValue(dest);
        }
    }

    fn token(&self) -> Handle {
        unsafe {
            let mut name_buffer = [0u8; MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH];
            let name_len = getESDTTokenName(name_buffer.as_mut_ptr());
            if name_len == 0 {
                TokenIdentifier::<Self>::egld().get_raw_handle()
            } else {
                TokenIdentifier::<Self>::from_esdt_bytes(&name_buffer[..name_len as usize])
                    .get_raw_handle()
            }
        }
    }

    fn esdt_token_nonce(&self) -> u64 {
        unsafe { getESDTTokenNonce() as u64 }
    }

    fn esdt_token_type(&self) -> EsdtTokenType {
        unsafe { (getESDTTokenType() as u8).into() }
    }

    fn esdt_value_by_index(&self, index: usize) -> Handle {
        unsafe {
            let value_handle = self.next_bigint_handle();
            bigIntGetESDTCallValueByIndex(value_handle, index as i32);
            value_handle
        }
    }

    fn token_by_index(&self, index: usize) -> Handle {
        unsafe {
            let mut name_buffer = [0u8; MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH];
            let name_len = getESDTTokenNameByIndex(name_buffer.as_mut_ptr(), index as i32);
            if name_len == 0 {
                TokenIdentifier::<Self>::egld().get_raw_handle()
            } else {
                TokenIdentifier::<Self>::from_esdt_bytes(&name_buffer[..name_len as usize])
                    .get_raw_handle()
            }
        }
    }

    fn esdt_token_nonce_by_index(&self, index: usize) -> u64 {
        unsafe { getESDTTokenNonceByIndex(index as i32) as u64 }
    }

    fn esdt_token_type_by_index(&self, index: usize) -> EsdtTokenType {
        unsafe { (getESDTTokenTypeByIndex(index as i32) as u8).into() }
    }
}
