mod system_sc_issue;
mod system_sc_special_roles;

use crate::{tx_mock::{TxContext, TxResult}, types::VMAddress};
use hex_literal::hex;
use system_sc_issue::*;
use system_sc_special_roles::*;

/// Address of the system smart contract that manages ESDT.
/// Bech32: erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u
pub const ESDT_SYSTEM_SC_ADDRESS_ARRAY: [u8; 32] =
    hex!("000000000000000000010000000000000000000000000000000000000002ffff");

pub fn is_system_sc_address(address: &VMAddress) -> bool {
    address.as_array() == &ESDT_SYSTEM_SC_ADDRESS_ARRAY
}

pub fn execute_system_sc(tx_context: TxContext) -> (TxContext, TxResult) {
    let func_name = tx_context.tx_input_box.func_name.clone();
    match func_name.as_str() {
        "issue" => issue(tx_context),
        "issueSemiFungible" => issue_semi_fungible(tx_context),
        "issueNonFungible" => issue_non_fungible(tx_context),
        "registerMetaESDT" => todo!(),
        "changeSFTToMetaESDT" => todo!(),
        "registerAndSetAllRoles" => todo!(),
        "ESDTBurn" => todo!(),
        "mint" => todo!(),
        "freeze" => todo!(),
        "unFreeze" => todo!(),
        "wipe" => todo!(),
        "pause" => todo!(),
        "unPause" => todo!(),
        "freezeSingleNFT" => todo!(),
        "unFreezeSingleNFT" => todo!(),
        "wipeSingleNFT" => todo!(),
        "claim" => todo!(),
        "configChange" => todo!(),
        "controlChanges" => todo!(),
        "transferOwnership" => todo!(),
        "getTokenProperties" => todo!(),
        "getSpecialRoles" => todo!(),
        "setSpecialRole" => set_special_role(tx_context),
        "unSetSpecialRole" => todo!(),
        "transferNFTCreateRole" => todo!(),
        "stopNFTCreate" => todo!(),
        "getAllAddressesAndRoles" => todo!(),
        "getContractConfig" => todo!(),
        "changeToMultiShardCreate" => todo!(),
        "setBurnRoleGlobally" => todo!(),
        "unsetBurnRoleGlobally" => todo!(),
        "sendAllTransferRoleAddresses" => todo!(),
        s => panic!("invalid system SC function: {s}"),
    }
}
