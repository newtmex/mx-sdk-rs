// Code generated by the elrond-wasm multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            3
// Async Callback (empty):               1
// Total number of exported functions:   5

#![no_std]

elrond_wasm_node::wasm_endpoints! {
    proxy_test_second
    (
        payMe
        payMeWithResult
        messageMe
    )
}

elrond_wasm_node::wasm_empty_callback! {}
