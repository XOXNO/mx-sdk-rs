mod bf_interact_cli;
mod bf_interact_config;
mod bf_interact_state;

use basic_features::basic_features_proxy;
pub use bf_interact_config::Config;
use bf_interact_state::State;
use clap::Parser;

use multiversx_sc_snippets::imports::*;

const INTERACTOR_SCENARIO_TRACE_PATH: &str = "interactor_trace.scen.json";

const CODE_EXPR_STORAGE_BYTES: MxscPath =
    MxscPath::new("../output/basic-features-storage-bytes.mxsc.json");

const CODE_EXPR: MxscPath = MxscPath::new("../output/basic-features.mxsc.json");

pub async fn basic_features_cli() {
    env_logger::init();

    let config = Config::load_config();

    let mut bf_interact = BasicFeaturesInteract::init(config).await;

    let cli = bf_interact_cli::InteractCli::parse();
    match &cli.command {
        Some(bf_interact_cli::InteractCliCommand::Deploy) => {
            bf_interact.deploy().await;
        },
        Some(bf_interact_cli::InteractCliCommand::DeployStorageBytes) => {
            bf_interact.deploy_storage_bytes().await;
        },
        Some(bf_interact_cli::InteractCliCommand::LargeStorage(args)) => {
            bf_interact.large_storage(args.size_kb).await;
        },
        Some(bf_interact_cli::InteractCliCommand::ReturnsEGLDDecimals(args)) => {
            bf_interact.returns_egld_decimal(args.egld).await;
        },
        Some(bf_interact_cli::InteractCliCommand::EchoManagedOption(args)) => {
            let mo = match args.managed_option {
                Some(value) => ManagedOption::some(BigUint::from(value)),
                None => ManagedOption::none(),
            };
            bf_interact.echo_managed_option(mo).await;
        },
        None => {},
    }
}

pub struct BasicFeaturesInteract {
    pub interactor: Interactor,
    pub wallet_address: Bech32Address,
    pub state: State,
    pub large_storage_payload: Vec<u8>,
}

impl BasicFeaturesInteract {
    pub async fn init(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri(), config.use_chain_simulator())
            .await
            .with_tracer(INTERACTOR_SCENARIO_TRACE_PATH)
            .await;
        let wallet_address = interactor.register_wallet(test_wallets::mike()).await;

        interactor.generate_blocks_until_epoch(1).await.unwrap();

        Self {
            interactor,
            wallet_address: wallet_address.into(),
            state: State::load_state(),
            large_storage_payload: Vec::new(),
        }
    }

    pub async fn large_storage(&mut self, size_kb: usize) {
        let large_data = std::fs::read_to_string("pi.txt").unwrap().into_bytes();
        let payload = &large_data[0..size_kb * 1024];
        println!("payload size: {}", payload.len());
        self.large_storage_payload = payload.to_vec();
        self.set_large_storage(payload).await;
    }

    async fn set_state(&mut self) {
        println!("wallet address: {}", self.wallet_address);
        self.interactor.retrieve_account(&self.wallet_address).await;
    }

    pub async fn deploy(&mut self) {
        self.set_state().await;

        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(400_000_000)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .init()
            .code(CODE_EXPR)
            .returns(ReturnsNewBech32Address)
            .run()
            .await;

        println!("new address for basic-features: {new_address}");

        self.state.set_bf_address(new_address);
    }

    pub async fn deploy_storage_bytes(&mut self) {
        self.set_state().await;

        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(4_000_000)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .init()
            .code(CODE_EXPR_STORAGE_BYTES)
            .returns(ReturnsNewBech32Address)
            .run()
            .await;

        println!("new address for basic-features-storage-bytes: {new_address}");

        self.state.set_bf_address_storage_bytes(new_address);
    }

    pub async fn set_large_storage(&mut self, value: &[u8]) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.bf_storage_bytes_contract())
            .gas(600_000_000)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .store_bytes(value)
            .run()
            .await;

        println!("successfully performed store_bytes");
    }

    pub async fn get_large_storage(&mut self) -> Vec<u8> {
        let data_raw = self
            .interactor
            .query()
            .to(self.state.bf_storage_bytes_contract())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .load_bytes()
            .returns(ReturnsResult)
            .run()
            .await;

        data_raw.to_vec()
    }

    pub async fn returns_egld_decimal(
        &mut self,
        egld: u64,
    ) -> ManagedDecimal<StaticApi, ConstDecimals<18>> {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.bf_contract())
            .gas(10_000_000)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .returns_egld_decimal()
            .egld(egld)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    pub async fn echo_managed_option(
        &mut self,
        mo: ManagedOption<StaticApi, BigUint<StaticApi>>,
    ) -> Option<RustBigUint> {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.bf_contract())
            .gas(10_000_000)
            .typed(basic_features::basic_features_proxy::BasicFeaturesProxy)
            .echo_managed_option(mo)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }
}
