use async_graphql::{Request, Response};
use linera_sdk::{
    graphql::GraphQLMutationRoot,
    linera_base_types::{ChainId, ContractAbi, ServiceAbi},
};
use serde::{Deserialize, Serialize};
pub mod models;
use models::ScoreEntry;

pub struct ChainCatcherScAbi;

impl ContractAbi for ChainCatcherScAbi {
    type Operation = Operation;
    type Response = ();
}

impl ServiceAbi for ChainCatcherScAbi {
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Operation {
    Increment {
        value: u64,
    },
    SetScore {
        owner_chain_id: ChainId,
        chain_id: ChainId,
        name: String,
        score: u64,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ChainCatcherMessage {
    ScoreEntryMessage {
        chain_id: ChainId,
        name: String,
        score: u64,
    },
    LeaderboardRequest {
        requester_chain_id: ChainId,
    },
    LeaderboardResponse {
        leaderboard: Vec<ScoreEntry>,
    },
}

// #[derive(
//     Debug, Clone, Default, Deserialize, Eq, Ord, PartialOrd, PartialEq, Serialize, SimpleObject,
// )]
// pub struct Leaderboard {
//     pub name: String,
//     pub score: u64,
// }
