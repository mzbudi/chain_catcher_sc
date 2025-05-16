#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use linera_sdk::{
    linera_base_types::WithContractAbi,
    views::{RootView, View},
    Contract, ContractRuntime,
};

use chain_catcher_sc::Operation;

use self::state::ChainCatcherScState;

pub struct ChainCatcherScContract {
    state: ChainCatcherScState,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(ChainCatcherScContract);

impl WithContractAbi for ChainCatcherScContract {
    type Abi = chain_catcher_sc::ChainCatcherScAbi;
}

impl Contract for ChainCatcherScContract {
    type Message = ();
    type Parameters = ();
    type InstantiationArgument = u64;
    type EventValue = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = ChainCatcherScState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        ChainCatcherScContract { state, runtime }
    }

    async fn instantiate(&mut self, argument: Self::InstantiationArgument) {
        // validate that the application parameters were configured correctly.
        self.runtime.application_parameters();
        self.state.value.set(argument);
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        match operation {
            Operation::Increment { value } => {
                self.state.value.set(self.state.value.get() + value);
            }

            Operation::SetScore { name, score } => match self.state.scores.get(&name).await {
                Ok(Some(existing_score)) => {
                    if existing_score < score {
                        self.state.scores.insert(&name, score).unwrap();
                        // self.state.names.insert(&name).unwrap();
                    }
                }
                Ok(None) => {
                    self.state.scores.insert(&name, score).unwrap();
                    // self.state.names.insert(&name).unwrap();
                }
                Err(e) => {
                    eprintln!("Error getting score for {}: {:?}", name, e);
                }
            },
        }
    }

    async fn execute_message(&mut self, _message: Self::Message) {}

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}
