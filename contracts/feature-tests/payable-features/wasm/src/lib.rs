// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           16
// Async Callback (empty):               1
// Total number of exported functions:  18

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    payable_features
    (
        init => init
        echo_call_value => echo_call_value
        payment_multiple => payment_multiple
        payment_array_3 => payment_array_3
        payable_any_1 => payable_any_1
        payable_any_2 => payable_any_2
        payable_any_3 => payable_any_3
        payable_any_4 => payable_any_4
        payable_egld_1 => payable_egld_1
        payable_egld_2 => payable_egld_2
        payable_egld_3 => payable_egld_3
        payable_egld_4 => payable_egld_4
        payable_token_1 => payable_token_1
        payable_token_2 => payable_token_2
        payable_token_3 => payable_token_3
        payable_token_4 => payable_token_4
        payable_all_transfers => payable_all_transfers
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
