// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

pub struct BasicFeaturesProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for BasicFeaturesProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = BasicFeaturesProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        BasicFeaturesProxyMethods { wrapped_tx: tx }
    }
}

pub struct BasicFeaturesProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> BasicFeaturesProxyMethods<Env, From, (), Gas>
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
impl<Env, From, To, Gas> BasicFeaturesProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn verify_secp256r1_signature<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg2: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        key: Arg0,
        message: Arg1,
        signature: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("verify_secp256r1_signature")
            .argument(&key)
            .argument(&message)
            .argument(&signature)
            .original_result()
    }

    pub fn verify_bls_signature_share<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg2: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        key: Arg0,
        message: Arg1,
        signature: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("verify_bls_signature_share")
            .argument(&key)
            .argument(&message)
            .argument(&signature)
            .original_result()
    }

    pub fn verify_bls_aggregated_signature<
        Arg0: ProxyArg<ManagedVec<Env::Api, ManagedBuffer<Env::Api>>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg2: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        key: Arg0,
        message: Arg1,
        signature: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("verify_bls_aggregated_signature")
            .argument(&key)
            .argument(&message)
            .argument(&signature)
            .original_result()
    }

    /// This tests how is generated type name in proxy 
    pub fn echo_managed_option<
        Arg0: ProxyArg<ManagedOption<Env::Api, BigUint<Env::Api>>>,
    >(
        self,
        mo: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedOption<Env::Api, BigUint<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("echo_managed_option")
            .argument(&mo)
            .original_result()
    }

    pub fn load_bytes(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedBuffer<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("load_bytes")
            .original_result()
    }

    pub fn store_bytes<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        bi: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("store_bytes")
            .argument(&bi)
            .original_result()
    }

    pub fn timelock_mapper(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("timelock_mapper")
            .original_result()
    }

    pub fn timelock_set_initial_value<
        Arg0: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        initial_value: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("timelock_set_initial_value")
            .argument(&initial_value)
            .original_result()
    }

    pub fn timelock_set_unlock_timestamp<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        unlock_timestamp: Arg0,
        future_value: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("timelock_set_unlock_timestamp")
            .argument(&unlock_timestamp)
            .argument(&future_value)
            .original_result()
    }

    pub fn timelock_commit_action(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, bool> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("timelock_commit_action")
            .original_result()
    }

    pub fn timelock_get_unlock_timestamp(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("timelock_get_unlock_timestamp")
            .original_result()
    }

    pub fn timelock_get_future_value(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("timelock_get_future_value")
            .original_result()
    }

    pub fn timelock_get_current_value_at_address<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("timelock_get_current_value_at_address")
            .argument(&address)
            .original_result()
    }

    pub fn timelock_get_unlock_timestamp_at_address<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("timelock_get_unlock_timestamp_at_address")
            .argument(&address)
            .original_result()
    }

    pub fn timelock_get_future_value_at_address<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("timelock_get_future_value_at_address")
            .argument(&address)
            .original_result()
    }

    pub fn returns_egld_decimal(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ManagedDecimal<Env::Api, ConstDecimals<U18>>> {
        self.wrapped_tx
            .raw_call("returns_egld_decimal")
            .original_result()
    }
}

#[type_abi]
#[derive(Clone, Copy)]
pub struct CodecErrorTestType {}

#[rustfmt::skip]
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub enum ExampleEnumWithFields {
    Unit,
    Newtype(u32),
    Tuple(u32, u32),
    Struct {
        a: u32,
    },
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub enum ExampleEnumSimple {
    Variant0,
    Variant1,
    Variant2,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct TokenAttributesStruct<Api>
where
    Api: ManagedTypeApi,
{
    pub field_biguint: BigUint<Api>,
    pub field_u64: u64,
    pub field_vec_u32: ManagedVec<Api, u32>,
}

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, Debug, Clone)]
pub struct ExampleStructManaged<Api>
where
    Api: ManagedTypeApi,
{
    pub big_uint: BigUint<Api>,
    pub int: u32,
    pub bytes: ManagedBuffer<Api>,
}
