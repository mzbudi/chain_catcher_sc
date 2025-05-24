#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use linera_sdk::{
    linera_base_types::WithContractAbi,
    views::{RootView, View},
    Contract, ContractRuntime,
};

use chain_catcher_sc::models::ScoreEntry;
use chain_catcher_sc::ChainCatcherMessage;
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
    type Message = ChainCatcherMessage;
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
                owner_chain_id,
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
                let message = ChainCatcherMessage::ScoreEntryMessage {
                    chain_id,
                    name,
                    score,
                };

                self.runtime
                    .prepare_message(message)
                    .send_to(owner_chain_id);
            }

            Operation::RequestLeaderboard { central_chain_id } => {
                let message = ChainCatcherMessage::LeaderboardRequest {
                    requester_chain_id: self.runtime.chain_id().clone(),
                };

                self.runtime
                    .prepare_message(message)
                    .send_to(central_chain_id);
            }
        }
    }

    async fn execute_message(&mut self, _message: Self::Message) {
        let is_bouncing = self.runtime.message_is_bouncing().unwrap_or_else(|| {
            panic!("Message delivery status has to be available when executing a message");
        });

        if is_bouncing {
            return;
        }

        match _message {
            ChainCatcherMessage::ScoreEntryMessage {
                chain_id,
                name,
                score,
            } => {
                let mut leaderboard = self.state.leaderboard.get().clone();

                match leaderboard.iter_mut().find(|entry| entry.name == name) {
                    Some(existing_entry) => {
                        if score > existing_entry.score {
                            existing_entry.score = score;
                            existing_entry.chain_id = chain_id;
                        }
                    }
                    None => {
                        leaderboard.push(ScoreEntry {
                            chain_id,
                            name: name.clone(),
                            score,
                        });
                    }
                }

                leaderboard.sort_unstable_by(|a, b| b.score.cmp(&a.score));

                self.state.leaderboard.set(leaderboard);
            }

            ChainCatcherMessage::LeaderboardRequest { requester_chain_id } => {
                let leaderboard = self.state.leaderboard.get().clone();

                let message = ChainCatcherMessage::LeaderboardResponse { leaderboard };

                self.runtime
                    .send_message(requester_chain_id.clone(), message)
            }

            ChainCatcherMessage::LeaderboardResponse { leaderboard } => {
                // Chain lokal: update leaderboard dari response
                self.state.leaderboard.set(leaderboard);
            }
        }
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}
