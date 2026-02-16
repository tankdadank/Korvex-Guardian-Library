use serde::{Serialize, Deserialize};

/// Current Guardian module supported by KGL.
///
/// For now only the IP module is defined.
/// New modules (ssh-hardening, port-surface, integrity, ...) will be added later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModuleId {
    /// IP module (whitelisting, geo, DNS, etc.).
    Ip,
}

impl ModuleId {
    /// Stable identifier used in configs and capability snapshots.
    pub fn as_str(&self) -> &'static str {
        match self {
            ModuleId::Ip => "ip",
        }
    }
}
