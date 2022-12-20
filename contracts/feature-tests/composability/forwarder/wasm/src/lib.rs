// Code generated by the mx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           69
// Async Callback:                       1
// Total number of exported functions:  71

#![no_std]
#![feature(alloc_error_handler, lang_items)]

mx_sc_wasm_adapter::allocator_declaration!();
mx_sc_wasm_adapter::panic_handler_declaration!();

mx_sc_wasm_adapter::endpoints! {
    forwarder
    (
        send_egld
        echo_arguments_sync
        echo_arguments_sync_twice
        forward_sync_accept_funds
        forward_sync_accept_funds_with_fees
        forward_sync_accept_funds_then_read
        forward_sync_retrieve_funds
        forward_sync_retrieve_funds_with_accept_func
        accept_funds_func
        forward_sync_accept_funds_multi_transfer
        echo_args_async
        forward_async_accept_funds
        forward_async_accept_funds_half_payment
        forward_async_accept_funds_with_fees
        forward_async_retrieve_funds
        send_funds_twice
        send_async_accept_multi_transfer
        callback_data
        callback_data_at_index
        clear_callback_data
        forward_transf_exec_accept_funds
        forward_transf_execu_accept_funds_with_fees
        forward_transf_exec_accept_funds_twice
        forward_transf_exec_accept_funds_return_values
        transf_exec_multi_accept_funds
        forward_transf_exec_reject_funds_multi_transfer
        transf_exec_multi_reject_funds
        queued_calls
        add_queued_call
        forward_queued_calls
        changeOwnerAddress
        deploy_contract
        deploy_two_contracts
        deploy_vault_from_source
        upgradeVault
        upgrade_vault_from_source
        getFungibleEsdtBalance
        getCurrentNftNonce
        send_esdt
        send_esdt_with_fees
        send_esdt_twice
        send_esdt_direct_multi_transfer
        issue_fungible_token
        local_mint
        local_burn
        get_esdt_local_roles
        get_esdt_token_data
        is_esdt_frozen
        is_esdt_paused
        is_esdt_limited_transfer
        validate_token_identifier
        sft_issue
        get_nft_balance
        buy_nft
        nft_issue
        nft_create
        nft_create_compact
        nft_add_uris
        nft_update_attributes
        nft_decode_complex_attributes
        nft_add_quantity
        nft_burn
        transfer_nft_via_async_call
        transfer_nft_and_execute
        create_and_send
        setLocalRoles
        unsetLocalRoles
        lastIssuedToken
        lastErrorMessage
        callBack
    )
}
