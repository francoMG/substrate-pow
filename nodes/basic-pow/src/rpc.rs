#![warn(missing_docs)]

use std::sync::Arc;

use futures::channel::mpsc::Sender;
use runtime::{opaque::Block, Hash, AccountId, Balance, Index, BlockNumber};
use sc_consensus_manual_seal::{
	rpc::{ManualSeal, ManualSealApi},
	EngineCommand,
};
pub use sc_rpc_api::DenyUnsafe;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_transaction_pool::TransactionPool;

/// Full client dependencies.
pub struct FullDeps<C, P> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	/// A command stream to send authoring commands to manual seal consensus engine
	pub command_sink: Sender<EngineCommand<Hash>>,
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P>(deps: FullDeps<C, P>) -> jsonrpc_core::IoHandler<sc_rpc::Metadata>
where
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
	C: Send + Sync + 'static,
	C::Api: BlockBuilder<Block>,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
    C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	// C::Api: sum_storage_runtime_api::SumStorageApi<Block>,
	C::Api: pallet_contracts_rpc::ContractsRuntimeApi<Block, AccountId, Balance, BlockNumber>,
	P: TransactionPool + 'static,
{
	let mut io = jsonrpc_core::IoHandler::default();
	let FullDeps {
		command_sink,
		client,
		pool,
        deny_unsafe 
	} = deps;

    use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};
	use substrate_frame_rpc_system::{FullSystem, SystemApi};
	use pallet_contracts_rpc::{Contracts, ContractsApi};

	// Add a silly RPC that returns constant values
	// io.extend_with(crate::silly_rpc::SillyRpc::to_delegate(
	// 	crate::silly_rpc::Silly {},
	// ));

	// Add a second RPC extension
	// Because this one calls a Runtime API it needs a reference to the client.
	// io.extend_with(sum_storage_rpc::SumStorageApi::to_delegate(
	// 	sum_storage_rpc::SumStorage::new(client),
	// ));

    io.extend_with(SystemApi::to_delegate(FullSystem::new(client.clone(), pool, deny_unsafe)));

	io.extend_with(TransactionPaymentApi::to_delegate(TransactionPayment::new(client.clone())));

	io.extend_with(
        ContractsApi::to_delegate(Contracts::new(client.clone()))
    );

	// The final RPC extension receives commands for the manual seal consensus engine.
	io.extend_with(
		// We provide the rpc handler with the sending end of the channel to allow the rpc
		// send EngineCommands to the background block authorship task.
		ManualSealApi::to_delegate(ManualSeal::new(command_sink)),
	);

	io
}