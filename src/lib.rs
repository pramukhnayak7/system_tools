use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemStats {
    pub cpu: f32,
    pub memory_mb: u64,
    pub timestamp: u64,
}
