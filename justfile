fmt:
  cargo +nightly fmt -- --config group_imports=StdExternalCrate,imports_granularity=One

server:
  cargo run --bin stoplight_server

client:
  cargo run --bin stoplight_client
