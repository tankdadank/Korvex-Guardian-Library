use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::fs;
use std::path::Path;

use crate::model::telemetry::TelemetryMode;

/// Core Guardian configuration as parsed from core.yml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreConfig {
    pub environment: EnvironmentConfig,
    pub telemetry: TelemetryConfig,
    pub extras: ExtrasConfig,

    #[serde(default)]
    pub gated_features: GatedFeaturesConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentConfig {
    pub env: String,
    pub role: String,
    pub location: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryConfig {
    pub mode: TelemetryMode,
    #[serde(default)]
    pub debug_timeout: Option<String>, // e.g. "1h"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtrasConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GatedFeaturesConfig {
    /// Map "module.feature" -> bool
    pub features: std::collections::HashMap<String, bool>,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse YAML: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("Validation error: {0}")]
    Validation(String),
}

impl CoreConfig {
    /// Load core configuration from a YAML file.
    pub fn load_from_file(path: &Path) -> Result<Self, ConfigError> {
        let data = fs::read_to_string(path)?;
        let cfg: CoreConfig = serde_yaml::from_str(&data)?;
        cfg.validate()?;
        Ok(cfg)
    }

    /// Strict validation. Rejects invalid or unsafe values.
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Telemetry: debug mode must have timeout.
        if matches!(self.telemetry.mode, TelemetryMode::Debug) && self.telemetry.debug_timeout.is_none() {
            return Err(ConfigError::Validation(
                "telemetry.debug_timeout must be set when telemetry.mode = \"debug\"".into(),
            ));
        }

        // Environment: simple non-empty checks.
        if self.environment.env.trim().is_empty() {
            return Err(ConfigError::Validation("environment.env must not be empty".into()));
        }
        if self.environment.role.trim().is_empty() {
            return Err(ConfigError::Validation("environment.role must not be empty".into()));
        }
        if self.environment.location.trim().is_empty() {
            return Err(ConfigError::Validation("environment.location must not be empty".into()));
        }

        Ok(())
    }

    /// Returns whether a gated feature is enabled locally.
    /// Feature key format: "module.feature"
    pub fn is_feature_enabled_locally(&self, key: &str) -> bool {
        if !self.extras.enabled {
            return false;
        }

        self.gated_features
            .features
            .get(key)
            .copied()
            .unwrap_or(true)
    }
}
