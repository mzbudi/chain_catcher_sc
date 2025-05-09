use async_graphql::{Request, Response};
use linera_sdk::{
    graphql::GraphQLMutationRoot,
    linera_base_types::{ContractAbi, ServiceAbi},
};
use serde::{Deserialize, Serialize};

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
    Increment { value: u64 },
    SetScore { name: String, score: u64 },
}
