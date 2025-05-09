use linera_sdk::views::{linera_views, MapView, RegisterView, RootView, ViewStorageContext};

#[derive(RootView, async_graphql::SimpleObject)]
#[view(context = "ViewStorageContext")]
pub struct ChainCatcherScState {
    pub value: RegisterView<u64>,
    pub scores: MapView<String, u64>,
}