use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TelemetryMode {
    Off,
    Minimal,
    Debug,
}

impl Default for TelemetryMode {
    fn default() -> Self {
        TelemetryMode::Minimal
    }
}
