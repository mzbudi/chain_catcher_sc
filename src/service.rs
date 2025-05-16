#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::sync::Arc;

use async_graphql::{EmptySubscription, Object, Schema};
use linera_sdk::{
    graphql::GraphQLMutationRoot, linera_base_types::WithServiceAbi, views::View, Service,
    ServiceRuntime,
};

use chain_catcher_sc::Operation;

use self::state::ChainCatcherScState;

pub struct ChainCatcherScService {
    state: ChainCatcherScState,
    runtime: Arc<ServiceRuntime<Self>>,
}

linera_sdk::service!(ChainCatcherScService);

impl WithServiceAbi for ChainCatcherScService {
    type Abi = chain_catcher_sc::ChainCatcherScAbi;
}

impl Service for ChainCatcherScService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = ChainCatcherScState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        ChainCatcherScService {
            state,
            runtime: Arc::new(runtime),
        }
    }

    async fn handle_query(&self, query: Self::Query) -> Self::QueryResponse {
        let value = *self.state.value.get();

        // Gunakan konstruktor baru untuk QueryRoot
        let query_root = QueryRoot::new(value, self.runtime.clone());

        Schema::build(
            query_root,
            Operation::mutation_root(self.runtime.clone()),
            EmptySubscription,
        )
        .finish()
        .execute(query)
        .await
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct ScoreEntry {
    pub name: String,
    pub score: u64,
}

struct QueryRoot {
    value: u64,
    runtime: Arc<ServiceRuntime<ChainCatcherScService>>,
}

impl QueryRoot {
    fn new(value: u64, runtime: Arc<ServiceRuntime<ChainCatcherScService>>) -> Self {
        QueryRoot { value, runtime }
    }
}

#[Object]
impl QueryRoot {
    async fn value(&self) -> u64 {
        self.value
    }

    async fn score(&self, name: String) -> Option<u64> {
        let state = ChainCatcherScState::load(self.runtime.root_view_storage_context())
            .await
            .ok()?;

        state.scores.get(&name).await.unwrap_or(None)
    }

    // async fn all_scores(&self) -> Vec<ScoreEntry> {
    //     let state = ChainCatcherScState::load(self.runtime.root_view_storage_context())
    //         .await
    //         .expect("Failed to load state");

    //     let mut entries = Vec::new();
    //     let names = state.names.read().await;
    //     for name in names {
    //         if let Some(score) = state.scores.get(&name).await.unwrap_or(None) {
    //             entries.push(ScoreEntry { name, score });
    //         }
    //     }

    //     entries
    // }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_graphql::{Request, Response, Value};
    use futures::FutureExt as _;
    use linera_sdk::{util::BlockingWait, views::View, Service, ServiceRuntime};
    use serde_json::json;

    use super::{ChainCatcherScService, ChainCatcherScState};

    #[test]
    fn query() {
        let value = 60u64;
        let runtime = Arc::new(ServiceRuntime::<ChainCatcherScService>::new());
        let mut state = ChainCatcherScState::load(runtime.root_view_storage_context())
            .blocking_wait()
            .expect("Failed to read from mock key value store");
        state.value.set(value);

        let service = ChainCatcherScService { state, runtime };
        let request = Request::new("{ value }");

        let response = service
            .handle_query(request)
            .now_or_never()
            .expect("Query should not await anything");

        let expected = Response::new(Value::from_json(json!({"value": 60})).unwrap());

        assert_eq!(response, expected)
    }
}
