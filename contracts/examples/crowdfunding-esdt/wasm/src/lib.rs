// Code generated by the elrond-wasm multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            8
// Async Callback (empty):               1
// Total number of exported functions:  10

#![no_std]

mx_sc_wasm_adapter::wasm_endpoints! {
    crowdfunding_esdt
    (
        fund
        status
        getCurrentFunds
        claim
        getTarget
        getDeadline
        getDeposit
        getCrowdfundingTokenIdentifier
    )
}

mx_sc_wasm_adapter::wasm_empty_callback! {}
