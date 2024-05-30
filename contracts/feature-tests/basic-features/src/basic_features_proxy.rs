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

    pub fn returns_egld_decimal(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ManagedDecimal<Env::Api, ConstDecimals<18>>> {
        self.wrapped_tx
            .raw_call("returns_egld_decimal")
            .original_result()
    }
}

#[type_abi]
#[derive(TopEncode, TopDecode)]
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
