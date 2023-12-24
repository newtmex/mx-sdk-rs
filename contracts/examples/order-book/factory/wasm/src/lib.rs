// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            2
// Async Callback (empty):               1
// Total number of exported functions:   4

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    order_book_factory
    (
        init => init
        createPair => create_pair
        getPair => get_pair
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
