pub mod config;
pub mod i18n;
pub mod build;
pub mod model;

pub use config::core::{CoreConfig, TelemetryConfig, ExtrasConfig, GatedFeaturesConfig};
pub use i18n::{I18n, Locale};
pub use build::fingerprint::{BuildFingerprint, BuildHash};
pub use model::telemetry::TelemetryMode;
pub use model::capabilities::{CapabilitySnapshot, FeatureFlags};
