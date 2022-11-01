use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Setups {
    scripts: Vec<Setup>,
}

impl Setups {
    pub fn new(scripts: Vec<Setup>) -> Self {
        Self { scripts }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Setup {
    name: String,
    path: String,
}
