use {
    axum::{routing::get, Router},
    state::AppState,
    std::{collections::HashMap, sync::Arc},
    tokio::sync::RwLock,
};

mod handlers;
mod seed_objects;
mod server_fn;
mod state;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let shared_state = AppState {
        object_store: HashMap::new(),
        property_store: Arc::new(RwLock::new(HashMap::new())),
    };

    let seed_objects = seed_objects::get_seed_objects();

    tokio::task::spawn(server_fn::server_fn(
        shared_state.clone(),
        seed_objects.clone(),
    ));

    let app = Router::new()
        .route("/object/:object_hash", get(handlers::get_object::handler))
        .route(
            "/property/:property_hash",
            get(handlers::get_property::handler),
        )
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
