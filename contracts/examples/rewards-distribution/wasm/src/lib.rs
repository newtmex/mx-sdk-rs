// Code generated by the elrond-wasm multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Number of endpoints: 9

#![no_std]

elrond_wasm_node::wasm_endpoints! {
    rewards_distribution
    (
        depositRoyalties
        raffle
        claimRewards
        getRoyalties
        getRewards
        getSeedNftMinterAddress
        getBrackets
        getLastRaffleEpoch
        getNftTokenIdentifier
    )
}

elrond_wasm_node::wasm_empty_callback! {}
