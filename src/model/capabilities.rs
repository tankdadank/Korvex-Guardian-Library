use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Snapshot of what the Korvex API says this host + build is allowed to do.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitySnapshot {
    /// Logical identifier for this host from the API.
    pub host_id: String,

    /// Build identifier (from Core).
    pub build_id: String,

    /// Optional entitlement tier, for example: "supporter", "enthusiast", "insider".
    pub tier: Option<String>,

    /// Feature flags as decided by the API.
    pub features: FeatureFlags,

    /// If true, this build/host combination is revoked. Extras must be disabled.
    pub revoked: bool,
}

/// Feature flags grouped by scope.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeatureFlags {
    /// Global keys, for example: "ip.whitelist".
    pub global: HashMap<String, bool>,

    /// Module-specific feature flags: module -> feature -> allowed.
    /// Example: "ip" -> ("whitelist" -> true).
    pub modules: HashMap<String, HashMap<String, bool>>,
}

impl CapabilitySnapshot {
    /// Returns whether a given feature key is allowed by this snapshot.
    ///
    /// The key may be either:
    ///   - A full key: "ip.whitelist"
    ///   - Evaluated via the (module, feature) mapping.
    pub fn allows(&self, key: &str) -> bool {
        if self.revoked {
            return false;
        }

        // First check global table, if present.
        if let Some(value) = self.features.global.get(key) {
            return *value;
        }

        // Then interpret as "module.feature".
        let mut parts = key.split('.');
        let module = match parts.next() {
            Some(m) => m,
            None => return false,
        };
        let feature = match parts.next() {
            Some(f) => f,
            None => return false,
        };

        if let Some(module_map) = self.features.modules.get(module) {
            if let Some(value) = module_map.get(feature) {
                return *value;
            }
        }

        false
    }
}
