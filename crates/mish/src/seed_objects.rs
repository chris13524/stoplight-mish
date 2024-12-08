use {
    alloy::primitives::{keccak256, B256},
    serde::Serialize,
    std::collections::HashMap,
};

pub struct Objects {
    pub seed_objects: HashMap<B256, serde_json::Value>,
}

impl Objects {
    pub fn new() -> Self {
        Self {
            seed_objects: HashMap::new(),
        }
    }

    pub fn add(&mut self, obj: impl Serialize) -> B256 {
        let hash = keccak256(serde_json::to_vec(&obj).unwrap());
        let obj = serde_json::to_value(obj).unwrap();
        self.seed_objects.insert(hash, obj);
        hash
    }
}

impl Default for Objects {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct StdSeedObjects {}

pub fn get_std_seed_objects() -> (HashMap<B256, serde_json::Value>, StdSeedObjects) {
    let obj = Objects::new();
    (obj.seed_objects, StdSeedObjects {})
}
