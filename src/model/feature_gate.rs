use crate::config::core::CoreConfig;
use crate::model::capabilities::CapabilitySnapshot;
use crate::model::modules::ModuleId;

/// Returns true if a Korvex extra feature is allowed.
///
/// Enforces:
/// - extras.enabled in core config must be true
/// - local kill switch for "module.feature" must be enabled
/// - capability snapshot must be present and allow the feature
///
/// For now, only ModuleId::Ip is defined.
/// Example feature keys:
///   "ip.whitelist"
///   "ip.geo_identity"
///   "ip.extended_history"
pub fn feature_allowed(
    cfg: &CoreConfig,
    snapshot: Option<&CapabilitySnapshot>,
    module: ModuleId,
    feature: &str,
) -> bool {
    let key = format!("{}.{}", module.as_str(), feature);

    // Global or feature-level disable
    if !cfg.is_feature_enabled_locally(&key) {
        return false;
    }

    let snapshot = match snapshot {
        Some(s) => s,
        None => return false,
    };

    snapshot.allows(&key)
}
