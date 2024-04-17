use std::path::PathBuf;

use multiversx_chain_scenario_format::serde_raw::ValueSubTree;
use multiversx_sc::{
    tuple_util::NestedTupleFlatten,
    types::{
        AnnotatedValue, Code, DeployCall, FunctionCall, ManagedAddress, ManagedBuffer, RHListExec,
        Tx, TxBaseWithEnv, TxCodeSource, TxCodeSourceSpecified, TxCodeValue, TxEnv,
        TxFromSpecified, TxGas, TxPayment, TxToSpecified,
    },
};

use crate::{
    api::StaticApi,
    scenario::tx_to_step::TxToStep,
    scenario_model::{AddressValue, BytesValue, ScCallStep, ScDeployStep, TxResponse},
    ScenarioEnvExec, ScenarioTxEnv, ScenarioTxRun, ScenarioWorld,
};

use super::ScenarioTxEnvData;

impl<'w, From, Payment, Gas, CodeValue, RH> ScenarioTxRun
    for Tx<
        ScenarioEnvExec<'w>,
        From,
        (),
        Payment,
        Gas,
        DeployCall<ScenarioEnvExec<'w>, Code<CodeValue>>,
        RH,
    >
where
    From: TxFromSpecified<ScenarioEnvExec<'w>>,
    Payment: TxPayment<ScenarioEnvExec<'w>>,
    Gas: TxGas<ScenarioEnvExec<'w>>,
    CodeValue: TxCodeValue<ScenarioEnvExec<'w>>,
    RH: RHListExec<TxResponse, ScenarioEnvExec<'w>>,
    RH::ListReturns: NestedTupleFlatten,
{
    type Returns = <RH::ListReturns as NestedTupleFlatten>::Unpacked;

    fn run(self) -> Self::Returns {
        let mut step_wrapper = self.tx_to_step();
        step_wrapper.env.world.sc_deploy(&mut step_wrapper.step);
        step_wrapper.process_result()
    }
}

impl ScenarioWorld {
    pub fn chain_deploy<From, Payment, Gas, CodeValue, RH, F>(&mut self, f: F) -> &mut Self
    where
        From: TxFromSpecified<ScenarioTxEnvData>,
        Payment: TxPayment<ScenarioTxEnvData>,
        Gas: TxGas<ScenarioTxEnvData>,
        CodeValue: TxCodeValue<ScenarioTxEnvData>,
        RH: RHListExec<TxResponse, ScenarioTxEnvData, ListReturns = ()>,
        F: FnOnce(
            TxBaseWithEnv<ScenarioTxEnvData>,
        ) -> Tx<
            ScenarioTxEnvData,
            From,
            (),
            Payment,
            Gas,
            DeployCall<ScenarioTxEnvData, Code<CodeValue>>,
            RH,
        >,
    {
        let env = self.new_env_data();
        let tx_base = TxBaseWithEnv::new_with_env(env);
        let tx = f(tx_base);

        let mut step_wrapper = tx.tx_to_step();
        self.sc_deploy(&mut step_wrapper.step);
        step_wrapper.process_result();

        self
    }
}
