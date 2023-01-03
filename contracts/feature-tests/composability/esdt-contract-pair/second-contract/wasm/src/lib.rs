// Code generated by the mx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            3
// Async Callback (empty):               1
// Total number of exported functions:   5

#![no_std]
#![feature(alloc_error_handler, lang_items)]

mx_sc_wasm_adapter::allocator!();
mx_sc_wasm_adapter::panic_handler!();

mx_sc_wasm_adapter::endpoints! {
    second_contract
    (
        acceptEsdtPayment
        rejectEsdtPayment
        getesdtTokenName
    )
}

mx_sc_wasm_adapter::empty_callback! {}
