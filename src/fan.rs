use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Fan {
    pub device: String,
    pub min: u32,
    pub max: u32,
    pub step: u32,
    pub interval: u32
}