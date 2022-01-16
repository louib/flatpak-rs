use lazy_static::lazy_static;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub const AUTOTOOLS: &str = "autotools";
pub const CMAKE: &str = "cmake";
pub const CMAKE_NINJA: &str = "cmake-ninja";
pub const MESON: &str = "meson";
pub const QMAKE: &str = "qmake";
pub const SIMPLE: &str = "simple";

#[derive(Clone)]
#[derive(Deserialize)]
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

pub fn serialize_to_string<S>(x: &FlatpakBuildSystem, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&x.to_string())
}

pub fn deserialize_from_string<'de, D>(deserializer: D) -> Result<FlatpakBuildSystem, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;

    FlatpakBuildSystem::from_string(&buf).map_err(serde::de::Error::custom)
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
