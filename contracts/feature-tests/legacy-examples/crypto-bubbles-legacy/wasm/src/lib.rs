// Code generated by the elrond-wasm multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            6
// Async Callback (empty):               1
// Total number of exported functions:   8

#![no_std]

mx_sc_wasm_adapter::wasm_endpoints! {
    crypto_bubbles_legacy
    (
        topUp
        withdraw
        joinGame
        rewardWinner
        rewardAndSendToWallet
        balanceOf
    )
}

mx_sc_wasm_adapter::wasm_empty_callback! {}
