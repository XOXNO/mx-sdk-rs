// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

pub struct PromisesFeaturesProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for PromisesFeaturesProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = PromisesFeaturesProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        PromisesFeaturesProxyMethods { wrapped_tx: tx }
    }
}

pub struct PromisesFeaturesProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> PromisesFeaturesProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init(
        self,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> PromisesFeaturesProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn callback_data(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, CallbackData<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("callback_data")
            .original_result()
    }

    pub fn callback_data_at_index<
        Arg0: ProxyArg<usize>,
    >(
        self,
        index: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValue5<ManagedBuffer<Env::Api>, EgldOrEsdtTokenIdentifier<Env::Api>, u64, BigUint<Env::Api>, MultiValueManagedVec<Env::Api, ManagedBuffer<Env::Api>>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("callback_data_at_index")
            .argument(&index)
            .original_result()
    }

    pub fn clear_callback_data(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("clear_callback_data")
            .original_result()
    }

    pub fn forward_promise_accept_funds<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        to: Arg0,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("forward_promise_accept_funds")
            .argument(&to)
            .original_result()
    }

    pub fn forward_promise_retrieve_funds<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<EgldOrEsdtTokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<u64>,
        Arg3: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        to: Arg0,
        token: Arg1,
        token_nonce: Arg2,
        amount: Arg3,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("forward_promise_retrieve_funds")
            .argument(&to)
            .argument(&token)
            .argument(&token_nonce)
            .argument(&amount)
            .original_result()
    }

    pub fn forward_payment_callback<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        to: Arg0,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("forward_payment_callback")
            .argument(&to)
            .original_result()
    }

    pub fn forward_payment_gas_for_callback<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        to: Arg0,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("forward_payment_gas_for_callback")
            .argument(&to)
            .original_result()
    }

    pub fn promise_raw_single_token_to_user<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<u64>,
        Arg2: ProxyArg<u64>,
    >(
        self,
        to: Arg0,
        gas_limit: Arg1,
        extra_gas_for_callback: Arg2,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("promise_raw_single_token_to_user")
            .argument(&to)
            .argument(&gas_limit)
            .argument(&extra_gas_for_callback)
            .original_result()
    }

    pub fn promise_raw_single_token<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg2: ProxyArg<u64>,
        Arg3: ProxyArg<u64>,
        Arg4: ProxyArg<MultiValueEncoded<Env::Api, ManagedBuffer<Env::Api>>>,
    >(
        self,
        to: Arg0,
        endpoint_name: Arg1,
        gas_limit: Arg2,
        extra_gas_for_callback: Arg3,
        args: Arg4,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("promise_raw_single_token")
            .argument(&to)
            .argument(&endpoint_name)
            .argument(&gas_limit)
            .argument(&extra_gas_for_callback)
            .argument(&args)
            .original_result()
    }

    pub fn promise_raw_multi_transfer<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg2: ProxyArg<u64>,
        Arg3: ProxyArg<MultiValueEncoded<Env::Api, EsdtTokenPaymentMultiValue<Env::Api>>>,
    >(
        self,
        to: Arg0,
        endpoint_name: Arg1,
        extra_gas_for_callback: Arg2,
        token_payment_args: Arg3,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("promise_raw_multi_transfer")
            .argument(&to)
            .argument(&endpoint_name)
            .argument(&extra_gas_for_callback)
            .argument(&token_payment_args)
            .original_result()
    }

    pub fn forward_sync_retrieve_funds_bt<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<EgldOrEsdtTokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<u64>,
        Arg3: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        to: Arg0,
        token: Arg1,
        token_nonce: Arg2,
        amount: Arg3,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("forward_sync_retrieve_funds_bt")
            .argument(&to)
            .argument(&token)
            .argument(&token_nonce)
            .argument(&amount)
            .original_result()
    }

    pub fn forward_sync_retrieve_funds_bt_twice<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<EgldOrEsdtTokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<u64>,
        Arg3: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        to: Arg0,
        token: Arg1,
        token_nonce: Arg2,
        amount: Arg3,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("forward_sync_retrieve_funds_bt_twice")
            .argument(&to)
            .argument(&token)
            .argument(&token_nonce)
            .argument(&amount)
            .original_result()
    }

    pub fn forward_promise_retrieve_funds_back_transfers<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<EgldOrEsdtTokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<u64>,
        Arg3: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        to: Arg0,
        token: Arg1,
        token_nonce: Arg2,
        amount: Arg3,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("forward_promise_retrieve_funds_back_transfers")
            .argument(&to)
            .argument(&token)
            .argument(&token_nonce)
            .argument(&amount)
            .original_result()
    }
}

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub struct CallbackData<Api>
where
    Api: ManagedTypeApi,
{
    pub callback_name: ManagedBuffer<Api>,
    pub token_identifier: EgldOrEsdtTokenIdentifier<Api>,
    pub token_nonce: u64,
    pub token_amount: BigUint<Api>,
    pub args: ManagedVec<Api, ManagedBuffer<Api>>,
}
