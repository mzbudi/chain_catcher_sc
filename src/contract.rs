#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use linera_sdk::{
    linera_base_types::WithContractAbi,
    views::{RootView, View},
    Contract, ContractRuntime,
};

use chain_catcher_sc::models::ScoreEntry;
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

            Operation::SetScore {
                chain_id,
                name,
                score,
            } => {
                let key = name.clone();

                match self.state.scores.get(&key).await {
                    Ok(Some(existing_entry)) => {
                        if score > existing_entry.score {
                            let entry = ScoreEntry {
                                chain_id,
                                name: key.clone(),
                                score,
                            };
                            self.state.scores.insert(&key, entry).unwrap_or_else(|_| {
                                panic!("Failed to update Play Data for {:?} - {:?}", name, score);
                            });
                        }
                    }
                    Ok(None) => {
                        let entry = ScoreEntry {
                            chain_id,
                            name: key.clone(),
                            score,
                        };
                        self.state.scores.insert(&key, entry).unwrap_or_else(|_| {
                            panic!("Failed to insert Play Data for {:?} - {:?}", name, score);
                        });
                    }
                    Err(e) => {
                        eprintln!("Error getting score for {}: {:?}", key, e);
                    }
                }
            }
        }
    }

    async fn execute_message(&mut self, _message: Self::Message) {}

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}
