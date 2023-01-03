// Code generated by the mx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            6
// Async Callback:                       1
// Total number of exported functions:   8

#![no_std]
#![feature(alloc_error_handler, lang_items)]

mx_sc_wasm_adapter::allocator!();
mx_sc_wasm_adapter::panic_handler!();

mx_sc_wasm_adapter::endpoints! {
    nft_minter
    (
        createNft
        claimRoyaltiesFromMarketplace
        issueToken
        setLocalRoles
        buyNft
        getNftPrice
        callBack
    )
}
