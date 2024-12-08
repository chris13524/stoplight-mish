use {
    crate::{seed_objects::get_seed_objects, state::AppState},
    alloy::primitives::B256,
    axum::{
        extract::{Path, State},
        Json,
    },
    serde_json::Value,
};

#[axum::debug_handler]
pub async fn handler(Path(object_hash): Path<B256>, State(state): State<AppState>) -> Json<Value> {
    if let Some(value) = state.object_store.get(&object_hash) {
        return Json(value.clone());
    }

    Json(get_seed_objects().0.get(&object_hash).unwrap().clone())
}
