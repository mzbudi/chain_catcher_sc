use async_graphql::{Request, Response, SimpleObject};
use linera_sdk::{
    graphql::GraphQLMutationRoot,
    linera_base_types::{ChainId, ContractAbi, ServiceAbi},
};
use serde::{Deserialize, Serialize};
pub mod models;

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
        chain_id: ChainId,
        name: String,
        score: u64,
    },
}

// #[derive(
//     Debug, Clone, Default, Deserialize, Eq, Ord, PartialOrd, PartialEq, Serialize, SimpleObject,
// )]
// pub struct Leaderboard {
//     pub name: String,
//     pub score: u64,
// }
