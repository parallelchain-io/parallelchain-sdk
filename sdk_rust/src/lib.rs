/*
 Copyright (c) 2022 ParallelChain Lab

 This program is free software: you can redistribute it and/or modify
 it under the terms of the GNU General Public License as published by
 the Free Software Foundation, either version 3 of the License, or
 (at your option) any later version.

 This program is distributed in the hope that it will be useful,
 but WITHOUT ANY WARRANTY; without even the implied warranty of
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 GNU General Public License for more details.

 You should have received a copy of the GNU General Public License
 along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

mod exports;
mod imports;

pub(crate) mod transaction;
pub use crate::transaction::Transaction;

mod context;
pub use self::context::{CallDataBuilder, ContractCallData, Storage, StorageField, Callback};


pub use pchain_sdk_macros::{
    contract_init,
    sdk_method_bindgen,
    contract,
    use_contract,
    view, action, init,
};

pub use pchain_sdk_macros::ContractField;

// associated function to perform conversion of contract address to PublicAddress 
pub fn decode_contract_address(address: String) -> protocol_types::PublicAddress {
    protocol_types::Base64URL::decode(&address).unwrap().try_into().unwrap()
}

use borsh::{BorshSerialize, BorshDeserialize};
/// Convert Borsh-serializable structure to bytes
pub fn convert_bytes<T: BorshSerialize>(data :&T) -> Vec<u8> {
    let mut bs :Vec<u8> = vec![];
    data.serialize(&mut bs).unwrap();
    bs
}

/// Convert Borsh-serialized bytes to option of the structure
pub fn convert_from<T: BorshDeserialize>(bytes :&Vec<u8>) -> Option<T>{
    let bs = bytes.clone();
    let deserialize_ret = BorshDeserialize::deserialize(&mut bs.as_ref());
    if !deserialize_ret.is_ok() { return None; }
    let ret : T = deserialize_ret.unwrap();
    Some(ret)
}

// Precompiles API
pub mod precompile {
    use crate::imports;

    pub fn random() -> u64 {
        unsafe {
            imports::random()
        }
    }
    pub fn sha256(input :Vec<u8>) -> Vec<u8>{
        let input = input.clone();
        let input_ptr = input.as_ptr();

        let mut val_ptr: u32 = 0;
        let val_ptr_ptr = &mut val_ptr;

        let value;
        unsafe {
            let val_len = crate::imports::sha256(input_ptr, input.len() as u32, val_ptr_ptr);
            value = Vec::<u8>::from_raw_parts(val_ptr as *mut u8, val_len as usize, val_len as usize);
        }

        value
    }

    pub fn keccak256(input :Vec<u8>) -> Vec<u8>{
        let input = input.clone();
        let input_ptr = input.as_ptr();

        let mut val_ptr: u32 = 0;
        let val_ptr_ptr = &mut val_ptr;

        let value;
        unsafe {
            let val_len = crate::imports::keccak256(input_ptr, input.len() as u32, val_ptr_ptr);
            value = Vec::<u8>::from_raw_parts(val_ptr as *mut u8, val_len as usize, val_len as usize);
        }

        value
    }

    pub fn keccak512(input :Vec<u8>) -> Vec<u8>{
        let input = input.clone();
        let input_ptr = input.as_ptr();

        let mut val_ptr: u32 = 0;
        let val_ptr_ptr = &mut val_ptr;

        let value;
        unsafe {
            let val_len = crate::imports::keccak512(input_ptr, input.len() as u32, val_ptr_ptr);
            value = Vec::<u8>::from_raw_parts(val_ptr as *mut u8, val_len as usize, val_len as usize);
        }

        value
    }

    pub fn ripemd160(input :Vec<u8>) -> Vec<u8>{
        let input = input.clone();
        let input_ptr = input.as_ptr();

        let mut val_ptr: u32 = 0;
        let val_ptr_ptr = &mut val_ptr;

        let value;
        unsafe {
            let val_len = crate::imports::ripemd160(input_ptr, input.len() as u32, val_ptr_ptr);
            value = Vec::<u8>::from_raw_parts(val_ptr as *mut u8, val_len as usize, val_len as usize);
        }

        value
    }

    pub fn blake2b(input :Vec<u8>, output_len: u32) -> Vec<u8>{
        let input = input.clone();
        let input_ptr = input.as_ptr();

        let mut val_ptr: u32 = 0;
        let val_ptr_ptr = &mut val_ptr;

        let value;
        unsafe {
            let val_len = crate::imports::blake2b(input_ptr, input.len() as u32, output_len, val_ptr_ptr);
            value = Vec::<u8>::from_raw_parts(val_ptr as *mut u8, val_len as usize, val_len as usize);
        }

        value
    }

    pub fn verify_signature(input :Vec<u8>, signature: Vec<u8>, address :Vec<u8>) -> bool {
        let input = input.clone();
        let input_ptr = input.as_ptr();

        let signature = signature.clone();
        let signature_ptr = signature.as_ptr();

        let address = address.clone();
        let address_ptr = address.as_ptr();

        let value;
        unsafe {
            value = crate::imports::verify_signature(input_ptr, input.len() as u32, signature_ptr, address_ptr);
        }

        value != 0
    }
}
