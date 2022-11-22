// Code generated by the elrond-wasm multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Number of endpoints: 20

#![no_std]

elrond_wasm_node::wasm_endpoints! {
    multisig
    (
        deposit
        discardAction
        dnsRegister
        getActionLastIndex
        getNumBoardMembers
        getNumProposers
        getQuorum
        performAction
        proposeAddBoardMember
        proposeAddProposer
        proposeAsyncCall
        proposeChangeQuorum
        proposeRemoveUser
        proposeSCDeployFromSource
        proposeSCUpgradeFromSource
        proposeTransferExecute
        quorumReached
        sign
        signed
        unsign
    )
}

elrond_wasm_node::wasm_empty_callback! {}
