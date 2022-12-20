// Code generated by the mx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           18
// Async Callback:                       1
// Total number of exported functions:  20

#![no_std]
#![feature(alloc_error_handler, lang_items)]

mx_sc_wasm_adapter::allocator_declaration!();
mx_sc_wasm_adapter::panic_handler_declaration!();

mx_sc_wasm_adapter::endpoints! {
    local_esdt_and_nft
    (
        issueFungibleToken
        localMint
        localBurn
        nftIssue
        nftCreate
        nftAddQuantity
        nftBurn
        transferNftViaAsyncCall
        transfer_nft_and_execute
        sftIssue
        setLocalRoles
        unsetLocalRoles
        controlChanges
        getFungibleEsdtBalance
        getNftBalance
        getCurrentNftNonce
        lastIssuedToken
        lastErrorMessage
        callBack
    )
}
