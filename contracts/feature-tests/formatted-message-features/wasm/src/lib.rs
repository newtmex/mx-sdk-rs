// Code generated by the mx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           15
// Async Callback (empty):               1
// Total number of exported functions:  17

#![no_std]
#![feature(alloc_error_handler, lang_items)]

mx_sc_wasm_adapter::allocator_declaration!();
mx_sc_wasm_adapter::panic_handler_declaration!();

mx_sc_wasm_adapter::endpoints! {
    formatted_message_features
    (
        static_message
        dynamic_message
        dynamic_message_hex
        dynamic_message_multiple
        dynamic_message_ascii
        decode_error_message
        print_message
        print_message_hex
        print_message_binary
        print_message_codec
        format_message_one_part
        format_message_multiple_parts
        format_message_big_int
        format_message_managed_buffer
        format_message_managed_buffer_hex
    )
}

mx_sc_wasm_adapter::empty_callback! {}
