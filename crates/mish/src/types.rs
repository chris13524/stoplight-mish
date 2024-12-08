use {
    alloy::primitives::B256,
    serde::{Deserialize, Serialize},
    std::time::Duration,
    url::Url,
};

#[derive(Serialize, Deserialize)]
pub struct Property {
    pub initial_value: B256,
    pub strategy: B256,
}

#[derive(Serialize, Deserialize)]
pub enum Strategy {
    HttpPollGet {
        endpoint: Url,
        poll_interval: Duration,
    },
}

#[derive(Serialize, Deserialize)]
pub struct Update {
    pub new_value: B256,
    pub property: B256,
}

#[derive(Serialize, Deserialize)]
pub struct Type {
    // Unique identifier, use this to obtain:
    // dd if=/dev/urandom bs=1 count=32 2>/dev/null | xxd -p -c 32
    pub id: B256,
}

#[derive(Serialize, Deserialize)]
pub struct Value {
    pub r#type: B256,
    pub value: serde_json::Value,
}
