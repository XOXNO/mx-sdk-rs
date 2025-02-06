// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           12
// Async Callback (empty):               1
// Promise callbacks:                    4
// Total number of exported functions:  18

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    promises_features
    (
        init => init
        callback_data => callback_data
        callback_data_at_index => callback_data_at_index
        clear_callback_data => clear_callback_data
        forward_promise_accept_funds => forward_promise_accept_funds
        forward_promise_retrieve_funds => forward_promise_retrieve_funds
        forward_payment_callback => forward_payment_callback
        forward_payment_gas_for_callback => forward_payment_gas_for_callback
        promise_raw_single_token => promise_raw_single_token
        promise_raw_multi_transfer => promise_raw_multi_transfer
        forward_sync_retrieve_funds_bt => forward_sync_retrieve_funds_bt
        forward_sync_retrieve_funds_bt_twice => forward_sync_retrieve_funds_bt_twice
        forward_promise_retrieve_funds_back_transfers => forward_promise_retrieve_funds_back_transfers
        retrieve_funds_callback => retrieve_funds_callback
        transfer_callback => transfer_callback
        the_one_callback => the_one_callback
        retrieve_funds_back_transfers_callback => retrieve_funds_back_transfers_callback
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
