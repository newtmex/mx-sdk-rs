// Code generated by the elrond-wasm multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            2
// Async Callback (empty):               1
// Total number of exported functions:   4

#![no_std]

mx_sc_wasm_adapter::wasm_endpoints! {
    adder
    (
        getSum
        add
    )
}

mx_sc_wasm_adapter::wasm_empty_callback! {}
