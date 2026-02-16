pub mod config;
pub mod i18n;
pub mod build;
pub mod model;

// Core configuration
pub use config::core::{
    CoreConfig,
    TelemetryConfig,
    ExtrasConfig,
    GatedFeaturesConfig,
    ConfigError,
    EnvironmentConfig,
};

// Localization
pub use i18n::{I18n, Locale, I18N};

// Build fingerprinting
pub use build::fingerprint::{BuildFingerprint, BuildHash, BuildFingerprintError};

// Telemetry + capabilities
pub use model::telemetry::TelemetryMode;
pub use model::capabilities::{CapabilitySnapshot, FeatureFlags};
pub use model::modules::ModuleId;
pub use model::feature_gate::feature_allowed;
