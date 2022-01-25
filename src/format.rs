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
impl FlatpakManifestFormat {
    pub fn from_path(file_path: &str) -> Option<FlatpakManifestFormat> {
        if file_path.to_lowercase().ends_with("yaml") || file_path.to_lowercase().ends_with("yml") {
            return Some(FlatpakManifestFormat::YAML);
        }
        if file_path.to_lowercase().ends_with("json") {
            return Some(FlatpakManifestFormat::JSON);
        }
        None
    }
}
