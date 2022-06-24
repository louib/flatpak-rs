use serde::{Deserialize, Serialize};

pub const APPLICATION: &str = "application";
pub const MODULE: &str = "module";
pub const SOURCE: &str = "source";

/// All the Flatpak manifest types supported.
#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(Debug)]
pub enum FlatpakManifestType {
    Application,
    Module,
    Source,
}
impl FlatpakManifestType {
    pub fn to_string(&self) -> String {
        match &self {
            FlatpakManifestType::Application => APPLICATION.to_string(),
            FlatpakManifestType::Module => MODULE.to_string(),
            FlatpakManifestType::Source => SOURCE.to_string(),
        }
    }

    pub fn from_string(manifest_type: &str) -> Result<FlatpakManifestType, String> {
        if manifest_type == APPLICATION {
            return Ok(FlatpakManifestType::Application);
        }
        if manifest_type == MODULE {
            return Ok(FlatpakManifestType::Module);
        }
        if manifest_type == SOURCE {
            return Ok(FlatpakManifestType::Source);
        }
        Err(format!("Invalid manifest type {}.", manifest_type))
    }
}
