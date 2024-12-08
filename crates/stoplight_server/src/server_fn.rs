use {
    crate::{seed_objects::SeedObjects, state::AppState},
    alloy::primitives::B256,
    serde_json::Value,
    std::{collections::HashMap, time::Duration},
    tracing::info,
};

pub async fn server_fn(shared_state: AppState, seed_objects: (HashMap<B256, Value>, SeedObjects)) {
    loop {
        info!("Setting stoplight to on");
        (*shared_state.property_store.write().await)
            .insert(seed_objects.1.stoplight, seed_objects.1.on);
        tokio::time::sleep(Duration::from_secs(1)).await;

        info!("Setting stoplight to off");
        (*shared_state.property_store.write().await)
            .insert(seed_objects.1.stoplight, seed_objects.1.off);
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
