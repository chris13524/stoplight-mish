use {
    crate::state::AppState,
    alloy::primitives::B256,
    axum::{
        extract::{Path, State},
        Json,
    },
};

#[axum::debug_handler]
pub async fn handler(
    Path(property_hash): Path<B256>,
    State(state): State<AppState>,
) -> Json<Option<B256>> {
    Json(
        state
            .property_store
            .read()
            .await
            .get(&property_hash)
            .cloned(),
    )
}
