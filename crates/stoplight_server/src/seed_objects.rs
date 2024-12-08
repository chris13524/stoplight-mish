use {
    alloy::primitives::{b256, B256},
    mish::{
        seed_objects::Objects,
        types::{Property, Strategy, Type, Value},
    },
    std::{collections::HashMap, time::Duration},
    url::Url,
};

#[derive(Clone)]
pub struct SeedObjects {
    pub on: B256,
    pub off: B256,
    pub stoplight: B256,
}

pub fn get_seed_objects() -> (HashMap<B256, serde_json::Value>, SeedObjects) {
    let _std_seed_objects = mish::seed_objects::get_std_seed_objects();

    let mut obj = Objects::new();

    let state = obj.add(Type {
        id: b256!("3e2246b310f9c7adeb64bc5a5eb50786e036a9d4efb1ca338a9f5bf70a15652f"),
    });
    let on = obj.add(Value {
        r#type: state,
        value: serde_json::json!(true),
    });
    let off = obj.add(Value {
        r#type: state,
        value: serde_json::json!(false),
    });

    let strategy = obj.add(Strategy::HttpPollGet {
        endpoint: Url::parse("http://127.0.0.1:3000").unwrap(),
        poll_interval: Duration::from_secs(1),
    });
    let stoplight = obj.add(Property {
        initial_value: on,
        strategy,
    });

    (obj.seed_objects, SeedObjects { on, off, stoplight })
}
