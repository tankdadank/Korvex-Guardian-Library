use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Minimal capability snapshot model.
/// Signatures and cryptographic checks are handled in higher layers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitySnapshot {
    pub host_id: String,
    pub build_id: String,
    pub tier: String, // "none" | "supporter" | "enthusiast" | "insider"

    pub issued_at: String,
    pub expires_at: String,

    pub features: FeatureFlags,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeatureFlags {
    pub global: HashMap<String, bool>,
    pub modules: HashMap<String, HashMap<String, bool>>, // module -> feature -> bool
}

impl CapabilitySnapshot {
    /// Returns whether a given "module.feature" is allowed by snapshot.
    pub fn allows(&self, key: &str) -> bool {
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
