use {
    crate::{
        seed_objects::SeedObjects,
        types::{Property, Strategy},
    },
    alloy::{primitives::B256, transports::http::reqwest},
    serde_json::Value,
    std::collections::HashMap,
};

pub async fn client_fn(seed_objects: (HashMap<B256, Value>, SeedObjects)) {
    let property = serde_json::from_value::<Property>(
        seed_objects
            .0
            .get(&seed_objects.1.stoplight)
            .unwrap()
            .clone(),
    )
    .unwrap();
    let strategy =
        serde_json::from_value::<Strategy>(seed_objects.0.get(&property.strategy).unwrap().clone())
            .unwrap();
    match strategy {
        crate::types::Strategy::HttpPollGet {
            endpoint,
            poll_interval,
        } => loop {
            let response = reqwest::get(
                endpoint
                    .join(&format!("/property/{}", seed_objects.1.stoplight))
                    .unwrap(),
            )
            .await
            .unwrap()
            .json::<B256>()
            .await
            .unwrap();
            let state = match response {
                x if x == seed_objects.1.on => true,
                x if x == seed_objects.1.off => false,
                _ => panic!("Invalid response: {response}"),
            };
            println!("state: {state}");

            tokio::time::sleep(poll_interval).await;
        },
    }
}
