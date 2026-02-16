use crate::config::core::CoreConfig;

use super::capabilities::CapabilitySnapshot;
use super::modules::ModuleId;

/// Evaluate whether a given module feature is allowed.
///
/// This function combines:
///   1. Local configuration (CoreConfig, including extras + gated_features)
///   2. Optional remote CapabilitySnapshot (tier / revocation / feature flags)
///
/// `feature` is the feature name without the module prefix, for example:
///   - "whitelist"
///   - "geo_identity"
pub fn feature_allowed(
    cfg: &CoreConfig,
    snapshot: Option<&CapabilitySnapshot>,
    module: ModuleId,
    feature: &str,
) -> bool {
    let full_key = format!("{}.{}", module.as_str(), feature);

    // Local kill switches.
    if !cfg.is_feature_enabled_locally(&full_key) {
        return false;
    }

    // Remote capability snapshot (if present).
    if let Some(snap) = snapshot {
        if !snap.allows(&full_key) {
            return false;
        }
    }

    true
}
