use std::{collections::{BTreeMap, HashMap}, fmt::Debug};

use alloy_primitives::{Address, Bytes, U256};
use ethers::{signers::LocalWallet, types::Log};
use foundry_cheatcodes::{BroadcastableTransactions, Cheatcodes};
use foundry_evm_core::{backend::LocalForkId, fork::CreateFork};
use revm::{
    interpreter::{InstructionResult, Stack},
    primitives::{Env, Output},
    JournaledState,
};
use zksync_basic_types::H160;

#[derive(Debug, Clone)]
pub enum ForkUrlType {
    Evm,
    Zk,
}

#[derive(Default, Debug, Clone)]
pub struct CachedForkUrlType(HashMap<String, ForkUrlType>);

impl CachedForkUrlType {
    pub fn get(&mut self, fork: &CreateFork) -> ForkUrlType {
        if let Some(fork_url_type) = self.0.get(&fork.url) {
            return fork_url_type.clone()
        }

        let is_zk_url = foundry_common::try_get_http_provider(&fork.url)
            .and_then(|provider| {
                let is_zk_url = tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(provider.request("zks_getTokenPrice", [H160::zero()]))
                    .map(|_: String| true)
                    .unwrap_or_default();

                Ok(is_zk_url)
            })
            .unwrap_or_default();

        let fork_url_type = if is_zk_url { ForkUrlType::Zk } else { ForkUrlType::Evm };
        println!("fork_url = {} {:?}", fork.url, fork_url_type);
        self.0.insert(fork.url.clone(), fork_url_type.clone());

        fork_url_type
    }
}

pub trait EvmExecutor: Debug + Clone {
    fn create_fork(&mut self, fork: CreateFork) -> eyre::Result<LocalForkId>;
    fn select_fork(
        &mut self,
        id: LocalForkId,
        env: &mut Env,
        journaled_state: &mut JournaledState,
    ) -> eyre::Result<()>;
    fn create_select_fork(
        &mut self,
        fork: CreateFork,
        env: &mut Env,
        journaled_state: &mut JournaledState,
    ) -> eyre::Result<LocalForkId> {
        let id = self.create_fork(fork)?;
        self.select_fork(id, env, journaled_state)?;
        Ok(id)
    }
    fn call_evm(
        &mut self,
        from: Address,
        to: Address,
        calldata: Bytes,
        value: U256,
    ) -> eyre::Result<EvmRawCallResult>;
}

#[derive(Debug)]
pub struct EvmRawCallResult {
    /// The status of the call
    pub exit_reason: InstructionResult,
    /// Whether the call reverted or not
    pub reverted: bool,
    /// Whether the call includes a snapshot failure
    ///
    /// This is tracked separately from revert because a snapshot failure can occur without a
    /// revert, since assert failures are stored in a global variable (ds-test legacy)
    pub has_snapshot_failure: bool,
    /// The raw result of the call
    pub result: Bytes,
    /// The gas used for the call
    pub gas_used: u64,
    /// Refunded gas
    pub gas_refunded: u64,
    /// The initial gas stipend for the transaction
    pub stipend: u64,
    /// The logs emitted during the call
    pub logs: Vec<Log>,
    /// The labels assigned to addresses during the call
    pub labels: BTreeMap<Address, String>,
    /// Scripted transactions generated from this call
    pub transactions: Option<BroadcastableTransactions>,
    /// The changeset of the state.
    ///
    /// This is only present if the changed state was not committed to the database (i.e. if you
    /// used `call` and `call_raw` not `call_committing` or `call_raw_committing`).
    pub state_changeset: Option<foundry_evm_core::utils::StateChangeset>,
    /// The wallets added during the call using the `rememberKey` cheatcode
    pub script_wallets: Vec<LocalWallet>,
    /// The `revm::Env` after the call
    pub env: Env,
    /// The cheatcode states after execution
    pub cheatcodes: Option<Cheatcodes>,
    /// The raw output of the execution
    pub out: Option<Output>,
    /// The chisel state
    pub chisel_state: Option<(Stack, Vec<u8>, InstructionResult)>,
}
