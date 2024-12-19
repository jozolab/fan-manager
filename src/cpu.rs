use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct CPU {
    pub min: u32,
    pub max: u32,
}