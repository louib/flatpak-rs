use std::collections::BTreeMap;
use std::fs;
use std::path;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref SOURCE_TYPES: Vec<String> = vec![
        "archive".to_string(),
        "git".to_string(),
        "bzr".to_string(),
        "svn".to_string(),
        "dir".to_string(),
        "file".to_string(),
        "script".to_string(),
        "shell".to_string(),
        "patch".to_string(),
        "extra-data".to_string(),
    ];
    pub static ref CODE_TYPES: Vec<String> = vec![
        "archive".to_string(),
        "git".to_string(),
        "bzr".to_string(),
        "svn".to_string(),
        "dir".to_string(),
    ];
    pub static ref VCS_TYPES: Vec<String> = vec!["git".to_string(), "bzr".to_string(), "svn".to_string(),];
}

pub const DEFAULT_SOURCE_TYPE: &str = "archive";

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(Debug)]
#[derive(Hash)]
#[serde(rename_all = "kebab-case")]
#[serde(untagged)]
/// The sources are a list pointer to the source code that needs to be extracted into
/// the build directory before the build starts.
/// They can be of several types, distinguished by the type property.
///
/// Additionally, the sources list can contain a plain string, which is interpreted as the name
/// of a separate json or yaml file that is read and inserted at this
/// point. The file can contain a single source, or an array of sources.
pub enum FlatpakSource {
    Path(String),
    Description(FlatpakSourceDescription),
}
impl FlatpakSource {
    pub fn get_url(&self) -> Option<String> {
        let source_description = match self {
            FlatpakSource::Path(_) => return None,
            FlatpakSource::Description(sd) => sd,
        };
        match &source_description.url {
            Some(s) => Some(s.to_string()),
            None => None,
        }
    }

    pub fn get_all_mirror_urls(&self) -> Vec<String> {
        let mut response: Vec<String> = vec![];

        let source_description = match self {
            FlatpakSource::Path(_) => return response,
            FlatpakSource::Description(sd) => sd,
        };
        if let Some(urls) = &source_description.mirror_urls {
            for url in urls {
                response.push(url.to_string());
            }
        }
        return response;
    }

    pub fn get_all_urls(&self) -> Vec<String> {
        let mut response: Vec<String> = vec![];
        let source_description = match self {
            FlatpakSource::Path(_) => return response,
            FlatpakSource::Description(sd) => sd,
        };
        if let Some(url) = &source_description.url {
            response.push(url.to_string());
        }
        if let Some(urls) = &source_description.mirror_urls {
            for url in urls {
                response.push(url.to_string());
            }
        }
        return response;
    }

    pub fn get_type_name(&self) -> String {
        return match self {
            FlatpakSource::Path(_) => "path".to_string(),
            FlatpakSource::Description(d) => {
                if let Some(t) = &d.r#type {
                    return t.to_string();
                }
                return "empty".to_string();
            }
        };
    }

    pub fn has_commit(&self) -> bool {
        return match self {
            FlatpakSource::Path(_) => false,
            FlatpakSource::Description(d) => d.commit.is_some(),
        };
    }

    pub fn has_tag(&self) -> bool {
        return match self {
            FlatpakSource::Path(_) => false,
            FlatpakSource::Description(d) => d.tag.is_some(),
        };
    }

    pub fn has_branch(&self) -> bool {
        return match self {
            FlatpakSource::Path(_) => false,
            FlatpakSource::Description(d) => d.branch.is_some(),
        };
    }

    pub fn type_is_valid(&self) -> bool {
        return match self {
            FlatpakSource::Path(_) => true,
            FlatpakSource::Description(d) => {
                if let Some(t) = &d.r#type {
                    return SOURCE_TYPES.contains(&t);
                }
                return false;
            }
        };
    }

    pub fn type_is_empty(&self) -> bool {
        return match self {
            FlatpakSource::Path(_) => false,
            FlatpakSource::Description(d) => d.r#type.is_none(),
        };
    }

    pub fn supports_mirror_urls(&self) -> bool {
        let type_name = self.get_type_name();
        // FIXME why are mirror urls not supported for types git, svn and bzr.
        if type_name == "archive" || type_name == "file" {
            return true;
        }
        return false;
    }
}

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(Debug)]
#[derive(Default)]
#[derive(Hash)]
#[serde(rename_all = "kebab-case")]
/// These contain a pointer to the source that will be extracted into the
/// source directory before the build starts. They can be of several types,
/// distinguished by the type property.
pub struct FlatpakSourceDescription {
    /// Defines the type of the source description.
    /// It is not explicit in the flatpak-manifest man page,
    /// but we only found 1 source in all our dataset with an empty
    /// type, so we assume that the field is actually required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// An array of shell commands.
    /// types: script, shell
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<String>>,

    /// Filename to use inside the source dir.
    /// types: script, archive, file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_filename: Option<String>,

    /// The name to use for the downloaded extra data
    /// types: extra-data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    /// The url to the resource.
    /// types: extra-data, svn, bzr, git, archive, file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// A list of alternative urls that are used if the main url fails.
    /// types: archive, file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirror_urls: Option<Vec<String>>,

    /// The md5 checksum of the file, verified after download
    /// Note that md5 is no longer considered a safe checksum, we recommend you use at least sha256.
    /// types: archive, file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5: Option<String>,

    /// The sha1 checksum of the file, verified after download
    /// Note that sha1 is no longer considered a safe checksum, we recommend you use at least sha256.
    /// types: archive, file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha1: Option<String>,

    /// The sha256 of the resource.
    /// types: extra-data, archive, file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,

    /// The sha512 checksum of the file, verified after download
    /// types: archive, file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha512: Option<String>,

    /// The size of the extra data in bytes.
    /// types: extra-data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,

    /// Whether to initialise the repository as a git repository.
    /// types: archive
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_init: Option<bool>,

    /// The extra installed size this adds to the app (optional).
    /// types: extra-data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_size: Option<i64>,

    /// A specific revision number to use
    /// types: svn, bzr
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,

    /// The branch to use from the git repository
    /// types: git
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,

    /// The type of archive if it cannot be guessed from the path.
    /// Possible values are:
    ///   * "rpm",
    ///   * "tar",
    ///   * "tar-gzip",
    ///   * "tar-compress",
    ///   * "tar-bzip2",
    ///   * "tar-lzip",
    ///   * "tar-lzma",
    ///   * "tar-lzop",
    ///   * "tar-xz",
    ///   * "zip",
    ///   * "7z",
    /// types: archive
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_type: Option<String>,

    /// The commit to use from the git repository.
    /// If branch is also specified, then it is verified that the branch/tag is at this specific commit.
    /// This is a readable way to document that you're using a particular tag,
    /// but verify that it does not change.
    /// types: git
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,

    /// The tag to use from the git repository
    /// types: git
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    /// The path to associated with the resource.
    /// types: git, archive, dir, patch, file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// An list of paths to a patch files that will be applied in the source dir, in order
    /// types: patch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,

    /// Whether to use "git apply" rather than "patch" to apply the patch, required when the
    /// patch file contains binary diffs.
    /// types: patch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_git: Option<bool>,

    /// Whether to use "git am" rather than "patch" to apply the patch, required when the patch
    /// file contains binary diffs.
    /// You cannot use this at the same time as use-git.
    /// types: patch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_git_am: Option<bool>,

    /// Extra options to pass to the patch command.
    /// types: patch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,

    /// Don't use transfer.fsckObjects=1 to mirror git repository. This may be needed for some
    /// (broken) repositories.
    /// types: git
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_fsckobjects: Option<bool>,

    /// Don't optimize by making a shallow clone when downloading the git repo.
    /// types: git
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_shallow_clone: Option<bool>,

    /// Don't checkout the git submodules when cloning the repository.
    /// types: git
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_submodules: Option<bool>,

    /// The number of initial pathname components to strip.
    /// defaults to 1.
    /// types: archive, patch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strip_components: Option<i64>,

    /// Source files to ignore in the directory.
    /// types: dir
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<Vec<String>>,

    /// If non-empty, only build the module on the arches listed.
    /// types: all
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_arches: Option<Vec<String>>,

    /// Don't build on any of the arches listed.
    /// types: all
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_arches: Option<Vec<String>>,

    /// Directory inside the source dir where this source will be extracted.
    /// types: all
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_checker_data: Option<FlatpakDataCheckerConfig>,
}
impl FlatpakSourceDescription {
    // The logic of this functions is based on the code from BuilderArchiveType.get_type
    // in the flatpak-builder project.
    pub fn detect_archive_type(url: &str) -> Option<String> {
        let url = url.to_lowercase();
        if url.ends_with(".tar") {
            return Some("tar".to_string());
        }
        if url.ends_with(".tar.gz") || url.ends_with(".tgz") || url.ends_with(".taz") {
            return Some("tar-gzip".to_string());
        }
        if url.ends_with(".tar.z") || url.ends_with(".taz") {
            return Some("tar-compress".to_string());
        }
        if url.ends_with(".tar.bz2") || url.ends_with(".tz2") {
            return Some("tar-bzip2".to_string());
        }
        if url.ends_with(".tbz2") || url.ends_with(".tbz") {
            return Some("tar-bzip2".to_string());
        }
        if url.ends_with(".tar.lz") {
            return Some("tar-lzip".to_string());
        }
        if url.ends_with(".tar.lzma") || url.ends_with(".tlz") {
            return Some("tar-lzma".to_string());
        }
        if url.ends_with(".tar.lzo") {
            return Some("tar-lzop".to_string());
        }
        if url.ends_with(".tar.xz") || url.ends_with(".txz") {
            return Some("tar-xz".to_string());
        }
        if url.ends_with(".zip") {
            return Some("zip".to_string());
        }
        if url.ends_with(".rpm") {
            return Some("rpm".to_string());
        }
        if url.ends_with(".7z") {
            return Some("sevenz".to_string());
        }
        None
    }

    pub fn file_path_matches(path: &str) -> bool {
        return crate::application::FlatpakApplication::file_extension_matches(path);
    }

    pub fn load_from_file(path: String) -> Result<Vec<FlatpakSourceDescription>, String> {
        let file_path = path::Path::new(&path);
        if !file_path.is_file() {
            return Err(format!("{} is not a file.", path));
        }

        if FlatpakSourceDescription::file_path_matches(&file_path.to_str().unwrap()) {
            let source_content = match fs::read_to_string(file_path) {
                Ok(content) => content,
                Err(e) => {
                    return Err(format!(
                        "Could not read file {}: {}!",
                        file_path.to_str().unwrap(),
                        e
                    ))
                }
            };

            // A standalone source manifest can contain a single source, or an array
            // of sources!!
            if let Ok(source) = FlatpakSourceDescription::parse(&path, &source_content) {
                return Ok(vec![source]);
            }
            if let Ok(sources) = FlatpakSourceDescription::parse_many(&path, &source_content) {
                return Ok(sources);
            }

            return Err(format!("Failed to parse Flatpak source at {}.", path));
        } else {
            return Err(format!("{} is not a Flatpak module.", path));
        }
    }

    pub fn parse(source_path: &str, source_content: &str) -> Result<FlatpakSourceDescription, String> {
        let mut flatpak_source: FlatpakSourceDescription = FlatpakSourceDescription::default();

        if source_path.to_lowercase().ends_with("yaml") || source_path.to_lowercase().ends_with("yml") {
            flatpak_source = match serde_yaml::from_str(&source_content) {
                Ok(m) => m,
                Err(e) => {
                    return Err(format!("Failed to parse the Flatpak manifest: {}.", e));
                }
            };
        } else if source_path.to_lowercase().ends_with("json") {
            let json_content_without_comments = crate::utils::remove_comments_from_json(source_content);
            flatpak_source = match serde_json::from_str(&json_content_without_comments) {
                Ok(m) => m,
                Err(e) => {
                    return Err(format!("Failed to parse the Flatpak manifest: {}.", e));
                }
            };
        }

        if let Err(e) = flatpak_source.is_valid() {
            return Err(e);
        }
        Ok(flatpak_source)
    }

    pub fn parse_many(
        source_path: &str,
        source_content: &str,
    ) -> Result<Vec<FlatpakSourceDescription>, String> {
        let mut flatpak_sources: Vec<FlatpakSourceDescription> = vec![];

        if source_path.to_lowercase().ends_with("yaml") || source_path.to_lowercase().ends_with("yml") {
            flatpak_sources = match serde_yaml::from_str(&source_content) {
                Ok(m) => m,
                Err(e) => {
                    return Err(format!("Failed to parse the Flatpak manifest: {}.", e));
                }
            };
        } else if source_path.to_lowercase().ends_with("json") {
            let json_content_without_comments = crate::utils::remove_comments_from_json(source_content);
            flatpak_sources = match serde_json::from_str(&json_content_without_comments) {
                Ok(m) => m,
                Err(e) => {
                    return Err(format!("Failed to parse the Flatpak manifest: {}.", e));
                }
            };
        }

        for flatpak_source in &flatpak_sources {
            if let Err(e) = flatpak_source.is_valid() {
                return Err(e);
            }
        }
        Ok(flatpak_sources)
    }

    pub fn is_valid(&self) -> Result<(), String> {
        if self.r#type.is_none() {
            return Err("Required top-level field type is missing from Flatpak source.".to_string());
        }
        if let Some(source_type) = &self.r#type {
            if !SOURCE_TYPES.contains(&source_type) {
                return Err(format!("Invalid source type {}.", source_type));
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(Debug)]
#[derive(Default)]
#[derive(Hash)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
/// See <https://github.com/flathub/flatpak-external-data-checker#changes-to-flatpak-manifests>
/// for the specification
pub struct FlatpakDataCheckerConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_pattern: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_main_source: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse_single_source() {
        match FlatpakSourceDescription::parse(
            "source.yaml",
            r###"
            type: file
            path: apply_extra.sh
            "###,
        ) {
            Err(e) => panic!(e),
            Ok(source) => {
                assert_eq!(source.path, Some("apply_extra.sh".to_string()));
            }
        }
    }

    #[test]
    pub fn test_parse_multiple_sources() {
        match FlatpakSourceDescription::parse_many(
            "source.yaml",
            r###"
            - type: file
              path: apply_extra.sh

            - type: file
              path: com.wps.Office.metainfo.xml

            - type: file
              path: wps.sh

            - type: extra-data
              filename: wps-office.deb
              only-arches:
                - x86_64
              url: https://wdl1.pcfg.cache.wpscdn.com/wps-office_11.1.0.10702.XA_amd64.deb
              sha256: 390a8b358aaccdfda54740d10d5306c2543c5cd42a7a8fd5c776ccff38492992
              size: 275210770
              installed-size: 988671247
              x-checker-data:
                type: html
                url: https://linux.wps.com/js/meta.js
                version-pattern: version\s*=\s*"([\d.-]+)"
                url-pattern: download_link_deb\s*=\s*"(http[s]?://[\w\d$-_@.&+]+)"
            "###,
        ) {
            Err(e) => panic!(e),
            Ok(sources) => {
                assert_eq!(sources.len(), 4);
                let last_source = sources.last().unwrap();
                assert_eq!(last_source.filename, Some("wps-office.deb".to_string()));
            }
        }
    }
}
