// src/models.rs
use async_graphql::SimpleObject;
use linera_sdk::linera_base_types::ChainId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct ScoreEntry {
    pub chain_id: ChainId,
    pub name: String,
    pub score: u64,
}