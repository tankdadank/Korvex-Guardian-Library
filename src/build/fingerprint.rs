use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildFingerprintError {
    #[error("I/O error while reading file: {0}")]
    Io(#[from] std::io::Error),

    #[error("Hash computation error: {0}")]
    Hash(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildHash {
    pub algo: String,
    pub value: String,
}

impl BuildHash {
    pub fn sha256_from_bytes(data: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();

        BuildHash {
            algo: "sha256".into(),
            value: hex::encode(result),
        }
    }

    pub fn sha256_from_file(path: &Path) -> Result<Self, BuildFingerprintError> {
        let mut file = File::open(path)?;
        let mut hasher = Sha256::new();
        let mut buf = [0u8; 8192];

        loop {
            let n = file.read(&mut buf)?;
            if n == 0 {
                break;
            }
            hasher.update(&buf[..n]);
        }

        let result = hasher.finalize();
        Ok(BuildHash {
            algo: "sha256".into(),
            value: hex::encode(result),
        })
    }
}

/// Fingerprint of a Guardian deployment:
/// - Core binary
/// - KGL library
/// - Individual modules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildFingerprint {
    pub build_id: String,
    pub core_binary: BuildHash,
    pub kgl_library: BuildHash,
    pub modules: HashMap<String, BuildHash>,
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

        let mut modules = HashMap::new();
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
