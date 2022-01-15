use std::collections::BTreeMap;
use std::fs;
use std::path;

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::source::FlatpakSource;

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(Debug)]
pub enum FlatpakManifestFormat {
    JSON,
    YAML,
}
impl Default for FlatpakManifestFormat {
    fn default() -> Self {
        FlatpakManifestFormat::YAML
    }
}

lazy_static! {
    static ref REVERSE_DNS_FILENAME_REGEX: Regex = Regex::new(
        r"[a-z][a-z][a-z]*\.[a-z][0-9a-zA-Z_\-]+\.[a-z][0-9a-zA-Z_\-]+(\.[a-z][0-9a-zA-Z_\-]+)*\.(json|yaml|yml)$"
    ).unwrap();
}

lazy_static! {
    pub static ref KNOWN_GENERATED_SOURCES_PATHS: Vec<String> = vec![
        "cargo-sources.json".to_string(),
        "generated-sources.json".to_string(),
        // For the go generator, the default is <module-name>-sources.json
        "-sources.json".to_string(),
        "python2-modules.json".to_string(),
        "python3-modules.json".to_string(),
        "python2-requirements.json".to_string(),
        "python3-requirements.json".to_string(),
        "pypi-dependencies.json".to_string(),
        "generated-poetry-sources.json".to_string(),
        "rubygems.json".to_string(),
    ];
}

/// Main structure for a Flatpak application manifest.
/// See `man flatpak-manifest` for the flatpak manifest specs.
#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(Debug)]
#[derive(Default)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct FlatpakManifest {
    #[serde(skip_serializing)]
    pub format: FlatpakManifestFormat,

    /// Name of the application.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub app_name: String,

    /// A string defining the application id.
    /// Both names (app-id and id) are accepted.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub app_id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub id: String,

    /// The branch to use when exporting the application.
    /// If this is unset the defaults come from the default-branch option.
    ///
    /// This key overrides both the default-branch key, and the --default-branch commandline option.
    /// Unless you need a very specific branchname (like for a runtime or an extension) it is recommended
    /// to use the default-branch key instead, because you can then override the default using
    /// --default-branch when building for instance a test build.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub branch: String,

    /// The default branch to use when exporting the application. Defaults to master.
    /// This key can be overridden by the --default-branch commandline option.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub default_branch: String,

    /// The collection ID of the repository, defaults to being unset.
    /// Setting a globally unique collection ID allows the apps in the
    /// repository to be shared over peer to peer systems without needing further configuration.
    /// If building in an existing repository, the collection ID must match the existing
    /// configured collection ID for that repository.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub collection_id: String,

    /// The name of the runtime that the application uses.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub runtime: String,

    /// The version of the runtime that the application uses, defaults to master.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub runtime_version: String,

    /// The name of the development runtime that the application builds with.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub sdk: String,

    /// The name of the development extensions that the application requires to build.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sdk_extensions: Vec<String>,

    /// Initialize the (otherwise empty) writable /var in the build with a copy of this runtime.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub var: String,

    /// Use this file as the base metadata file when finishing.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub metadata: String,

    /// Build a new runtime instead of an application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_runtime: Option<bool>,

    /// Whether the manifest describes an extension to be used by other manifests.
    /// Extensions can be used to bundle programming langages and their associated
    /// tools, for example.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_extension: Option<bool>,

    /// Start with the files from the specified application.
    /// This can be used to create applications that extend another application.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub base: String,

    /// Use this specific version of the application specified in base.
    /// If unspecified, this uses the value specified in branch
    #[serde(skip_serializing_if = "String::is_empty")]
    pub base_version: String,

    /// Install these extra extensions from the base application when
    /// initializing the application directory.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub base_extensions: Vec<String>,

    /// Inherit these extra extensions points from the base application or
    /// sdk when finishing the build.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub inherit_extensions: Vec<String>,

    /// Inherit these extra extensions points from the base application or sdk
    /// when finishing the build, but do not inherit them into the platform.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub inherit_sdk_extensions: Vec<String>,

    /// Inherit these extra extensions points from the base application or sdk when finishing the build,
    /// but do not inherit them into the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_options: Option<FlatpakBuildOptions>,

    /// The name of the command that the flatpak should run on execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,

    /// Add these tags to the metadata file.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,

    /// This is a dictionary of extension objects.
    /// The key is the name of the extension.
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub add_extensions: BTreeMap<String, FlatpakExtension>,

    /// This is a dictionary of extension objects similar to add-extensions.
    /// The main difference is that the extensions are added early and are
    /// available for use during the build.
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub add_build_extensions: BTreeMap<String, FlatpakExtension>,

    /// An array of file patterns that should be removed at the end.
    /// Patterns starting with / are taken to be full pathnames (without the /app prefix),
    /// otherwise they just match the basename.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cleanup: Vec<String>,

    /// An array of commandlines that are run during the cleanup phase.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cleanup_commands: Vec<String>,

    /// Extra files to clean up in the platform.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cleanup_platform: Vec<String>,

    /// An array of commandlines that are run during the cleanup phase of the platform.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cleanup_platform_commands: Vec<String>,

    /// An array of commandlines that are run after importing the base platform,
    /// but before applying the new files from the sdk. This is a good place to e.g. delete
    /// things from the base that may conflict with the files added in the sdk.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub prepare_platform_commands: Vec<String>,

    /// An array of arguments passed to the flatpak build-finish command.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub finish_args: Vec<String>,

    /// Any desktop file with this name will be renamed to a name
    /// based on id during the cleanup phase.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub rename_desktop_file: String,

    /// Any appdata file with this name will be renamed to a name based
    /// on id during the cleanup phase.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub rename_appdata_file: String,

    /// Any icon with this name will be renamed to a name based on id during
    /// the cleanup phase. Note that this is the icon name, not the full filenames,
    /// so it should not include a filename extension.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub rename_icon: String,

    /// Replace the appdata project-license field with this string.
    /// This is useful as the upstream license is typically only about
    /// the application itself, whereas the bundled app can contain other
    /// licenses too.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub appdata_license: String,

    /// If rename-icon is set, keep a copy of the old icon file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_icon: Option<bool>,

    /// This string will be prefixed to the Name key in the main application desktop file.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub desktop_file_name_prefix: String,

    /// This string will be suffixed to the Name key in the main application desktop file.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub desktop_file_name_suffix: String,

    /// An array of strings specifying the modules to be built in order.
    /// String members in the array are interpreted as the name of a separate
    /// json or yaml file that contains a module. See below for details.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub modules: Vec<FlatpakModule>,
}
impl FlatpakManifest {
    pub fn get_id(&self) -> String {
        if !self.app_id.is_empty() {
            return self.app_id.to_string();
        }
        return self.id.to_string();
    }

    pub fn load_from_file(path: String) -> Result<FlatpakManifest, String> {
        let file_path = path::Path::new(&path);
        if !file_path.is_file() {
            return Err(format!("{} is not a file.", path));
        }

        if FlatpakManifest::file_path_matches(&file_path.to_str().unwrap()) {
            let manifest_content = match fs::read_to_string(file_path) {
                Ok(content) => content,
                Err(e) => {
                    return Err(format!(
                        "Could not read file {}: {}!",
                        file_path.to_str().unwrap(),
                        e
                    ))
                }
            };

            let manifest = match FlatpakManifest::parse(&path, &manifest_content) {
                Ok(m) => m,
                Err(e) => return Err(format!("Failed to parse Flatpak manifest at {}: {}", path, e)),
            };

            return Ok(manifest);
        } else {
            return Err(format!("{} is not a Flatpak manifest.", path));
        }
    }

    pub fn file_extension_matches(path: &str) -> bool {
        if path.to_lowercase().ends_with("yml") || path.to_lowercase().ends_with("yaml") {
            return true;
        }
        if path.to_lowercase().ends_with("json") {
            return true;
        }
        return false;
    }

    pub fn file_path_matches(path: &str) -> bool {
        REVERSE_DNS_FILENAME_REGEX.is_match(&path.to_lowercase())
    }

    pub fn parse(manifest_path: &str, manifest_content: &str) -> Result<FlatpakManifest, String> {
        let mut flatpak_manifest: FlatpakManifest = FlatpakManifest::default();

        if manifest_path.to_lowercase().ends_with("yaml") || manifest_path.to_lowercase().ends_with("yml") {
            flatpak_manifest = match serde_yaml::from_str(&manifest_content) {
                Ok(m) => m,
                Err(e) => {
                    return Err(format!("Failed to parse the Flatpak manifest: {}.", e));
                }
            };
            flatpak_manifest.format = FlatpakManifestFormat::YAML;
        } else if manifest_path.to_lowercase().ends_with("json") {
            let json_content_without_comments = crate::utils::remove_comments_from_json(manifest_content);
            flatpak_manifest = match serde_json::from_str(&json_content_without_comments) {
                Ok(m) => m,
                Err(e) => {
                    return Err(format!("Failed to parse the Flatpak manifest: {}.", e));
                }
            };
            flatpak_manifest.format = FlatpakManifestFormat::JSON;
        }

        // From https://docs.flatpak.org/en/latest/manifests.html#basic-properties:
        // Each manifest file should specify basic information about the application that is to be built,
        // including the app-id, runtime, runtime-version, sdk and command parameters.
        // That being said, the `command` field is not required by manifests that declare an
        // extension to be build, using the `build-extension` field.
        if flatpak_manifest.app_id.is_empty() && flatpak_manifest.id.is_empty() {
            return Err(
                "Required top-level field id (or app-id) is missing from Flatpak manifest.".to_string(),
            );
        }
        if flatpak_manifest.runtime.is_empty() {
            return Err("Required top-level field runtime is missing from Flatpak manifest.".to_string());
        }
        if flatpak_manifest.runtime_version.is_empty() {
            return Err(
                "Required top-level field runtime-version is missing from Flatpak manifest.".to_string(),
            );
        }
        if flatpak_manifest.sdk.is_empty() {
            return Err("Required top-level field sdk is missing from Flatpak manifest.".to_string());
        }

        Ok(flatpak_manifest)
    }

    pub fn dump(&self) -> Result<String, String> {
        if let FlatpakManifestFormat::JSON = self.format {
            return match serde_json::to_string_pretty(&self) {
                Ok(d) => Ok(d),
                Err(e) => return Err(format!("Failed to dump the Flatpak manifest: {}.", e)),
            };
        }

        if let FlatpakManifestFormat::YAML = self.format {
            return match serde_yaml::to_string(&self) {
                Ok(d) => Ok(d),
                Err(e) => return Err(format!("Failed to dump the Flatpak manifest: {}.", e)),
            };
        }

        Err(format!("Invalid format for Flatpak manifest."))
    }

    pub fn get_all_module_urls(&self) -> Vec<String> {
        let mut all_urls = vec![];
        for module in &self.modules {
            let module: &FlatpakModuleDescription = match module {
                FlatpakModule::Path(_) => continue,
                FlatpakModule::Description(m) => &m,
            };
            all_urls.append(&mut module.get_all_urls());
        }
        all_urls
    }

    pub fn get_main_module_url(&self) -> Option<String> {
        let main_module = match self.modules.last() {
            Some(m) => m,
            None => return None,
        };
        let main_module: &FlatpakModuleDescription = match main_module {
            FlatpakModule::Path(_) => return None,
            FlatpakModule::Description(m) => m,
        };
        return main_module.get_main_url();
    }

    pub fn get_max_depth(&self) -> i32 {
        let mut max_depth: i32 = 1;
        for module in &self.modules {
            if let FlatpakModule::Description(module_description) = module {
                let module_depth = module_description.get_max_depth();
                if module_depth > max_depth {
                    max_depth = module_depth;
                }
            }
        }
        return max_depth;
    }

    pub fn is_extension(&self) -> bool {
        if let Some(e) = self.build_extension {
            return e;
        }
        false
    }

    pub fn get_all_modules_recursively(&self) -> Vec<&FlatpakModule> {
        let mut all_modules: Vec<&FlatpakModule> = vec![];
        for module in &self.modules {
            all_modules.push(module);

            let module = match module {
                FlatpakModule::Description(d) => d,
                FlatpakModule::Path(_) => continue,
            };
            for child_module in module.get_all_modules_recursively() {
                all_modules.push(child_module);
            }
        }
        all_modules
    }
}

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(Debug)]
#[derive(Hash)]
#[serde(rename_all = "kebab-case")]
#[serde(untagged)]
/// Each module item can be either a path to a module description file,
/// or an inline module description.
pub enum FlatpakModule {
    Path(String),
    Description(FlatpakModuleDescription),
}
impl FlatpakModule {
    pub fn get_all_repos_urls(&self) -> Vec<String> {
        if let FlatpakModule::Description(module_description) = self {
            return module_description.get_all_urls();
        } else {
            return vec![];
        }
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
/// Each module specifies a source that has to be separately built and installed.
/// It contains the build options and a list of sources to download and extract before
/// building.
///
/// Modules can be nested, in order to turn related modules on and off with a single key.
pub struct FlatpakModuleDescription {
    /// The name of the module, used in e.g. build logs. The name is also
    /// used for constructing filenames and commandline arguments,
    /// therefore using spaces or '/' in this string is a bad idea.
    pub name: String,

    /// If true, skip this module
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,

    /// An array of objects defining sources that will be downloaded and extracted in order.
    /// String members in the array are interpreted as the name of a separate
    /// json or yaml file that contains sources. See below for details.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<FlatpakSource>,

    /// An array of options that will be passed to configure
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub config_opts: Vec<String>,

    /// An array of arguments that will be passed to make
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub make_args: Vec<String>,

    /// An array of arguments that will be passed to make install
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub make_install_args: Vec<String>,

    /// If true, remove the configure script before starting build
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rm_configure: Option<bool>,

    /// Ignore the existence of an autogen script
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_autogen: Option<bool>,

    /// Don't call make with arguments to build in parallel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_parallel_make: Option<bool>,

    /// Name of the rule passed to make for the install phase, default is install
    #[serde(skip_serializing_if = "String::is_empty")]
    pub install_rule: String,

    /// Don't run the make install (or equivalent) stage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_make_install: Option<bool>,

    /// Don't fix up the python (*.pyo or *.pyc) header timestamps for ostree use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_python_timestamp_fix: Option<bool>,

    /// Use cmake instead of configure (deprecated: use buildsystem instead)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmake: Option<bool>,

    /// Build system to use.
    /// See [`static@crate::build_systems::FLATPAK_BUILD_SYSTEMS`] for available values.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub buildsystem: String,

    /// Use a build directory that is separate from the source directory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builddir: Option<bool>,

    /// Build inside this subdirectory of the extracted sources
    #[serde(skip_serializing_if = "String::is_empty")]
    pub subdir: String,

    /// A build options object that can override global options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_options: Option<FlatpakBuildOptions>,

    /// An array of commands to run during build (between make and make install if those are used).
    /// This is primarily useful when using the "simple" buildsystem.
    /// Each command is run in /bin/sh -c, so it can use standard POSIX shell syntax such as piping output.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub build_commands: Vec<String>,

    /// An array of shell commands that are run after the install phase.
    /// Can for example clean up the install dir, or install extra files.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub post_install: Vec<String>,

    /// An array of file patterns that should be removed at the end.
    /// Patterns starting with / are taken to be full pathnames (without the /app prefix), otherwise
    /// they just match the basename. Note that any patterns will only match
    /// files installed by this module.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cleanup: Vec<String>,

    /// The way the builder works is that files in the install directory are hard-links to the cached files,
    /// so you're not allowed to modify them in-place. If you list a file in this then the hardlink
    /// will be broken and you can modify it. This is a workaround, ideally installing files should
    /// replace files, not modify existing ones.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ensure_writable: Vec<String>,

    /// If non-empty, only build the module on the arches listed.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub only_arches: Vec<String>,

    /// Don't build on any of the arches listed.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub skip_arches: Vec<String>,

    /// Extra files to clean up in the platform.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cleanup_platform: Vec<String>,

    /// If true this will run the tests after installing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_tests: Option<bool>,

    /// The target to build when running the tests. Defaults to "check" for make and "test" for ninja.
    /// Set to empty to disable.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub test_rule: String,

    /// Array of commands to run during the tests.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub test_commands: Vec<String>,

    /// An array of objects specifying nested modules to be built before this one.
    /// String members in the array are interpreted as names of a separate json or
    /// yaml file that contains a module.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub modules: Vec<FlatpakModule>,
}
impl FlatpakModuleDescription {
    pub fn uses_external_data_checker(&self) -> bool {
        for source in &self.sources {
            let source_description = match source {
                FlatpakSource::Description(d) => d,
                FlatpakSource::Path(_) => continue,
            };
            if source_description.x_checker_data.is_some() {
                return true;
            }
        }
        return false;
    }

    pub fn get_all_mirror_urls(&self) -> Vec<String> {
        let mut all_urls = vec![];
        for module in &self.modules {
            if let FlatpakModule::Description(module_description) = module {
                all_urls.append(&mut module_description.get_all_mirror_urls());
            }
        }
        for source in &self.sources {
            for url in source.get_all_mirror_urls() {
                all_urls.push(url.to_string());
            }
        }
        all_urls
    }

    pub fn get_buildsystem(&self) -> Option<String> {
        if !self.buildsystem.is_empty() {
            return Some(self.buildsystem.to_string());
        }
        if self.cmake.unwrap_or(false) {
            return Some(crate::build_systems::CMAKE.to_string());
        }
        return None;
    }

    pub fn is_patched(&self) -> bool {
        for source in &self.sources {
            if let FlatpakSource::Description(sd) = source {
                if let Some(t) = &sd.r#type {
                    if t == "patch" {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn load_from_file(path: String) -> Result<FlatpakModuleDescription, String> {
        let file_path = path::Path::new(&path);
        if !file_path.is_file() {
            return Err(format!("{} is not a file.", path));
        }

        if FlatpakModuleDescription::file_path_matches(&file_path.to_str().unwrap()) {
            let module_content = match fs::read_to_string(file_path) {
                Ok(content) => content,
                Err(e) => {
                    return Err(format!(
                        "Could not read file {}: {}!",
                        file_path.to_str().unwrap(),
                        e
                    ));
                }
            };
            let module = match FlatpakModuleDescription::parse(&path, &module_content) {
                Ok(m) => m,
                Err(e) => return Err(format!("Failed to parse Flatpak module at {}: {}", path, e)),
            };
            return Ok(module);
        } else {
            return Err(format!("{} is not a Flatpak module.", path));
        }
    }

    pub fn parse(module_path: &str, module_content: &str) -> Result<FlatpakModuleDescription, String> {
        let mut flatpak_module: FlatpakModuleDescription = FlatpakModuleDescription::default();

        if module_path.to_lowercase().ends_with("yaml") || module_path.to_lowercase().ends_with("yml") {
            flatpak_module = match serde_yaml::from_str(&module_content) {
                Ok(m) => m,
                Err(e) => {
                    return Err(format!("Failed to parse the Flatpak manifest: {}.", e));
                }
            };
        } else if module_path.to_lowercase().ends_with("json") {
            let json_content_without_comments = crate::utils::remove_comments_from_json(module_content);
            flatpak_module = match serde_json::from_str(&json_content_without_comments) {
                Ok(m) => m,
                Err(e) => {
                    return Err(format!("Failed to parse the Flatpak manifest: {}.", e));
                }
            };
        }

        if flatpak_module.name.is_empty() {
            return Err("Required top-level field name is missing from Flatpak module.".to_string());
        }
        if flatpak_module.sources.is_empty() {
            return Err("Required sources were not found in Flatpak module.".to_string());
        }
        for source in &flatpak_module.sources {
            let source_path = match source {
                FlatpakSource::Description(_) => continue,
                FlatpakSource::Path(p) => p,
            };
            // The string elements of the source array should only be FS paths, not
            // URLs or anything else.
            if source_path.starts_with("http://") || source_path.starts_with("https://") {
                return Err("Sources provided as strings cannot be URLs!".to_string());
            }
        }

        Ok(flatpak_module)
    }

    pub fn dump(&self) -> Result<String, String> {
        return match serde_yaml::to_string(&self) {
            Ok(d) => Ok(d),
            Err(e) => Err(format!("Failed to dump the Flatpak manifest: {}.", e)),
        };
    }

    pub fn file_path_matches(path: &str) -> bool {
        return FlatpakManifest::file_extension_matches(path);
    }

    pub fn get_all_urls(&self) -> Vec<String> {
        let mut all_urls = vec![];
        for module in &self.modules {
            if let FlatpakModule::Description(module_description) = module {
                all_urls.append(&mut module_description.get_all_urls());
            }
        }
        for source in &self.sources {
            for url in source.get_all_urls() {
                all_urls.push(url);
            }
        }
        all_urls
    }

    pub fn get_all_archive_urls(&self) -> Vec<String> {
        let mut all_archive_urls = vec![];
        for source in &self.sources {
            if source.get_type_name() != "archive" {
                continue;
            }

            let source_description = match &source {
                FlatpakSource::Path(_) => continue,
                FlatpakSource::Description(d) => d,
            };

            let archive_url = match &source_description.url {
                Some(u) => u,
                None => continue,
            };
            all_archive_urls.push(archive_url.to_string());
        }
        all_archive_urls
    }

    pub fn get_all_git_urls(&self) -> Vec<String> {
        let mut all_git_urls = vec![];
        for source in &self.sources {
            if source.get_type_name() != "git" {
                continue;
            }

            let source_description = match &source {
                FlatpakSource::Path(_) => continue,
                FlatpakSource::Description(d) => d,
            };

            let git_url = match &source_description.url {
                Some(u) => u,
                None => continue,
            };

            all_git_urls.push(git_url.to_string());
        }
        all_git_urls
    }

    pub fn get_max_depth(&self) -> i32 {
        let mut max_depth: i32 = 0;
        for module in &self.modules {
            if let FlatpakModule::Description(module_description) = module {
                let module_depth = module_description.get_max_depth();
                if module_depth > max_depth {
                    max_depth = module_depth;
                }
            }
        }
        return max_depth + 1;
    }

    pub fn get_main_url(&self) -> Option<String> {
        if self.sources.len() < 1 {
            return None;
        }

        // Here we assume that the first source is the actual project, and
        // anything after is a patch or an additional file.
        let main_module_source = self.sources.first().unwrap();

        let main_module_source_url: Option<String> = main_module_source.get_url();

        match &main_module_source_url {
            Some(s) => Some(s.to_string()),
            None => None,
        }
    }

    pub fn get_all_modules_recursively(&self) -> Vec<&FlatpakModule> {
        let mut all_modules: Vec<&FlatpakModule> = vec![];
        let mut next_modules: Vec<&FlatpakModule> = vec![];
        for module in &self.modules {
            next_modules.push(module);
        }
        while !next_modules.is_empty() {
            let module = next_modules.pop().unwrap();
            all_modules.push(module);

            let module = match module {
                FlatpakModule::Description(d) => d,
                FlatpakModule::Path(_) => continue,
            };
            for next_module in &module.modules {
                next_modules.push(next_module);
            }
        }
        all_modules
    }
}

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(Debug)]
#[derive(Default)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
/// Extension define extension points in the app/runtime that can be implemented by extensions,
/// supplying extra files which are available during runtime.
///
/// Additionally the standard flatpak extension properties are supported, and put
/// directly into the metadata file: autodelete, no-autodownload, subdirectories,
/// add-ld-path, download-if, enable-if, merge-dirs, subdirectory-suffix, locale-subset,
/// version, versions. See the flatpak metadata documentation for more information on these.
pub struct FlatpakExtension {
    /// The directory where the extension is mounted. If the extension point is for an application,
    /// this path is relative to /app, otherwise it is relative to /usr.
    pub extension_directory: String,

    /// If this is true, then the data created in the extension directory is omitted from the result,
    /// and instead packaged in a separate extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<bool>,

    /// If this is true, the extension is removed during when finishing.
    /// This is only interesting for extensions in the add-build-extensions property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_after_build: Option<bool>,

    /// Whether to automatically delete extensions matching this extension point
    /// when deleting a 'related' application or runtime.
    pub autodelete: Option<bool>,

    /// Whether to automatically download extensions matching this extension point
    /// when updating or installing a 'related' application or runtime.
    pub no_autodownload: Option<bool>,

    /// If this key is set to true, then flatpak will look for extensions whose name is a
    /// prefix of the extension point name, and mount them at the corresponding
    /// name below the subdirectory.
    pub subdirectories: Option<bool>,

    /// A path relative to the extension point directory that will be appended to LD_LIBRARY_PATH.
    pub add_ld_path: Option<String>,

    /// A list of conditions, separated by semi-colons, that must be true for the extension
    /// to be auto-downloaded.
    /// These are the supported conditions:
    ///  active-gl-driver
    ///     Is true if the name of the active GL driver matches the extension point basename.
    ///  active-gtk-theme
    ///     Is true if the name of the current GTK theme (via org.gnome.desktop.interface GSetting)
    ///     matches the extension point basename.
    ///  have-intel-gpu
    ///     Is true if the i915 kernel module is loaded.
    ///  on-xdg-desktop-*
    ///     Is true if the suffix (case-insensitively) is in the XDG_CURRENT_DESKTOP env var.
    ///     For example on-xdg-desktop-GNOME-classic.
    pub download_if: Option<String>,

    /// A list of conditions, separated by semi-colons, that must be true for the extension to be
    /// enabled. See download_if for available conditions.
    pub enable_if: Option<String>,

    /// A list of relative paths of directories below the extension point directory that will be merged.
    pub merge_dirs: Option<String>,

    /// A suffix that gets appended to the directory name.
    /// This is very useful when the extension point naming scheme is "reversed".
    /// For example, an extension point for GTK+ themes would be /usr/share/themes/$NAME/gtk-3.0,
    /// which could be achieved using subdirectory-suffix=gtk-3.0.
    pub subdirectory_suffix: Option<String>,

    /// If set, then the extensions are partially downloaded by default, based on the currently
    /// configured locales. This means that the extension contents should be
    /// a set of directories with the language code as name.
    pub locale_subset: Option<bool>,

    /// The branch to use when looking for the extension.
    /// If this is not specified, it defaults to the branch of the application or
    /// runtime that the extension point is for.
    pub version: Option<String>,

    /// The branches to use when looking for the extension.
    /// If this is not specified, it defaults to the branch of the application or
    /// runtime that the extension point is for.
    pub versions: Option<String>,
}

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(Debug)]
#[derive(Default)]
#[derive(Hash)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
/// Build options specify the build environment of a module,
/// and can be specified globally as well as per-module.
/// Options can also be specified on a per-architecture basis using the arch property.
pub struct FlatpakBuildOptions {
    /// This is set in the environment variable CFLAGS during the build.
    /// Multiple specifications of this (in e.g. per-arch area) are concatenated, separated by spaces.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub cflags: String,

    /// If this is true, clear cflags from previous build options before adding it from these options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cflags_override: Option<bool>,

    /// This is set in the environment variable CPPFLAGS during the build.
    /// Multiple specifications of this (in e.g. per-arch area) are concatenated, separated by spaces.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub cppflags: String,

    /// If this is true, clear cppflags from previous build options before adding it from these options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cppflags_override: Option<bool>,

    /// This is set in the environment variable CXXFLAGS during the build.
    /// Multiple specifications of this (in e.g. per-arch area) are concatenated, separated by spaces.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub cxxflags: String,

    /// If this is true, clear cxxflags from previous build options before adding it from these options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cxxflags_override: Option<bool>,

    /// This is set in the environment variable LDFLAGS during the build.
    /// Multiple specifications of this (in e.g. per-arch area) are concatenated,
    /// separated by spaces.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub ldflags: String,

    /// If this is true, clear ldflags from previous build options before adding it from these options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ldflags_override: Option<bool>,

    /// The build prefix for the modules (defaults to /app for applications and /usr for runtimes).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub prefix: String,

    /// The build libdir for the modules (defaults to /app/lib for applications and /usr/lib for runtimes).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub libdir: String,

    /// This will get appended to PATH in the build environment (with an leading colon if needed).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub append_path: String,

    /// This will get prepended to PATH in the build environment (with an trailing colon if needed).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub prepend_path: String,

    /// This will get appended to LD_LIBRARY_PATH in the build environment (with an leading colon if needed).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub append_ld_library_path: String,

    /// This will get prepended to LD_LIBRARY_PATH in the build environment (with an trailing colon if needed).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub prepend_ld_library_path: String,

    /// This will get appended to PKG_CONFIG_PATH in the build environment (with an leading colon if needed).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub append_pkg_config_path: String,

    /// This will get prepended to PKG_CONFIG_PATH in the build environment (with an trailing colon if needed).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub prepend_pkg_config_path: String,

    // This is a dictionary defining environment variables to be set during the build.
    // Elements in this override the properties that set the environment, like
    // cflags and ldflags. Keys with a null value unset the corresponding variable.
    // FIXME the doc says this should be an object, but when defined in the modules,
    // it is actually an array with values like PPC_CONFIG_PATH=/app/etc.
    // pub env: BTreeMap<String, String>,
    /// This is an array containing extra options to pass to flatpak build.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub build_args: Vec<String>,

    /// Similar to build-args but affects the tests, not the normal build.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub test_args: Vec<String>,

    /// This is an array containing extra options to pass to configure.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub config_opts: Vec<String>,

    /// An array of extra arguments that will be passed to make
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub make_args: Vec<String>,

    /// An array of extra arguments that will be passed to make install
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub make_install_args: Vec<String>,

    /// If this is true (the default is false) then all ELF files will be stripped after install.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strip: Option<bool>,

    /// By default (if strip is not true) flatpak-builder extracts all debug info in ELF files to a
    /// separate files and puts this in an extension. If you want to disable this, set no-debuginfo
    /// to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_debuginfo: Option<bool>,

    /// By default when extracting debuginfo we compress the debug sections.
    /// If you want to disable this, set no-debuginfo-compression to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_debuginfo_compression: Option<bool>,

    /// This is a dictionary defining for each arch a separate build options object that override the main one.
    pub arch: BTreeMap<String, FlatpakBuildOptions>,
}

#[cfg(test)]
mod manifest_tests {
    use super::*;

    #[test]
    pub fn test_file_path_matches() {
        assert!(FlatpakManifest::file_path_matches("com.example.appName.yaml"));
        assert!(FlatpakManifest::file_path_matches("COM.EXAMPLE.APPNAME.YAML"));
        assert!(FlatpakManifest::file_path_matches(
            "io.github.user.repo.Devel.yaml"
        ));
        assert!(FlatpakManifest::file_path_matches(
            "/path/to/com.example.appName.yaml"
        ));
        assert!(FlatpakManifest::file_path_matches(
            "/path/to/com.example.appName.yml"
        ));
        assert!(FlatpakManifest::file_path_matches(
            "/path/to/com.example.department.product.yaml"
        ));
        assert!(FlatpakManifest::file_path_matches(
            "/path/to/com.example.department.name-of-product.yaml"
        ));
        assert!(FlatpakManifest::file_path_matches(
            "contrib/linux/com.dosbox_x.DOSBox-X.yaml"
        ));
        assert!(!FlatpakManifest::file_path_matches(
            "/tmp/com.github.flathub.org.freedesktop.LinuxAudio.Plugins.WolfShaper/flathub.json"
        ));
        assert!(!FlatpakManifest::file_path_matches("Firefox-62.0.3.update.json"));
        assert!(!FlatpakManifest::file_path_matches("/path/to/file.yaml"));
        assert!(!FlatpakManifest::file_path_matches("/path/to/file.json"));
        assert!(!FlatpakManifest::file_path_matches("/path/to/___432423fdsf.json"));
        assert!(!FlatpakManifest::file_path_matches("/path/to/example.com.json"));
        assert!(!FlatpakManifest::file_path_matches("/path/to/example.com.json."));
        assert!(!FlatpakManifest::file_path_matches(""));
        assert!(!FlatpakManifest::file_path_matches("/////////////"));
    }

    #[test]
    #[should_panic]
    pub fn test_parse_invalid_yaml() {
        FlatpakManifest::parse("manifest.yaml", "----------------------------").unwrap();
    }

    #[test]
    pub fn test_parse_missing_fields() {
        assert!(FlatpakManifest::parse(
            "manifest.yaml",
            r###"
            runtime: org.gnome.Platform
            runtime-version: "3.36"
            sdk: org.gnome.Sdk
            command: flatpak-review
            "###
        )
        .is_err());
    }

    #[test]
    pub fn test_parse() {
        match FlatpakManifest::parse(
            "manifest.yaml",
            r###"
            app-id: net.louib.flatpak-review
            runtime: org.gnome.Platform
            runtime-version: "3.36"
            sdk: org.gnome.Sdk
            command: flatpak-review
            tags: ["nightly"]
            modules:
              -
                name: "flatpak-review"
                buildsystem: simple
                cleanup: [ "*" ]
                config-opts: []
                sources:
                  -
                    type: git
                    url: https://github.com/louib/flatpak-review.git
                    branch: master
            "###,
        ) {
            Err(e) => panic!(e),
            Ok(manifest) => {
                assert_eq!(manifest.app_id, "net.louib.flatpak-review");
            }
        }
    }

    #[test]
    pub fn test_parse_shared_modules() {
        match FlatpakManifest::parse(
            "manifest.yaml",
            r###"
            app-id: net.louib.flatpak-review
            runtime: org.gnome.Platform
            runtime-version: "3.36"
            sdk: org.gnome.Sdk
            command: flatpak-review
            tags: ["nightly"]
            modules:
              -
                name: "flatpak-review"
                buildsystem: simple
                cleanup: [ "*" ]
                config-opts: []
                sources:
                  -
                    type: git
                    url: https://github.com/louib/flatpak-review.git
                    branch: master
              -
                "shared-modules/linux-audio/lv2.json"
            "###,
        ) {
            Err(e) => panic!(e),
            Ok(manifest) => {
                assert_eq!(manifest.app_id, "net.louib.flatpak-review");
            }
        }
    }

    #[test]
    pub fn test_parse_add_extensions() {
        match FlatpakManifest::parse(
            "manifest.yaml",
            r###"
            app-id: net.pcsx2.PCSX2
            runtime: org.freedesktop.Platform
            runtime-version: "19.08"
            sdk: org.freedesktop.Sdk
            command: PCSX2
            tags: ["nightly"]
            modules: []
            add-extensions:
                "org.freedesktop.Platform.Compat.i386":
                    directory: "lib/i386-linux-gnu"
                    version: "19.08"
                "org.freedesktop.Platform.Compat.i386.Debug":
                    directory: "lib/debug/lib/i386-linux-gnu"
                    version: "19.08"
                    no-autodownload: true
                "org.freedesktop.Platform.GL32":
                    directory: "lib/i386-linux-gnu/GL"
                    version: "1.4"
                    versions: "19.08;1.4"
                    subdirectories: true
                    no-autodownload: true
                    autodelete: false
                    add-ld-path: "lib"
                    merge-dirs: "vulkan/icd.d;glvnd/egl_vendor.d"
                    download-if: "active-gl-driver"
                    enable-if: "active-gl-driver"
            "###,
        ) {
            Err(e) => panic!(e),
            Ok(manifest) => {
                assert_eq!(manifest.app_id, "net.pcsx2.PCSX2");
                assert_eq!(manifest.add_extensions.len(), 3);
            }
        }
    }

    #[test]
    pub fn test_parse_string_source() {
        match FlatpakManifest::parse(
            "manifest.yaml",
            r###"
            app-id: net.louib.flatpak-review
            runtime: org.gnome.Platform
            runtime-version: "3.36"
            sdk: org.gnome.Sdk
            command: flatpak-review
            tags: ["nightly"]
            modules:
              -
                name: "flatpak-review"
                buildsystem: simple
                cleanup: [ "*" ]
                config-opts: []
                sources:
                  -
                    "shared-modules/linux-audio/lv2.json"
            "###,
        ) {
            Err(e) => panic!(e),
            Ok(manifest) => {
                assert_eq!(manifest.app_id, "net.louib.flatpak-review");
            }
        }
    }

    #[test]
    pub fn test_parse_source_without_type() {
        match FlatpakManifest::parse(
            "manifest.yaml",
            r###"
            app-id: net.louib.flatpak-review
            runtime: org.gnome.Platform
            runtime-version: "3.36"
            sdk: org.gnome.Sdk
            command: flatpak-review
            tags: ["nightly"]
            modules:
              -
                name: "gcc"
                buildsystem: simple
                cleanup: [ "*" ]
                config-opts: []
                sources:
                  -
                    url: "https://ftp.gnu.org/gnu/gcc/gcc-7.5.0/gcc-7.5.0.tar.xz"
                    sha256: "b81946e7f01f90528a1f7352ab08cc602b9ccc05d4e44da4bd501c5a189ee661"

            "###,
        ) {
            Err(e) => panic!(e),
            Ok(manifest) => {
                assert_eq!(manifest.app_id, "net.louib.flatpak-review");
            }
        }
    }

    #[test]
    pub fn test_parse_build_options() {
        match FlatpakManifest::parse(
            "manifest.yaml",
            r###"
            app-id: net.louib.flatpak-review
            runtime: org.gnome.Platform
            runtime-version: "3.36"
            sdk: org.gnome.Sdk
            command: flatpak-review
            tags: ["nightly"]
            modules:
              -
                name: "flatpak-review"
                buildsystem: simple
                cleanup: [ "*" ]
                build-options:
                   cflags: "-O2 -g"
                   cxxflags: "-O2 -g"
                   env:
                       V: "1"
                   arch:
                       x86_64:
                           cflags: "-O3 -g"
                config-opts: []
                sources:
                  -
                    "shared-modules/linux-audio/lv2.json"
            "###,
        ) {
            Err(e) => panic!(e),
            Ok(manifest) => {
                assert_eq!(manifest.app_id, "net.louib.flatpak-review");
            }
        }
    }

    #[test]
    pub fn test_parse_script_source() {
        match FlatpakManifest::parse(
            "manifest.yaml",
            r###"
            app-id: net.louib.flatpak-review
            runtime: org.gnome.Platform
            runtime-version: "3.36"
            sdk: org.gnome.Sdk
            command: flatpak-review
            tags: ["nightly"]
            modules:
              -
                name: "flatpak-review"
                buildsystem: simple
                cleanup: [ "*" ]
                config-opts: []
                sources:
                  -
                    url: "https://ftp.gnu.org/gnu/gcc/gcc-7.5.0/gcc-7.5.0.tar.xz"
                    sha256: "b81946e7f01f90528a1f7352ab08cc602b9ccc05d4e44da4bd501c5a189ee661"
                  -
                    type: "shell"
                    commands:
                      -
                        sed -i -e 's/\${\${NAME}_BIN}-NOTFOUND/\${NAME}_BIN-NOTFOUND/' cpp/CMakeLists.txt
            "###,
        ) {
            Err(e) => panic!(e),
            Ok(manifest) => {
                assert_eq!(manifest.app_id, "net.louib.flatpak-review");
            }
        }
    }

    #[test]
    pub fn test_parse_json() {
        match FlatpakManifest::parse(
            "manifest.json",
            r###"
            {
                "app-id": "org.gnome.SoundJuicer",
                "runtime": "org.gnome.Platform",
                "runtime-version": "master",
                "sdk": "org.gnome.Sdk",
                "command": "sound-juicer",
                "tags": [ "nightly" ],
                "desktop-file-name-suffix": " ☢️",
                "finish-args": [
                    "--talk-name=org.gtk.vfs", "--talk-name=org.gtk.vfs.*",
                    "--env=GST_PLUGIN_PATH=/app/lib/codecs/lib/gstreamer-1.0"
                ],
                "cleanup": [ "/include", "/share/bash-completion" ],
                "modules": [
                    {
                        "name": "cdparanoia",
                        "buildsystem": "simple",
                        "build-commands": [
                            "cp /usr/share/automake-*/config.{sub,guess} .",
                            "./configure --prefix=/app",
                            "make all slib",
                            "make install"
                        ],
                        "sources": [
                            {
                                "type": "archive",
                                "url": "http://downloads.xiph.org/releases/cdparanoia/cdparanoia-III-10.2.src.tgz",
                                "sha256": "005db45ef4ee017f5c32ec124f913a0546e77014266c6a1c50df902a55fe64df"
                            },
                            {
                                "type": "patch",
                                "path": "cdparanoia-use-proper-gnu-config-files.patch"
                            }
                        ]
                    },
                    {
                        "name": "gst-plugins-base",
                        "buildsystem": "meson",
                        "config-opts": [
                            "--prefix=/app",
                            "-Dauto_features=disabled",
                            "-Dcdparanoia=enabled"
                        ],
                        "cleanup": [ "*.la", "/share/gtk-doc" ],
                        "sources": [
                            {
                                "type": "git",
                                "url": "https://gitlab.freedesktop.org/gstreamer/gst-plugins-base.git",
                                "branch" : "1.16.2",
                                "commit" : "9d3581b2e6f12f0b7e790d1ebb63b90cf5b1ef4e"
                            }
                        ]
                    }
                ]
            }
            "###,
        ) {
            Err(e) => panic!(e),
            Ok(manifest) => {
                assert_eq!(manifest.app_id, "org.gnome.SoundJuicer");
            }
        }
    }

    #[test]
    pub fn test_parse_json_with_comments() {
        match FlatpakManifest::parse(
            "manifest.json",
            r###"
            {
                "app-id": "org.gnome.SoundJuicer",
                "runtime": "org.gnome.Platform",
                "runtime-version": "master",
                "sdk": "org.gnome.Sdk",
                "command": "sound-juicer",
                "tags": [ "nightly" ],
                "desktop-file-name-suffix": " ☢️",
                "finish-args": [
                    /* X11 + XShm access */
                    "--share=ipc", "--socket=fallback-x11",
                    /* Wayland access */
                    "--socket=wayland",
                    /* audio CDs */
                    "--device=all",
                    /* Needs to talk to the network */
                    "--share=network",
                    /* Play sounds */
                    "--socket=pulseaudio",
                    /* Browse user's Music directory */
                    "--filesystem=xdg-music",
                    /* Migrate DConf settings from the host */
                    "--metadata=X-DConf=migrate-path=/org/gnome/sound-juicer/",
                    /* optical media detection */
                    "--talk-name=org.gtk.vfs", "--talk-name=org.gtk.vfs.*",
                    /* Ensure cdda gstreamer plugin is picked found for audio CD's */
                    "--env=GST_PLUGIN_PATH=/app/lib/codecs/lib/gstreamer-1.0"
                ],
                "cleanup": [ "/include", "/share/bash-completion" ],
                "modules": [
                    /* gst-plugins-base needs cdparanoia to add support for cdda */
                    {
                        "name": "cdparanoia",
                        "buildsystem": "simple",
                        "build-commands": [
                            "cp /usr/share/automake-*/config.{sub,guess} .",
                            "./configure --prefix=/app",
                            "make all slib",
                            "make install"
                        ],
                        "sources": [
                            {
                                "type": "archive",
                                "url": "http://downloads.xiph.org/releases/cdparanoia/cdparanoia-III-10.2.src.tgz",
                                "sha256": "005db45ef4ee017f5c32ec124f913a0546e77014266c6a1c50df902a55fe64df"
                            },
                            {
                                "type": "patch",
                                "path": "cdparanoia-use-proper-gnu-config-files.patch"
                            }
                        ]
                    },
                    /* To play cdda */
                    {
                        "name": "gst-plugins-base",
                        "buildsystem": "meson",
                        "config-opts": [
                            "--prefix=/app",
                            "-Dauto_features=disabled",
                            "-Dcdparanoia=enabled"
                        ],
                        "cleanup": [ "*.la", "/share/gtk-doc" ],
                        "sources": [
                            {
                                "type": "git",
                                "url": "https://gitlab.freedesktop.org/gstreamer/gst-plugins-base.git",
                                "branch" : "1.16.2",
                                "commit" : "9d3581b2e6f12f0b7e790d1ebb63b90cf5b1ef4e"
                            }
                        ]
                    }
                ]
            }
            "###,
        ) {
            Err(e) => panic!(e),
            Ok(manifest) => {
                assert_eq!(manifest.app_id, "org.gnome.SoundJuicer");
            }
        }
    }

    #[test]
    pub fn test_parse_json_with_multi_line_comments() {
        match FlatpakManifest::parse(
            "manifest.json",
            r###"
            {
              "app-id": "org.gnome.Lollypop",
              "runtime": "org.gnome.Platform",
              "runtime-version": "40",
              "sdk": "org.gnome.Sdk",
              "command": "lollypop",
              "finish-args": [
                "--share=ipc",
                "--own-name=org.mpris.MediaPlayer2.Lollypop",
                "--metadata=X-DConf=migrate-path=/org/gnome/Lollypop/"
              ],
              /* FFmpeg-full and gst-plugins-ugly required for .wma support
               * Due to possible legal stubbornness in the USA, it can't be downloaded automatically
               */
              "add-extensions": {
                "org.freedesktop.Platform.ffmpeg-full": {
                  "directory": "lib/ffmpeg",
                  "version": "20.08",
                  "add-ld-path": ".",
                  "no-autodownload": true,
                  "autodelete": false
                }
              },
              "cleanup-commands": [
                "mkdir -p /app/lib/ffmpeg"
              ],
              "modules": [
                "pypi-dependencies.json",
                {
                  "name": "gst-plugins-ugly",
                  "buildsystem": "meson",
                  "cleanup": [
                    "*.la",
                    "/share/gtk-doc"
                  ],
                  "sources": [{
                    "type": "archive",
                    "url": "https://gstreamer.freedesktop.org/src/gst-plugins-ugly/gst-plugins-ugly-1.16.2.tar.xz",
                    "sha256": "5500415b865e8b62775d4742cbb9f37146a50caecfc0e7a6fc0160d3c560fbca"
                  }]
                }
              ]
            }
            "###,
        ) {
            Err(e) => panic!(e),
            Ok(manifest) => {
                assert_eq!(manifest.app_id, "org.gnome.Lollypop");
            }
        }
    }

    #[test]
    pub fn test_parse_extension() {
        match FlatpakManifest::parse(
            "manifest.json",
            r###"
            {
                "id": "org.freedesktop.Platform.Icontheme.Paper",
                "branch": "1.0",
                "runtime": "org.freedesktop.Sdk",
                "build-extension": true,
                "sdk": "org.freedesktop.Sdk",
                "runtime-version": "1.6",
                "sdk-extensions": [],
                "separate-locales": false,
                "cleanup": [ "/share/info", "/share/man" ],
                "appstream-compose": false,
                "build-options" : {
                    "prefix": "/usr/share/runtime"
                },
                "modules": []
            }
            "###,
        ) {
            Err(e) => panic!(e),
            Ok(manifest) => {
                assert_eq!(manifest.id, "org.freedesktop.Platform.Icontheme.Paper");
                assert_eq!(manifest.get_id(), "org.freedesktop.Platform.Icontheme.Paper");
            }
        }
    }

    #[test]
    pub fn test_parse_recursive_modules() {
        match FlatpakManifest::parse(
            "manifest.yaml",
            r###"
            app-id: com.georgefb.haruna
            runtime: org.kde.Platform
            runtime-version: '5.15'
            sdk: org.kde.Sdk
            command: haruna
            finish-args:
              - '--share=ipc'
              - '--share=network'
              - '--socket=x11'
              - '--socket=wayland'
              - '--socket=pulseaudio'
              - '--device=dri'
              - '--filesystem=host'
              - '--talk-name=ca.desrt.dconf'
              - '--talk-name=org.freedesktop.ScreenSaver'
              - '--own-name=org.mpris.MediaPlayer2.haruna'
              - '--env=DCONF_USER_CONFIG_DIR=.config/dconf'
              - '--env=LC_NUMERIC=C'
              - '--env=XDG_DATA_DIRS=/usr/share:/app/share/'
            modules:
              - name: haruna
                buildsystem: cmake-ninja
                sources:
                  - type: archive
                    url: 'https://github.com/g-fb/haruna/archive/refs/tags/0.6.3.tar.gz'
                    sha256: 'c79ec1e351f47faf9a58a6ba7ec3cc05cfdc5423fde0584f2d4081f5058363e3'
                modules:
                  - name: taglib
                    buildsystem: cmake-ninja
                    config-opts:
                    - '-DBUILD_SHARED_LIBS=ON'
                    sources:
                    - type: archive
                      url: 'https://github.com/taglib/taglib/releases/download/v1.12/taglib-1.12.tar.gz'
                      sha256: '7fccd07669a523b07a15bd24c8da1bbb92206cb19e9366c3692af3d79253b703'
                  - name: ffmpegthumbs
                    buildsystem: cmake-ninja
                    sources:
                    - type: archive
                      url: 'https://invent.kde.org/multimedia/ffmpegthumbs/-/archive/v20.12.3/ffmpegthumbs-v20.12.3.tar.gz'
                      sha256: '9292a503808688b37e45ee336efcd28c39035d49c96e1df466b091f2eaf7a529'
                  - name: kio-extras
                    buildsystem: cmake-ninja
                    sources:
                    - type: archive
                      url: 'https://invent.kde.org/network/kio-extras/-/archive/v20.12.3/kio-extras-v20.12.3.tar.gz'
                      sha256: 'bedccdbf3664f2669270c2864c6f7c0e73237d18fae04067bc21ae5e12716b0b'
                  - name: libmpv
                    cleanup:
                      - /include
                      - /lib/pkgconfig
                      - /share/man
                    buildsystem: simple
                    build-commands:
                      - python3 waf configure --prefix=/app --enable-libmpv-shared --disable-cplayer --disable-build-date --disable-alsa
                      - python3 waf build
                      - python3 waf install
                    sources:
                      - type: archive
                        url: 'https://github.com/mpv-player/mpv/archive/v0.33.1.tar.gz'
                        sha256: '100a116b9f23bdcda3a596e9f26be3a69f166a4f1d00910d1789b6571c46f3a9'
                      - type: file
                        url: 'https://waf.io/waf-2.0.21'
                        sha256: '7cebf2c5efe53cbb9a4b5bdc4b49ae90ecd64a8fce7a3222d58e591b58215306'
                        dest-filename: waf
                    modules:
                      - name: luajit
                        no-autogen: true
                        cleanup:
                          - /bin
                          - /include
                          - /lib/pkgconfig
                          - /share/man
                        sources:
                          - type: archive
                            url: 'http://luajit.org/download/LuaJIT-2.1.0-beta3.tar.gz'
                            sha256: '1ad2e34b111c802f9d0cdf019e986909123237a28c746b21295b63c9e785d9c3'
                          - type: shell
                            commands:
                              - sed -i 's|/usr/local|/app|' ./Makefile
                      - name: libv4l2
                        cleanup:
                          - /sbin
                          - /bin
                          - /include
                          - /lib/pkgconfig
                          - /share/man
                        config-opts:
                          - '--disable-static'
                          - '--disable-bpf'
                          - '--with-udevdir=/app/lib/udev'
                        sources:
                          - type: archive
                            url: 'https://linuxtv.org/downloads/v4l-utils/v4l-utils-1.20.0.tar.bz2'
                            sha256: '956118713f7ccb405c55c7088a6a2490c32d54300dd9a30d8d5008c28d3726f7'
                      - name: nv-codec-headers
                        cleanup:
                          - '*'
                        no-autogen: true
                        make-install-args:
                          - PREFIX=/app
                        sources:
                          - type: archive
                            url: 'https://github.com/FFmpeg/nv-codec-headers/releases/download/n11.0.10.0/nv-codec-headers-11.0.10.0.tar.gz'
                            sha256: 'e5d1fe6b18254a3c8747a38714564030e4fda506961a11a7eafb94f2400419bb'
                      - name: ffmpeg
                        cleanup:
                          - /include
                          - /lib/pkgconfig
                          - /share/ffmpeg/examples
                        config-opts:
                          - '--enable-shared'
                          - '--disable-static'
                          - '--enable-gnutls'
                          - '--enable-gpl'
                          - '--disable-doc'
                          - '--disable-programs'
                          - '--disable-encoders'
                          - '--disable-muxers'
                          - '--enable-encoder=png,libwebp'
                          - '--enable-libv4l2'
                          - '--enable-libdav1d'
                          - '--enable-libfontconfig'
                          - '--enable-libfreetype'
                          - '--enable-libopus'
                          - '--enable-librsvg'
                          - '--enable-libvpx'
                          - '--enable-libmp3lame'
                          - '--enable-libwebp'
                        sources:
                          - type: archive
                            url: 'https://ffmpeg.org/releases/ffmpeg-4.3.2.tar.xz'
                            sha256: '46e4e64f1dd0233cbc0934b9f1c0da676008cad34725113fb7f802cfa84ccddb'
                      - name: libass
                        cleanup:
                          - /include
                          - /lib/pkgconfig
                        config-opts:
                          - '--disable-static'
                        sources:
                          - type: archive
                            url: 'https://github.com/libass/libass/releases/download/0.15.0/libass-0.15.0.tar.gz'
                            sha256: '9cbddee5e8c87e43a5fe627a19cd2aa4c36552156eb4edcf6c5a30bd4934fe58'
                      - name: uchardet
                        buildsystem: cmake-ninja
                        config-opts:
                          - '-DCMAKE_BUILD_TYPE=Release'
                          - '-DBUILD_STATIC=0'
                        cleanup:
                          - /bin
                          - /include
                          - /lib/pkgconfig
                          - /share/man
                        sources:
                          - type: archive
                            url: 'https://gitlab.freedesktop.org/uchardet/uchardet/-/archive/v0.0.7/uchardet-v0.0.7.tar.gz'
                            sha256: 'f3635d1d10e1470452bc42c1bf509451a9926b399a11740a9949e86069d69f58'
                  - name: youtube-dl
                    no-autogen: true
                    no-make-install: true
                    make-args:
                      - youtube-dl
                      - PYTHON=/usr/bin/python3
                    post-install:
                      - install youtube-dl /app/bin
                    sources:
                      - type: archive
                        url: 'https://github.com/ytdl-org/youtube-dl/archive/2021.04.26.tar.gz'
                        sha256: 'd80023ab221b3cb89229b632e247035a22c5afaee9a7b3c653bbd702f71c1083'
            "###,
        ) {
            Err(e) => panic!(e),
            Ok(manifest) => {
                assert_eq!(manifest.app_id, "com.georgefb.haruna");
                assert_eq!(manifest.get_max_depth(), 3);
                assert_eq!(manifest.modules.len(), 1);
                assert_eq!(manifest.get_all_modules_recursively().len(), 12);
            }
        }
    }
}

#[cfg(test)]
mod modules_tests {
    use super::*;

    #[test]
    pub fn test_parse_extra_data() {
        match FlatpakModuleDescription::parse(
            "module.yaml",
            r###"
            name: wps
            buildsystem: simple
            build-commands:
              - install -Dm755 apply_extra.sh /app/bin/apply_extra
              - install -Dm755 wps.sh /app/bin/wps
              - ln -s wps /app/bin/et
              - ln -s wps /app/bin/wpp
              - ln -s wps /app/bin/wpspdf
              - install -Dm644 ${FLATPAK_ID}.metainfo.xml -t /app/share/metainfo/
              - install -Dm755 /usr/bin/desktop-file-edit -t /app/bin/
              - install -Dm755 /usr/bin/ar -t /app/bin/
              - install -Dm755 /usr/lib/$(gcc -print-multiarch)/libbfd-*.so -t /app/lib/
            sources:
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
            Ok(module) => {
                assert_eq!(module.name, "wps");
            }
        }
    }

    #[test]
    pub fn test_parse_helm_files() {
        assert!(FlatpakModuleDescription::parse(
            "module.yaml",
            r###"
            name: wps
            sources:
              - "https://github.com/user/repo"
            "###,
        )
        .is_err())
    }
}
