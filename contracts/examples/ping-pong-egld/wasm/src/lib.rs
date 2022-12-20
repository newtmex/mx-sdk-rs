// Code generated by the mx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           10
// Async Callback (empty):               1
// Total number of exported functions:  12

#![no_std]
#![feature(alloc_error_handler, lang_items)]

mx_sc_wasm_adapter::allocator_declaration!();
mx_sc_wasm_adapter::panic_handler_declaration!();

mx_sc_wasm_adapter::endpoints! {
    ping_pong_egld
    (
        ping
        pong
        pongAll
        getUserAddresses
        getPingAmount
        getDeadline
        getActivationTimestamp
        getMaxFunds
        getUserStatus
        pongAllLastUser
    )
}

mx_sc_wasm_adapter::empty_callback! {}
