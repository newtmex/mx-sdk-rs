// Code generated by the mx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            2
// Async Callback (empty):               1
// Total number of exported functions:   4

#![no_std]

mx_sc_wasm_adapter::external_view_wasm_endpoints! {
    use_module
    (
        external_view_mod_a
        external_view_mod_b
    )
}

mx_sc_wasm_adapter::wasm_empty_callback! {}
