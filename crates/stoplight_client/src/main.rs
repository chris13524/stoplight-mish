mod client_fn;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let seed_objects = stoplight_server::seed_objects::get_seed_objects();

    client_fn::client_fn(seed_objects).await
}
