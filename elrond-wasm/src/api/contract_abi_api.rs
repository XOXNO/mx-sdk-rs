use super::{ErrorApi, StorageReadApi, StorageWriteApi};
use crate::abi::{ContractAbi, TypeAbi};

/// Required by contract ABI generators.
/// Provides the same associated types as the `ContractBase`,
/// so that associated types that show up in arguments and results match.
pub trait ContractAbiProvider {
	/// The generated ABI generation code uses the same types as the contract to provide `TypeAbi`s to endpoints.
	/// This associated type allows `Self::BigUint` to also make sense in the ABI context.
	type BigUint: TypeAbi;

	/// The generated ABI generation code uses the same types as the contract to provide `TypeAbi`s to endpoints.
	/// This associated type allows `Self::BigInt` to also make sense in the ABI context.
	type BigInt: TypeAbi;

	/// The generated ABI generation code uses the same types as the contract to provide `TypeAbi`s to endpoints.
	/// It sometimes references the contract storage manager type in with storage mappers,
	/// as for example in `SingleValueMapper<Self::Storage, i32>`.
	type Storage: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static;

	/// Associated function that provides the contract or module ABI.
	/// Since ABI generation is static, no state from the contract is required.
	fn abi() -> ContractAbi;
}
