use serde::de::DeserializeOwned;
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
    #[cfg(feature = "toml")]
    TOML,
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
        #[cfg(feature = "toml")]
        if file_path.to_lowercase().ends_with("toml") {
            return Some(FlatpakManifestFormat::TOML);
        }
        None
    }

    pub fn parse<T>(&self, content: &str) -> Result<T, String>
    where
        T: DeserializeOwned,
    {
        match self {
            FlatpakManifestFormat::YAML => serde_yaml::from_str::<T>(content).map_err(|e| e.to_string()),
            FlatpakManifestFormat::JSON => {
                let json_content_without_comments = crate::utils::remove_comments_from_json(content);
                serde_json::from_str::<T>(&json_content_without_comments).map_err(|e| e.to_string())
            }
            #[cfg(feature = "toml")]
            FlatpakManifestFormat::TOML => toml::from_str::<T>(content).map_err(|e| e.to_string()),
        }
    }
}
