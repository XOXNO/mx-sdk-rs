mod basic_interact_cli;
mod basic_interact_config;
mod basic_interact_state;

use basic_interact_cli::NftDummyAttributes;
use basic_interact_config::Config;
use basic_interact_state::State;
use clap::Parser;

use multiversx_sc_snippets::imports::*;

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut basic_interact = SysFuncCallsInteract::init().await;

    let cli = basic_interact_cli::InteractCli::parse();
    match &cli.command {
        Some(basic_interact_cli::InteractCliCommand::IssueToken(args)) => {
            basic_interact
                .issue_token(
                    args.cost.clone(),
                    &args.display_name,
                    &args.ticker,
                    args.num_decimals,
                    args.token_type.into(),
                )
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::Mint(args)) => {
            basic_interact
                .mint_token(&args.token_id, args.amount.clone())
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::SetRoles(args)) => {
            basic_interact
                .set_roles(&args.token_id, args.roles.clone())
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::Burn(args)) => {
            basic_interact
                .burn_token(&args.token_id, args.amount.clone())
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::PauseToken(args)) => {
            basic_interact.pause_token(&args.token_id).await;
        },
        Some(basic_interact_cli::InteractCliCommand::UnpauseToken(args)) => {
            basic_interact.unpause_token(&args.token_id).await;
        },
        Some(basic_interact_cli::InteractCliCommand::FreezeToken(args)) => {
            basic_interact
                .freeze_token(&args.token_id, &args.address)
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::UnfreezeToken(args)) => {
            basic_interact
                .unfreeze_token(&args.token_id, &args.address)
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::FreezeNFT(args)) => {
            basic_interact
                .freeze_nft(&args.token_id, args.nft_nonce, &args.address)
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::UnfreezeNFT(args)) => {
            basic_interact
                .unfreeze_nft(&args.token_id, args.nft_nonce, &args.address)
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::WipeToken(args)) => {
            basic_interact
                .wipe_token(&args.token_id, &args.address)
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::WipeNFT(args)) => {
            basic_interact
                .wipe_nft(&args.token_id, args.nft_nonce, &args.address)
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::IssueNFTCollection(args)) => {
            basic_interact
                .issue_non_fungible_collection(args.cost.clone(), &args.display_name, &args.ticker)
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::CreateNFT(args)) => {
            basic_interact
                .create_nft(
                    &args.token_id,
                    args.amount.clone(),
                    &args.name,
                    args.royalties,
                    &args.hash,
                )
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::IssueFungible(args)) => {
            basic_interact
                .issue_fungible_token(
                    args.cost.clone(),
                    &args.display_name,
                    &args.ticker,
                    args.supply.clone(),
                    args.num_decimals,
                )
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::IssueSftCollection(args)) => {
            basic_interact
                .issue_semi_fungible_collection(args.cost.clone(), &args.display_name, &args.ticker)
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::MintSft(args)) => {
            basic_interact
                .mint_sft(
                    &args.token_id,
                    args.amount.clone(),
                    &args.name,
                    args.royalties.clone(),
                    &args.hash,
                )
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::RegisterMetaEsdt(args)) => {
            basic_interact
                .register_meta_esdt(
                    args.cost.clone(),
                    &args.display_name,
                    &args.ticker,
                    args.num_decimals,
                )
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::ChangeSftMetaEsdt(args)) => {
            basic_interact
                .change_sft_meta_esdt(&args.token_id, args.num_decimals)
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::UnsetRoles(args)) => {
            basic_interact
                .unset_roles(&args.address, &args.token_id, args.roles.clone())
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::TransferOwnership(args)) => {
            basic_interact
                .transfer_ownership(&args.token_id, &args.new_owner)
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::TransferNftCreateRole(args)) => {
            basic_interact
                .transfer_nft_create_role(&args.token_id, &args.old_owner, &args.new_owner)
                .await;
        },
        Some(basic_interact_cli::InteractCliCommand::ControlChanges(args)) => {
            basic_interact.control_changes(&args.token_id).await;
        },

        None => {},
    }
}

#[allow(unused)]
struct SysFuncCallsInteract {
    interactor: Interactor,
    wallet_address: Bech32Address,
    state: State,
}

impl SysFuncCallsInteract {
    async fn init() -> Self {
        let config = Config::load_config();
        let mut interactor = Interactor::new(config.gateway()).await;

        let wallet_address = interactor.register_wallet(test_wallets::alice());

        Self {
            interactor,
            wallet_address: wallet_address.into(),
            state: State::load_state(),
        }
    }

    async fn issue_fungible_token(
        &mut self,
        issue_cost: RustBigUint,
        token_display_name: &str,
        token_ticker: &str,
        initial_supply: RustBigUint,
        num_decimals: usize,
    ) {
        println!("Issuing fungible token...");
        let res = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress.to_managed_address())
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .issue_fungible(
                issue_cost.into(),
                &token_display_name.into(),
                &token_ticker.into(),
                &initial_supply.into(),
                FungibleTokenProperties {
                    num_decimals,
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_mint: true,
                    can_burn: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .returns(ReturnsNewTokenIdentifier)
            .prepare_async()
            .run()
            .await;

        println!("TOKEN ID: {:?}", res);
    }

    async fn issue_non_fungible_collection(
        &mut self,
        issue_cost: RustBigUint,
        token_display_name: &str,
        token_ticker: &str,
    ) {
        println!("Issuing NFT Collection...");
        let nft_collection_id = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress.to_managed_address())
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .issue_non_fungible(
                issue_cost.into(),
                &token_display_name.into(),
                &token_ticker.into(),
                NonFungibleTokenProperties {
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_transfer_create_role: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .returns(ReturnsNewTokenIdentifier)
            .prepare_async()
            .run()
            .await;

        println!("NFT Collection ID: {:?}", nft_collection_id);
    }

    async fn issue_semi_fungible_collection(
        &mut self,
        issue_cost: RustBigUint,
        token_display_name: &str,
        token_ticker: &str,
    ) {
        println!("Issuing SFT Collection...");
        let sft_collection_id = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress.to_managed_address())
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .issue_semi_fungible(
                issue_cost.into(),
                &token_display_name.into(),
                &token_ticker.into(),
                SemiFungibleTokenProperties {
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_transfer_create_role: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .returns(ReturnsNewTokenIdentifier)
            .prepare_async()
            .run()
            .await;

        println!("SFT Collection ID: {:?}", sft_collection_id);
    }

    async fn issue_token(
        &mut self,
        issue_cost: RustBigUint,
        token_display_name: &str,
        token_ticker: &str,
        num_decimals: usize,
        token_type: EsdtTokenType,
    ) {
        println!(
            "Registering token... Don't forget to mint afterwards in order to receive the tokens!"
        );
        let token_id = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress.to_managed_address())
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .issue_and_set_all_roles(
                issue_cost.into(),
                token_display_name.into(),
                token_ticker.into(),
                token_type,
                num_decimals,
            )
            .returns(ReturnsNewTokenIdentifier)
            .prepare_async()
            .run()
            .await;

        println!("TOKEN ID: {:?}", token_id);
    }

    async fn set_roles(&mut self, token_id: &str, roles: Vec<u16>) {
        let wallet_address = &self.wallet_address.clone().into_address();
        let converted_roles: Vec<EsdtLocalRole> =
            roles.into_iter().map(EsdtLocalRole::from).collect();

        println!("Setting the following roles: {:?}", converted_roles);

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress.to_managed_address())
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .set_special_roles::<std::vec::IntoIter<EsdtLocalRole>>(
                &ManagedAddress::from_address(wallet_address),
                &TokenIdentifier::from(token_id),
                converted_roles.into_iter(),
            )
            .prepare_async()
            .run()
            .await;
    }

    async fn mint_sft(
        &mut self,
        token_id: &str,
        amount: RustBigUint,
        name: &str,
        royalties: RustBigUint,
        hash: &str,
    ) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(&self.wallet_address)
            .gas(100_000_000u64)
            .typed(UserBuiltinProxy)
            .esdt_nft_create(
                &TokenIdentifier::from(token_id),
                &BigUint::from(amount),
                &ManagedBuffer::from(name),
                &royalties.into(),
                &ManagedBuffer::from(hash),
                &NftDummyAttributes {
                    creation_epoch: 2104,
                    cool_factor: 5,
                },
                &ManagedVec::new(),
            )
            .prepare_async()
            .run()
            .await;
    }

    async fn register_meta_esdt(
        &mut self,
        issue_cost: RustBigUint,
        token_display_name: &str,
        token_ticker: &str,
        num_decimals: usize,
    ) {
        println!("Registering meta ESDT... Don't forget to mint afterwards in order to receive the tokens!");
        let meta_esdt = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress.to_managed_address())
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .register_meta_esdt(
                issue_cost.into(),
                &token_display_name.into(),
                &token_ticker.into(),
                MetaTokenProperties {
                    num_decimals,
                    can_freeze: true,
                    can_wipe: true,
                    can_transfer_create_role: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_pause: true,
                    can_add_special_roles: true,
                },
            )
            .returns(ReturnsNewTokenIdentifier)
            .prepare_async()
            .run()
            .await;

        println!("Meta-ESDT ID: {:?}", meta_esdt);
    }

    async fn change_sft_meta_esdt(&mut self, token_id: &str, num_decimals: usize) {
        println!("Changing SFT to Meta-ESDT...");
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress.to_managed_address())
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .change_sft_to_meta_esdt(&TokenIdentifier::from(token_id), num_decimals)
            .prepare_async()
            .run()
            .await;
    }

    async fn mint_token(&mut self, token_id: &str, amount: RustBigUint) {
        println!("Minting tokens...");
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(&self.wallet_address)
            .gas(100_000_000u64)
            .typed(UserBuiltinProxy)
            .esdt_local_mint(&TokenIdentifier::from(token_id), 0, &BigUint::from(amount))
            .prepare_async()
            .run()
            .await;
    }

    async fn burn_token(&mut self, token_id: &str, amount: RustBigUint) {
        println!("Burning tokens...");
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(&self.wallet_address)
            .gas(100_000_000u64)
            .typed(UserBuiltinProxy)
            .esdt_local_burn(&TokenIdentifier::from(token_id), 0, &BigUint::from(amount))
            .prepare_async()
            .run()
            .await;
    }

    async fn pause_token(&mut self, token_id: &str) {
        println!("Pausing token...");
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress.to_managed_address())
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .pause(&TokenIdentifier::from(token_id))
            .prepare_async()
            .run()
            .await;
    }

    async fn unpause_token(&mut self, token_id: &str) {
        println!("Unpausing token...");
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress.to_managed_address())
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .unpause(&TokenIdentifier::from(token_id))
            .prepare_async()
            .run()
            .await;
    }

    async fn freeze_token(&mut self, token_id: &str, address: &str) {
        println!("Freezing token...");
        let address = Bech32Address::from_bech32_string(address.to_string()).to_address();
        let managed_address: ManagedAddress<StaticApi> = ManagedAddress::from_address(&address);
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress.to_managed_address())
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .freeze(&TokenIdentifier::from(token_id), &managed_address)
            .prepare_async()
            .run()
            .await;
    }

    async fn unfreeze_token(&mut self, token_id: &str, address: &str) {
        println!("Unfreezing token...");
        let address = Bech32Address::from_bech32_string(address.to_string()).to_address();
        let managed_address: ManagedAddress<StaticApi> = ManagedAddress::from_address(&address);
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress.to_managed_address())
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .unfreeze(&TokenIdentifier::from(token_id), &managed_address)
            .prepare_async()
            .run()
            .await;
    }

    async fn freeze_nft(&mut self, token_id: &str, nonce: u64, address: &str) {
        println!("Freezing NFT/SFT/Meta-ESDT...");
        let address = Bech32Address::from_bech32_string(address.to_string()).to_address();
        let managed_address: ManagedAddress<StaticApi> = ManagedAddress::from_address(&address);
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress.to_managed_address())
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .freeze_nft(&TokenIdentifier::from(token_id), nonce, &managed_address)
            .prepare_async()
            .run()
            .await;
    }

    async fn unfreeze_nft(&mut self, token_id: &str, nonce: u64, address: &str) {
        println!("Unfreezing NFT/SFT/Meta-ESDT...");
        let address = Bech32Address::from_bech32_string(address.to_string()).to_address();
        let managed_address: ManagedAddress<StaticApi> = ManagedAddress::from_address(&address);
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress.to_managed_address())
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .unfreeze_nft(&TokenIdentifier::from(token_id), nonce, &managed_address)
            .prepare_async()
            .run()
            .await;
    }

    async fn wipe_token(&mut self, token_id: &str, address: &str) {
        println!("Wiping token...");
        let address = Bech32Address::from_bech32_string(address.to_string()).to_address();
        let managed_address: ManagedAddress<StaticApi> = ManagedAddress::from_address(&address);
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(&ESDTSystemSCAddress.to_managed_address())
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .wipe(&TokenIdentifier::from(token_id), &managed_address)
            .prepare_async()
            .run()
            .await;
    }

    async fn wipe_nft(&mut self, token_id: &str, nonce: u64, address: &str) {
        println!("Wiping NFT/SFT/Meta-ESDT...");
        let address = Bech32Address::from_bech32_string(address.to_string()).to_address();
        let managed_address: ManagedAddress<StaticApi> = ManagedAddress::from_address(&address);
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress.to_managed_address())
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .wipe_nft(&TokenIdentifier::from(token_id), nonce, &managed_address)
            .prepare_async()
            .run()
            .await;
    }

    async fn create_nft(
        &mut self,
        token_id: &str,
        amount: RustBigUint,
        name: &str,
        royalties: u64,
        hash: &str,
    ) {
        println!("Minting NFT...");
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(&self.wallet_address)
            .gas(100_000_000u64)
            .typed(UserBuiltinProxy)
            .esdt_nft_create(
                &TokenIdentifier::from(token_id),
                &BigUint::from(amount),
                &ManagedBuffer::from(name),
                &BigUint::from(royalties),
                &ManagedBuffer::from(hash),
                &NftDummyAttributes {
                    creation_epoch: 2104,
                    cool_factor: 5,
                },
                &ManagedVec::new(),
            )
            .prepare_async()
            .run()
            .await;
    }

    async fn unset_roles(&mut self, address: &str, token_id: &str, roles: Vec<u16>) {
        let converted_roles: Vec<EsdtLocalRole> =
            roles.into_iter().map(EsdtLocalRole::from).collect();

        println!("Unsetting the following roles: {:?}", converted_roles);

        let bech32_addr = Bech32Address::from_bech32_string(address.to_string());
        let addr = bech32_addr.to_address();
        let managed_addr: ManagedAddress<StaticApi> = ManagedAddress::from_address(&addr);

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress.to_managed_address())
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .unset_special_roles(
                &managed_addr,
                &TokenIdentifier::from(token_id),
                converted_roles.into_iter(),
            )
            .prepare_async()
            .run()
            .await;
    }

    async fn transfer_ownership(&mut self, token_id: &str, new_owner: &str) {
        println!("Transferring token ownership...");
        let bech32_addr = Bech32Address::from_bech32_string(new_owner.to_string());
        let addr = bech32_addr.to_address();
        let managed_addr: ManagedAddress<StaticApi> = ManagedAddress::from_address(&addr);

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress.to_managed_address())
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .transfer_ownership(&TokenIdentifier::from(token_id), &managed_addr)
            .prepare_async()
            .run()
            .await;
    }

    async fn transfer_nft_create_role(&mut self, token_id: &str, old_owner: &str, new_owner: &str) {
        println!("Transferring NFT create role...");
        let bech32_addr_new_owner = Bech32Address::from_bech32_string(new_owner.to_string());
        let addr_new_owner = bech32_addr_new_owner.to_address();
        let managed_addr_new_owner: ManagedAddress<StaticApi> =
            ManagedAddress::from_address(&addr_new_owner);

        let bech32_addr_old_owner = Bech32Address::from_bech32_string(old_owner.to_string());
        let addr_old_owner = bech32_addr_old_owner.to_address();
        let managed_addr_old_owner: ManagedAddress<StaticApi> =
            ManagedAddress::from_address(&addr_old_owner);

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress.to_managed_address())
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .transfer_nft_create_role(
                &TokenIdentifier::from(token_id),
                &managed_addr_old_owner,
                &managed_addr_new_owner,
            )
            .prepare_async()
            .run()
            .await;
    }

    async fn control_changes(&mut self, token_id: &str) {
        println!("Control changes");
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(ESDTSystemSCAddress.to_managed_address())
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .control_changes(
                &TokenIdentifier::from(token_id),
                &TokenPropertyArguments {
                    can_freeze: Some(true),
                    can_wipe: Some(true),
                    can_pause: Some(true),
                    can_transfer_create_role: Some(true),
                    can_mint: Some(true),
                    can_burn: Some(true),
                    can_change_owner: Some(true),
                    can_upgrade: Some(true),
                    can_add_special_roles: Some(true),
                },
            )
            .prepare_async()
            .run()
            .await;
    }
}
