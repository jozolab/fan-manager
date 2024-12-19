use serde::{Deserialize, Serialize};
use crate::cpu::CPU;
use crate::fan::Fan;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub fan: Fan,
    pub cpu: CPU,
}