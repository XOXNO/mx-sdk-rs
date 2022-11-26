use elrond_wasm::abi::ContractAbi;

pub const DEFAULT_LABEL: &str = "default";

#[derive(Debug)]
pub struct OutputContractConfig {
    pub default_contract_config_name: String,
    pub contracts: Vec<OutputContract>,
}

impl OutputContractConfig {
    pub fn main_contract(&self) -> &OutputContract {
        self.contracts
            .iter()
            .find(|contract| contract.main)
            .unwrap_or_else(|| {
                panic!(
                    "Could not find default contract '{}' among the output contracts.",
                    self.default_contract_config_name
                )
            })
    }

    pub fn secondary_contracts(&self) -> impl Iterator<Item = &OutputContract> {
        self.contracts.iter().filter(move |contract| !contract.main)
    }

    pub fn secondary_contracts_mut(&mut self) -> impl Iterator<Item = &mut OutputContract> {
        self.contracts
            .iter_mut()
            .filter(move |contract| !contract.main)
    }

    pub fn get_contract_by_id(&self, name: String) -> Option<&OutputContract> {
        self.contracts
            .iter()
            .find(|contract| contract.contract_id == name)
    }

    pub fn get_contract_by_name(&self, name: String) -> Option<&OutputContract> {
        self.contracts
            .iter()
            .find(|contract| contract.contract_id == name)
    }

    /// Yields the contract with the given public name.
    pub fn find_contract(&self, contract_name: &str) -> &OutputContract {
        self.contracts
            .iter()
            .find(|contract| contract.contract_name == contract_name)
            .unwrap_or_else(|| panic!("output contract {} not found", contract_name))
    }
}

/// Represents a contract created by the framework when building.
///
/// It might have only some of the endpoints written by the developer and maybe some other function.
pub struct OutputContract {
    /// If it is the main contract, then the wasm crate is called just `wasm`,
    ///and the wasm `Cargo.toml` is provided by the dev.
    pub main: bool,

    /// External view contracts are just readers of data from another contract.
    pub external_view: bool,

    /// The contract id is defined in `multicontract.toml`. It has no effect on the produced assets.
    ///
    /// It can be the same as the contract name, but it is not necessary.
    pub contract_id: String,

    /// The name, as seen in the generated contract names.
    ///
    /// It is either defined in the multicontract.toml, or is inferred from the main crate name.
    pub contract_name: String,

    /// Filtered and processed ABI of the output contract.
    pub abi: ContractAbi,
}

impl OutputContract {
    pub fn public_name_snake_case(&self) -> String {
        self.contract_name.replace('-', "_")
    }

    /// The name of the directory of the wasm crate.
    ///
    /// Note this does not necessarily have to match the wasm crate name defined in Cargo.toml.
    pub fn wasm_crate_dir_name(&self) -> String {
        if self.main {
            "wasm".to_string()
        } else {
            format!("wasm-{}", &self.contract_name)
        }
    }

    pub fn wasm_crate_path(&self) -> String {
        format!("../{}", &self.wasm_crate_dir_name())
    }

    pub fn cargo_toml_path(&self) -> String {
        format!("{}/Cargo.toml", &self.wasm_crate_path())
    }

    /// The name of the wasm crate, as defined in its corresponding `Cargo.toml`.
    ///
    /// Note this does not necessarily have to match the name of the crate directory.
    pub fn wasm_crate_name(&self) -> String {
        format!("{}-wasm", &self.contract_name)
    }

    pub fn wasm_crate_name_snake_case(&self) -> String {
        self.wasm_crate_name().replace('-', "_")
    }

    /// This is where Rust will initially compile the WASM binary.
    pub fn wasm_compilation_output_path(&mut self, explicit_target_dir: &Option<String>) -> String {
        let target_dir = explicit_target_dir
            .clone()
            .unwrap_or_else(|| format!("{}/target", &self.wasm_crate_path(),));
        format!(
            "{}/wasm32-unknown-unknown/release/{}.wasm",
            &target_dir,
            &self.wasm_crate_name_snake_case(),
        )
    }

    pub fn abi_output_name(&self) -> String {
        format!("{}.abi.json", &self.contract_name)
    }

    pub fn wasm_output_name(&self, opt_suffix: &Option<String>) -> String {
        if let Some(suffix) = opt_suffix {
            format!("{}-{}.wasm", &self.contract_name, suffix)
        } else {
            format!("{}.wasm", &self.contract_name)
        }
    }

    pub fn endpoint_names(&self) -> Vec<String> {
        self.abi
            .endpoints
            .iter()
            .map(|endpoint| endpoint.name.to_string())
            .collect()
    }
}

impl std::fmt::Debug for OutputContract {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("OutputContract")
            .field("main", &self.main)
            .field("config_name", &self.contract_id)
            .field("public_name", &self.contract_name)
            .finish()
    }
}
