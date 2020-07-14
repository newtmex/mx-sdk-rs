#![no_std]

// re-export basic heap types
extern crate alloc;
pub use alloc::boxed::Box;
pub use alloc::vec::Vec;
pub use alloc::string::String;

pub use elrond_codec;

mod types;
pub mod io;
mod proxy;
pub mod storage;
pub mod err_msg;
pub mod call_data;

pub use types::*;
pub use io::*;
pub use storage::{storage_get, storage_set, BorrowedMutStorage};
pub use finish::SCResult;
pub use call_data::*;
pub use proxy::OtherContractHandle;

use core::ops::{Add, Sub, Mul, Div, Rem, Neg};
use core::ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
use core::ops::{BitAnd, BitOr, BitXor, Shr, Shl};
use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShrAssign, ShlAssign};

/// Interface to be used by the actual smart contract code.
/// 
/// Note: contracts and the api are not mutable.
/// They simply pass on/retrieve data to/from the protocol.
/// When mocking the blockchain state, we use the Rc/RefCell pattern 
/// to isolate mock state mutability from the contract interface.
pub trait ContractHookApi<BigInt, BigUint>: Sized
where
    BigInt: elrond_codec::Encode + 'static,
    BigUint: elrond_codec::Encode + 'static,
{

    fn get_sc_address(&self) -> Address;

    fn get_owner_address(&self) -> Address;

    fn get_caller(&self) -> Address;

    fn get_balance(&self, address: &Address) -> BigUint;

    fn get_sc_balance(&self) -> BigUint {
        self.get_balance(&self.get_sc_address())
    }
    
    fn storage_store(&self, key: &[u8], value: &[u8]);

    fn storage_load(&self, key: &[u8]) -> Vec<u8>;

    fn storage_load_len(&self, key: &[u8]) -> usize;

    fn storage_store_bytes32(&self, key: &[u8], value: &[u8; 32]);
    
    fn storage_load_bytes32(&self, key: &[u8]) -> [u8; 32];

    fn storage_store_big_uint(&self, key: &[u8], value: &BigUint);
    
    fn storage_load_big_uint(&self, key: &[u8]) -> BigUint;

    fn storage_store_big_int(&self, key: &[u8], value: &BigInt);
    
    fn storage_load_big_int(&self, key: &[u8]) -> BigInt;

    fn storage_store_i64(&self, key: &[u8], value: i64);
    
    fn storage_load_i64(&self, key: &[u8]) -> Option<i64>;

    #[inline]
    fn storage_load_cumulated_validator_reward(&self) -> BigUint {
        self.storage_load_big_uint(storage::protected_keys::ELROND_REWARD_KEY)
    }
    
    fn get_call_value_big_uint(&self) -> BigUint;

    fn send_tx(&self, to: &Address, amount: &BigUint, message: &str);

    fn async_call(&self, to: &Address, amount: &BigUint, data: &[u8]);

    fn get_tx_hash(&self) -> H256;

    fn get_gas_left(&self) -> i64;

    fn get_block_timestamp(&self) -> u64;

    fn get_block_nonce(&self) -> u64;

    fn get_block_round(&self) -> u64;
    
    fn get_block_epoch(&self) -> u64;

    fn sha256(&self, data: &[u8]) -> [u8; 32];

    fn keccak256(&self, data: &[u8]) -> [u8; 32];
}

macro_rules! get_argument_signed_cast {
    ($method_name:ident, $type:ty) => {
        fn $method_name (&self, arg_id: i32) -> $type {
            let arg_i64 = self.get_argument_i64(arg_id);
            let min = <$type>::MIN as i64;
            let max = <$type>::MAX as i64;
            if arg_i64 < min || arg_i64 > max {
                self.signal_error(err_msg::ARG_OUT_OF_RANGE)
            }
            arg_i64 as $type
        }
  };
}

macro_rules! get_argument_unsigned_cast {
    ($method_name:ident, $type:ty) => {
        fn $method_name (&self, arg_id: i32) -> $type {
            let arg_i64 = self.get_argument_u64(arg_id);
            let min = <$type>::MIN as u64;
            let max = <$type>::MAX as u64;
            if arg_i64 < min || arg_i64 > max {
                self.signal_error(err_msg::ARG_OUT_OF_RANGE)
            }
            arg_i64 as $type
        }
  };
}

/// Interface to only be used by code generated by the macros.
/// The smart contract code doesn't have access to these methods directly.
pub trait ContractIOApi<BigInt, BigUint> {

    fn get_num_arguments(&self) -> i32;

    fn check_num_arguments(&self, expected: i32) -> bool {
        let nr_args = self.get_num_arguments();
        if nr_args != expected {
            self.signal_error(err_msg::ARG_WRONG_NUMBER);
        }
        true
    }

    fn check_not_payable(&self);

    fn get_argument_len(&self, arg_index: i32) -> usize;

    fn copy_argument_to_slice(&self, arg_index: i32, slice: &mut [u8]);

    fn get_argument_vec(&self, arg_index: i32) -> Vec<u8>;

    fn get_argument_bytes32(&self, arg_index: i32) -> [u8; 32];
    
    fn get_argument_address(&self, arg_index: i32) -> Address {
        self.get_argument_bytes32(arg_index).into()
    }
    
    fn get_argument_big_int(&self, arg_id: i32) -> BigInt;

    fn get_argument_big_uint(&self, arg_id: i32) -> BigUint;
    
    // signed
    fn get_argument_i64(&self, arg_id: i32) -> i64;
    get_argument_signed_cast!{get_argument_i32, i32}
    get_argument_signed_cast!{get_argument_isize, isize}
    get_argument_signed_cast!{get_argument_i8, i8}

    // unsigned
    fn get_argument_u64(&self, arg_id: i32) -> u64 {
        let bytes = self.get_argument_vec(arg_id);
        if bytes.len() > 8 {
            self.signal_error(err_msg::ARG_OUT_OF_RANGE);
        }
        elrond_codec::bytes_to_number(bytes.as_slice(), false)
    }
    get_argument_unsigned_cast!{get_argument_u32, u32}
    get_argument_unsigned_cast!{get_argument_usize, usize}
    get_argument_unsigned_cast!{get_argument_u8, u8}

    fn get_argument_bool (&self, arg_id: i32) -> bool {
        let arg_i64 = self.get_argument_i64(arg_id);
        match arg_i64 {
            1 => true,
            0 => false,
            _ => self.signal_error(err_msg::ARG_WRONG_NUMBER)
        }
    }
    
    fn finish_slice_u8(&self, slice: &[u8]);

    fn finish_bytes32(&self, bytes: &[u8; 32]);

    fn finish_big_int(&self, b: &BigInt);

    fn finish_big_uint(&self, b: &BigUint);

    fn finish_i64(&self, value: i64);

    fn signal_error(&self, message: &[u8]) -> !;

    fn write_log(&self, topics: &[[u8;32]], data: &[u8]);
}

/// Definition of the BigUint type required by the API.
/// The API doesn't care about the actual BigInt implementation.
/// The Arwen VM provides an implementation directly in the protocol.
/// For debugging we use a different implementation, based on Rust's BigInt.
/// 
/// Since most values in smart contracts will not be signed, as well as for safety,
/// most of the functionality if provided for unsigned integers.
pub trait BigUintApi: 
    Sized +
    From<u64> +
    From<u32> +
    From<usize> +
    Clone +
    Add<Output=Self> + 
    AddAssign + 
    Sub<Output=Self> + 
    SubAssign +
    Mul<Output=Self> +
    MulAssign +
    Div<Output=Self> +
    DivAssign +
    Rem<Output=Self> +
    RemAssign +
    BitAnd<Output=Self> +
    BitAndAssign +
    BitOr<Output=Self> +
    BitOrAssign +
    BitXor<Output=Self> +
    BitXorAssign +
    Shr<usize, Output=Self> +
    ShrAssign<usize> +
    Shl<usize, Output=Self> +
    ShlAssign<usize> +
    PartialEq<Self> +
    Eq +
    PartialOrd<Self> +
    Ord +
    PartialEq<u64> +
    PartialOrd<u64> +
    elrond_codec::Encode +
    elrond_codec::Decode +
{
    fn zero() -> Self {
        0u64.into()
    }

    fn byte_length(&self) -> i32;

    fn copy_to_slice_big_endian(&self, slice: &mut [u8]) -> i32;

    fn copy_to_array_big_endian_pad_right(&self, target: &mut [u8; 32]);

    fn to_bytes_be(&self) -> Vec<u8>;

    fn to_bytes_be_pad_right(&self, nr_bytes: usize) -> Option<Vec<u8>>;

    fn from_bytes_be(bytes: &[u8]) -> Self;
}

// BigInt sign.
pub enum Sign {
    Minus,
    NoSign,
    Plus,
}

/// Definition of the BigInt type required by the API.
pub trait BigIntApi<BigUint>: 
        Sized +
        From<BigUint> +
        From<i64> +
        From<i32> +
        Clone +
        Add<Output=Self> + 
        AddAssign + 
        Sub<Output=Self> + 
        SubAssign +
        Mul<Output=Self> +
        MulAssign +
        Div<Output=Self> +
        DivAssign +
        Rem<Output=Self> +
        RemAssign +
        Neg +
        PartialEq<Self> +
        Eq +
        PartialOrd<Self> +
        Ord +
        PartialEq<i64> +
        PartialOrd<i64> +
        elrond_codec::Encode +
        elrond_codec::Decode +
{
    fn zero() -> Self {
        0i64.into()
    }

    fn abs_uint(&self) -> BigUint;

    fn sign(&self) -> Sign;

    fn to_signed_bytes_be(&self) -> Vec<u8>;

    fn from_signed_bytes_be(bytes: &[u8]) -> Self;
}

/// CallableContract is the means by which the debugger calls methods in the contract.
pub trait CallableContract {
    fn call(&self, fn_name: &'static str);

    fn clone_contract(&self) -> Box<dyn CallableContract>;
}

/// Handy way of casting to a contract proxy trait.
/// Would make more sense to be in elrond-wasm-derive, but Rust "cannot export macro_rules! macros from a `proc-macro` crate type currently".
#[macro_export]
macro_rules! contract_proxy {
    ($s:expr, $address:expr, $proxy_trait:ident) => {
      $s.contract_proxy($address) as Box<dyn $proxy_trait<BigInt, BigUint>>
  };
}

/// Getting all imports needed for a smart contract.
#[macro_export]
macro_rules! imports {
    () => {
        use elrond_wasm::{Box, Vec, String, Queue, VarArgs, BorrowedMutStorage};
        use elrond_wasm::{SCError, SCResult, SCResult::Ok, SCResult::Err};
        use elrond_wasm::{H256, Address, StorageKey, ErrorMessage};
        use elrond_wasm::{ContractHookApi, ContractIOApi, BigIntApi, BigUintApi, OtherContractHandle, AsyncCallResult, AsyncCallError};
        use elrond_wasm::elrond_codec::{Encode, Decode, DecodeError};
        use elrond_wasm::io::*;
        use elrond_wasm::err_msg;
        use core::ops::{Add, Sub, Mul, Div, Rem};
        use core::ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
        use core::ops::{BitAnd, BitOr, BitXor, Shr, Shl};
        use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShrAssign, ShlAssign};
  };
}

/// Compact way of returning a static error message.
#[macro_export]
macro_rules! sc_error {
    ($s:expr) => {
        elrond_wasm::SCResult::Err(elrond_wasm::SCError::Static($s.as_bytes()))
    }
}

/// Equivalent of the ? operator for SCResult.
#[macro_export]
macro_rules! sc_try {
    ($s:expr) => {
        match $s {
            elrond_wasm::SCResult::Ok(t) => t,
            elrond_wasm::SCResult::Err(e) => { return elrond_wasm::SCResult::Err(e); }
        }
        
    }
}

/// Compact way of returning a static error message.
#[macro_export]
macro_rules! mut_storage (
    ($t:ty) => (
        BorrowedMutStorage<'_, T, BigInt, BigUint, $t>
    )
);
