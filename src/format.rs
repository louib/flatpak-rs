use serde::{Deserialize, Serialize};

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(Hash)]
#[derive(Debug)]
/// All denominations of Flatpak manifests (app manifests, module manifests and source manifests)
/// can use either YAML or JSON.
pub enum FlatpakManifestFormat {
    YAML,
    JSON,
}
impl Default for FlatpakManifestFormat {
    fn default() -> Self {
        FlatpakManifestFormat::YAML
    }
}
