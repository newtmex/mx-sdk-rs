// Code generated by the elrond-wasm multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Number of endpoints: 9

#![no_std]

elrond_wasm_node::wasm_endpoints! {
    kitty_auction
    (
        callBack
        setKittyOwnershipContractAddress
        createAndAuctionGenZeroKitty
        isUpForAuction
        getAuctionStatus
        getCurrentWinningBid
        createSaleAuction
        createSiringAuction
        bid
        endAuction
    )
}
