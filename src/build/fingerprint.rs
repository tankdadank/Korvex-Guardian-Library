use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildHash {
    pub algo: String, // "sha256"
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildFingerprint {
    pub build_id: String,
    pub core_binary: BuildHash,
    pub kgl_library: BuildHash,
    pub modules: std::collections::HashMap<String, BuildHash>,
}

#[derive(Debug, Error)]
pub enum BuildFingerprintError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to compute hash: {0}")]
    Hash(String),
}

impl BuildHash {
    pub fn sha256_from_file(path: &Path) -> Result<Self, BuildFingerprintError> {
        let data = fs::read(path)?;
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let result = hasher.finalize();
        Ok(BuildHash {
            algo: "sha256".into(),
            value: hex::encode(result),
        })
    }
}

impl BuildFingerprint {
    /// Helper for creating a fingerprint from file paths.
    pub fn from_paths(
        build_id: String,
        core_bin: &Path,
        kgl_lib: &Path,
        module_paths: &[(String, &Path)],
    ) -> Result<Self, BuildFingerprintError> {
        let core_binary = BuildHash::sha256_from_file(core_bin)?;
        let kgl_library = BuildHash::sha256_from_file(kgl_lib)?;

        let mut modules = std::collections::HashMap::new();
        for (name, path) in module_paths {
            let hash = BuildHash::sha256_from_file(path)?;
            modules.insert(name.clone(), hash);
        }

        Ok(Self {
            build_id,
            core_binary,
            kgl_library,
            modules,
        })
    }
}
