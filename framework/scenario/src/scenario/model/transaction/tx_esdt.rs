use multiversx_sc::{
    api::ManagedTypeApi,
    chain_core::EGLD_000000_TOKEN_IDENTIFIER,
    types::{EgldOrEsdtTokenPayment, EsdtTokenPayment},
};

use crate::{
    scenario::model::{BigUintValue, BytesValue, U64Value},
    scenario_format::{
        interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
        serde_raw::{TxESDTRaw, ValueSubTree},
    },
};

#[derive(Debug, Clone)]
pub struct TxESDT {
    pub esdt_token_identifier: BytesValue,
    pub nonce: U64Value,
    pub esdt_value: BigUintValue,
}

impl TxESDT {
    pub fn is_egld(&self) -> bool {
        self.esdt_token_identifier.value == EGLD_000000_TOKEN_IDENTIFIER.as_bytes()
    }
}

impl InterpretableFrom<TxESDTRaw> for TxESDT {
    fn interpret_from(from: TxESDTRaw, context: &InterpreterContext) -> Self {
        TxESDT {
            esdt_token_identifier: interpret_esdt_token_identifier(from.token_identifier, context),
            nonce: interpret_opt_u64(from.nonce, context),
            esdt_value: BigUintValue::interpret_from(from.value, context),
        }
    }
}

impl IntoRaw<TxESDTRaw> for TxESDT {
    fn into_raw(self) -> TxESDTRaw {
        TxESDTRaw {
            token_identifier: Some(self.esdt_token_identifier.into_raw()),
            nonce: self.nonce.into_raw_opt(),
            value: self.esdt_value.into_raw(),
        }
    }
}

impl<M: ManagedTypeApi> From<EsdtTokenPayment<M>> for TxESDT {
    fn from(value: EsdtTokenPayment<M>) -> Self {
        TxESDT {
            esdt_token_identifier: BytesValue::from(
                value.token_identifier.as_managed_buffer().to_vec(),
            ),
            nonce: U64Value::from(value.token_nonce),
            esdt_value: BigUintValue::from(value.amount),
        }
    }
}

impl<M: ManagedTypeApi> From<EgldOrEsdtTokenPayment<M>> for TxESDT {
    fn from(value: EgldOrEsdtTokenPayment<M>) -> Self {
        TxESDT {
            esdt_token_identifier: BytesValue::from(
                value.token_identifier.as_managed_buffer().to_vec(),
            ),
            nonce: U64Value::from(value.token_nonce),
            esdt_value: BigUintValue::from(value.amount),
        }
    }
}

fn interpret_esdt_token_identifier(
    esdt_token_identifier: Option<ValueSubTree>,
    context: &InterpreterContext,
) -> BytesValue {
    if let Some(esdt_token_identifier_raw) = esdt_token_identifier {
        BytesValue::interpret_from(esdt_token_identifier_raw, context)
    } else {
        BytesValue::empty()
    }
}

fn interpret_opt_u64(opt_u64: Option<ValueSubTree>, context: &InterpreterContext) -> U64Value {
    if let Some(u) = opt_u64 {
        U64Value::interpret_from(u, context)
    } else {
        U64Value::empty()
    }
}
