use serde::{Deserialize, Deserializer, Serializer};

pub const AUTOTOOLS: &str = "autotools";
pub const CMAKE: &str = "cmake";
pub const CMAKE_NINJA: &str = "cmake-ninja";
pub const MESON: &str = "meson";
pub const QMAKE: &str = "qmake";
pub const SIMPLE: &str = "simple";
#[cfg(feature = "extended_build_sytems")]
pub const CABAL: &str = "cabal";
#[cfg(feature = "extended_build_sytems")]
pub const MAKE: &str = "make";
#[cfg(feature = "extended_build_sytems")]
pub const CARGO: &str = "cargo";

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Hash)]
#[derive(PartialEq)]
pub enum FlatpakBuildSystem {
    Autotools,
    CMake,
    CMakeNinja,
    QMake,
    Meson,
    Simple,
    #[cfg(feature = "extended_build_sytems")]
    Cabal,
    #[cfg(feature = "extended_build_sytems")]
    Make,
    #[cfg(feature = "extended_build_sytems")]
    Cargo,
}

impl Default for FlatpakBuildSystem {
    fn default() -> Self {
        FlatpakBuildSystem::Autotools
    }
}
impl FlatpakBuildSystem {
    pub fn to_string(&self) -> String {
        match &self {
            FlatpakBuildSystem::Autotools => AUTOTOOLS.to_string(),
            FlatpakBuildSystem::CMake => CMAKE.to_string(),
            FlatpakBuildSystem::CMakeNinja => CMAKE_NINJA.to_string(),
            FlatpakBuildSystem::QMake => QMAKE.to_string(),
            FlatpakBuildSystem::Meson => MESON.to_string(),
            FlatpakBuildSystem::Simple => SIMPLE.to_string(),
            #[cfg(feature = "extended_build_sytems")]
            FlatpakBuildSystem::Cabal => CABAL.to_string(),
            #[cfg(feature = "extended_build_sytems")]
            FlatpakBuildSystem::Make => MAKE.to_string(),
            #[cfg(feature = "extended_build_sytems")]
            FlatpakBuildSystem::Cargo => CARGO.to_string(),
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
        #[cfg(feature = "extended_build_sytems")]
        if build_sys == CABAL {
            return Ok(FlatpakBuildSystem::Cabal);
        }
        #[cfg(feature = "extended_build_sytems")]
        if build_sys == MAKE {
            return Ok(FlatpakBuildSystem::Make);
        }
        #[cfg(feature = "extended_build_sytems")]
        if build_sys == CARGO {
            return Ok(FlatpakBuildSystem::Cargo);
        }
        Err(format!("Invalid build system {}.", build_sys))
    }

    pub fn serialize<S>(x: &Option<FlatpakBuildSystem>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(build_system) = x {
            return s.serialize_str(&build_system.to_string());
        }
        panic!("This should not happen.");
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<FlatpakBuildSystem>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let buf = String::deserialize(deserializer)?;

        match FlatpakBuildSystem::from_string(&buf) {
            Ok(b) => Ok(Some(b)),
            Err(e) => Err(e).map_err(serde::de::Error::custom),
        }
    }
}
