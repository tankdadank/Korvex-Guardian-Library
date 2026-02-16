use serde::{Deserialize, Serialize};

/// Guardian modules known to KGL.
///
/// At this stage, only the IP module is defined.
/// Additional modules can be added as the Guardian stack grows.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ModuleId {
    Ip,
}

impl ModuleId {
    /// Stable string identifier for this module, used in feature keys.
    pub fn as_str(&self) -> &'static str {
        match self {
            ModuleId::Ip => "ip",
        }
    }
}
