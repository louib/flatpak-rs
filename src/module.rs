use std::collections::BTreeMap;
use std::fs;
use std::path;

use serde::{Deserialize, Serialize};

use crate::build_system::FlatpakBuildSystem;
use crate::format::FlatpakManifestFormat;
use crate::source::{FlatpakSourceItem, ARCHIVE, GIT, PATCH};

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(Debug)]
#[derive(Hash)]
#[serde(untagged)]
/// Items in a module list can be either paths to external module manifests, or inline descriptions
/// of flatpak modules.
pub enum FlatpakModuleItem {
    Path(String),
    Description(FlatpakModule),
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
pub struct FlatpakModule {
    #[serde(skip_serializing)]
    pub format: FlatpakManifestFormat,

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
    pub sources: Vec<FlatpakSourceItem>,

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
    #[serde(deserialize_with = "crate::build_system::deserialize_from_string")]
    #[serde(serialize_with = "crate::build_system::serialize_to_string")]
    pub buildsystem: FlatpakBuildSystem,

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
    pub modules: Vec<FlatpakModuleItem>,
}
impl FlatpakModule {
    pub fn uses_external_data_checker(&self) -> bool {
        for source in &self.sources {
            let source_description = match source {
                FlatpakSourceItem::Description(d) => d,
                FlatpakSourceItem::Path(_) => continue,
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
            if let FlatpakModuleItem::Description(module_description) = module {
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
        if self.cmake.unwrap_or(false) {
            return Some(crate::build_system::CMAKE.to_string());
        }
        return Some(self.buildsystem.to_string());
    }

    pub fn is_patched(&self) -> bool {
        for source in &self.sources {
            if let FlatpakSourceItem::Description(sd) = source {
                if let Some(t) = &sd.r#type {
                    if t == PATCH {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn load_from_file(path: String) -> Result<FlatpakModule, String> {
        let file_path = path::Path::new(&path);
        if !file_path.is_file() {
            return Err(format!("{} is not a file.", path));
        }

        let manifest_format = match FlatpakManifestFormat::from_path(&path) {
            Some(f) => f,
            None => return Err(format!("{} is not a Flatpak module manifest.", path)),
        };

        let manifest_content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(e) => {
                return Err(format!("Could not read module manifest at {}: {}", &path, e));
            }
        };
        match FlatpakModule::parse(manifest_format, &manifest_content) {
            Ok(m) => Ok(m),
            Err(e) => Err(format!(
                "Failed to parse Flatpak module manifest at {}: {}",
                path, e
            )),
        }
    }

    pub fn parse(format: FlatpakManifestFormat, manifest_content: &str) -> Result<FlatpakModule, String> {
        let mut flatpak_module: FlatpakModule = match &format {
            FlatpakManifestFormat::YAML => match serde_yaml::from_str(&manifest_content) {
                Ok(m) => m,
                Err(e) => {
                    return Err(format!("Failed to parse the Flatpak module manifest: {}.", e));
                }
            },
            FlatpakManifestFormat::JSON => {
                let json_content_without_comments = crate::utils::remove_comments_from_json(manifest_content);
                match serde_json::from_str(&json_content_without_comments) {
                    Ok(m) => m,
                    Err(e) => {
                        return Err(format!("Failed to parse the Flatpak module manifest: {}.", e));
                    }
                }
            }
        };
        flatpak_module.format = format;

        if flatpak_module.name.is_empty() {
            return Err("Required top-level field name is missing from Flatpak module.".to_string());
        }
        if flatpak_module.sources.is_empty() {
            return Err("Required sources were not found in Flatpak module.".to_string());
        }
        for source in &flatpak_module.sources {
            let source_path = match source {
                FlatpakSourceItem::Description(_) => continue,
                FlatpakSourceItem::Path(p) => p,
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
        // The file path for a module is not necessarily in reverse DNS, so we can only test
        // for the extension of the file.
        crate::filename::extension_is_valid(path)
    }

    pub fn get_all_urls(&self) -> Vec<String> {
        let mut all_urls = vec![];
        for module in &self.modules {
            if let FlatpakModuleItem::Description(module_description) = module {
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
            if source.get_type_name() != ARCHIVE {
                continue;
            }

            let source_description = match &source {
                FlatpakSourceItem::Path(_) => continue,
                FlatpakSourceItem::Description(d) => d,
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
            if source.get_type_name() != GIT {
                continue;
            }

            let source_description = match &source {
                FlatpakSourceItem::Path(_) => continue,
                FlatpakSourceItem::Description(d) => d,
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
            if let FlatpakModuleItem::Description(module_description) = module {
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

    pub fn get_all_modules_recursively(&self) -> Vec<&FlatpakModuleItem> {
        let mut all_modules: Vec<&FlatpakModuleItem> = vec![];
        let mut next_modules: Vec<&FlatpakModuleItem> = vec![];
        for module in &self.modules {
            next_modules.push(module);
        }
        while !next_modules.is_empty() {
            let module = next_modules.pop().unwrap();
            all_modules.push(module);

            let module = match module {
                FlatpakModuleItem::Description(d) => d,
                FlatpakModuleItem::Path(_) => continue,
            };
            for next_module in &module.modules {
                next_modules.push(next_module);
            }
        }
        all_modules
    }

    /// A module is composite if it links to multiple software projects.
    /// This is determined by the type of the sources contained in the module.
    pub fn is_composite(&self) -> bool {
        let mut code_sources_count = 0;
        for source in &self.sources {
            let source_type_name = source.get_type_name();
            if crate::source::CODE_TYPES.contains(&source_type_name) {
                code_sources_count += 1;
            }
            if code_sources_count > 1 {
                return true;
            }
        }
        false
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
mod tests {
    use super::*;

    #[test]
    pub fn test_parse_extra_data() {
        match FlatpakModule::parse(
            FlatpakManifestFormat::YAML,
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
            Err(e) => std::panic::panic_any(e),
            Ok(module) => {
                assert_eq!(module.name, "wps");
            }
        }
    }

    #[test]
    pub fn test_parse_helm_files() {
        assert!(FlatpakModule::parse(
            FlatpakManifestFormat::YAML,
            r###"
            name: wps
            sources:
              - "https://github.com/user/repo"
            "###,
        )
        .is_err())
    }
}
