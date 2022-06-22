use serde::{Deserialize, Serialize};

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
