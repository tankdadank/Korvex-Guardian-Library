use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

use crate::model::telemetry::TelemetryMode;

/// Errors that can occur when loading or validating Core configuration.
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("I/O error while reading config: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse YAML config: {0}")]
    Parse(#[from] serde_yaml::Error),

    #[error("Configuration validation error: {0}")]
    Validation(String),
}

/// Environment metadata for the node running Guardian Core.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentConfig {
    /// Logical environment, for example: "prod", "staging", "dev".
    pub env: String,

    /// Role of this node, for example: "edge", "bastion", "game-host".
    pub role: String,

    /// Location identifier, for example: "us-east-1", "eu-central".
    pub location: String,
}

/// Telemetry configuration for Guardian.
/// This controls how much telemetry is allowed to be emitted by Core.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryConfig {
    /// Telemetry mode: off, minimal, or debug.
    pub mode: TelemetryMode,

    /// Optional timeout for debug mode, in seconds.
    ///
    /// When mode == debug:
    ///   - this value must be Some and > 0
    /// For other modes:
    ///   - this value is ignored and may be None.
    #[serde(default)]
    pub debug_timeout_seconds: Option<u64>,
}

/// Controls whether Korvex extras are allowed locally.
///
/// When disabled, all gated features are treated as disabled,
/// regardless of capability snapshots from the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtrasConfig {
    /// Global kill switch for Korvex extras on this node.
    pub enabled: bool,
}

impl Default for ExtrasConfig {
    fn default() -> Self {
        ExtrasConfig { enabled: false }
    }
}

/// Local feature switches for Korvex extras.
///
/// Keys are strings like "ip.whitelist", "ip.geo_identity", etc.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GatedFeaturesConfig {
    /// Map of local feature toggles.
    ///
    /// If a key is absent, the local default is "allowed" (subject to ExtrasConfig
    /// and remote capability snapshots).
    pub features: HashMap<String, bool>,
}

/// Top-level Core configuration model used by Guardian Core.
///
/// This is intended to be loaded from a YAML file such as:
///   /etc/guardian/core.yml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreConfig {
    pub environment: EnvironmentConfig,
    pub telemetry: TelemetryConfig,

    #[serde(default)]
    pub extras: ExtrasConfig,

    #[serde(default)]
    pub gated_features: GatedFeaturesConfig,
}

impl CoreConfig {
    /// Load configuration from a YAML file and validate it.
    pub fn load_from_file(path: &Path) -> Result<Self, ConfigError> {
        let contents = fs::read_to_string(path)?;
        let cfg: CoreConfig = serde_yaml::from_str(&contents)?;
        cfg.validate()?;
        Ok(cfg)
    }

    /// Validate configuration values.
    ///
    /// This does not perform any network or API calls.
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Environment must be non-empty.
        if self.environment.env.trim().is_empty() {
            return Err(ConfigError::Validation(
                "environment.env must not be empty".into(),
            ));
        }
        if self.environment.role.trim().is_empty() {
            return Err(ConfigError::Validation(
                "environment.role must not be empty".into(),
            ));
        }
        if self.environment.location.trim().is_empty() {
            return Err(ConfigError::Validation(
                "environment.location must not be empty".into(),
            ));
        }

        // Telemetry: enforce debug timeout when in debug mode.
        if matches!(self.telemetry.mode, TelemetryMode::Debug) {
            let timeout = self.telemetry.debug_timeout_seconds.ok_or_else(|| {
                ConfigError::Validation(
                    "telemetry.debug_timeout_seconds must be set when telemetry.mode = \"debug\""
                        .into(),
                )
            })?;

            if timeout == 0 {
                return Err(ConfigError::Validation(
                    "telemetry.debug_timeout_seconds must be greater than 0".into(),
                ));
            }
        }

        Ok(())
    }

    /// Returns whether a gated feature is enabled locally.
    ///
    /// This is purely local evaluation:
    /// - Respects the global extras.enabled kill switch.
    /// - Respects the gated_features.features map.
    ///
    /// Remote capability snapshots (tier, revocation, etc.) are
    /// handled separately by CapabilitySnapshot.
    ///
    /// Feature key format: "module.feature", for example:
    ///   - "ip.whitelist"
    ///   - "ip.geo_identity"
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
