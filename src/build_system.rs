use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

pub const AUTOTOOLS: &str = "autotools";
pub const CMAKE: &str = "cmake";
pub const CMAKE_NINJA: &str = "cmake-ninja";
pub const MESON: &str = "meson";
pub const QMAKE: &str = "qmake";
pub const SIMPLE: &str = "simple";

lazy_static! {
    /// List of all build systems available natively with Flatpak.
    /// THIS WILL BE DEPRECATED IN FAVOR OF THE ENUM.
    pub static ref FLATPAK_BUILD_SYSTEMS: Vec<String> = vec![
        AUTOTOOLS.to_string(),
        CMAKE.to_string(),
        CMAKE_NINJA.to_string(),
        MESON.to_string(),
        QMAKE.to_string(),
        SIMPLE.to_string(),
    ];
}

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(Debug)]
#[derive(Hash)]
pub enum FlatpakBuildSystem {
    Autotools,
    CMake,
    CMakeNinja,
    QMake,
    Meson,
    Simple,
}
impl Default for FlatpakBuildSystem {
    fn default() -> Self {
        FlatpakBuildSystem::Simple
    }
}
impl FlatpakBuildSystem {
    pub fn to_string(&self) -> String {
        match &self {
            Autotools => AUTOTOOLS.to_string(),
            CMake => CMAKE.to_string(),
            CMakeNinja => CMAKE_NINJA.to_string(),
            QMake => QMAKE.to_string(),
            Meson => MESON.to_string(),
            Simple => SIMPLE.to_string(),
        }
    }
    pub fn from_string(build_sys: &str) -> Result<FlatpakBuildSystem, String> {
        if build_sys == AUTOTOOLS {
            return Ok(FlatpakBuildSystem::Autotools);
        }
        if build_sys == CMAKE {
            return Ok(FlatpakBuildSystem::CMake);
        }
        if build_sys == CMAKE_NINJA {
            return Ok(FlatpakBuildSystem::CMakeNinja);
        }
        if build_sys == QMAKE {
            return Ok(FlatpakBuildSystem::QMake);
        }
        if build_sys == MESON {
            return Ok(FlatpakBuildSystem::Meson);
        }
        if build_sys == SIMPLE {
            return Ok(FlatpakBuildSystem::Simple);
        }
        Err(format!("Invalid build system {}.", build_sys))
    }
}
