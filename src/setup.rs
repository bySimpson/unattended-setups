use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Setups {
    pub scripts: Vec<Setup>,
}

impl Setups {
    pub fn new(scripts: Vec<Setup>) -> Self {
        Self { scripts }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Setup {
    pub name: String,
    pub path: String,
}
