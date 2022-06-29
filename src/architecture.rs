use serde::{Deserialize, Deserializer, Serializer};

pub const I386: &str = "i386";
pub const X86: &str = "x86_64";
pub const AARCH64: &str = "aarch64";
pub const ARM: &str = "arm";
pub const ARMEB: &str = "armeb";
pub const MIPSEL: &str = "mipsel";
pub const MIPS64EL: &str = "mips64el";

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Hash)]
#[derive(PartialEq)]
// FIXME I could not find a complete list of the architectures supported
// by flatpak-builder. There might be others, and some in the list might not
// be accurate.
pub enum FlatpakArchitecture {
    I386,
    X86,
    AARCH64,
    ARM,
    /// ARM big endian
    ARMEB,
    /// MIPS little endian
    MIPSEL,
    /// MIPS-64 little endian
    MIPS64EL,
}

impl Default for FlatpakArchitecture {
    fn default() -> Self {
        FlatpakArchitecture::X86
    }
}
impl FlatpakArchitecture {
    pub fn to_string(&self) -> String {
        match &self {
            FlatpakArchitecture::X86 => X86.to_string(),
            FlatpakArchitecture::I386 => I386.to_string(),
            FlatpakArchitecture::AARCH64 => AARCH64.to_string(),
            FlatpakArchitecture::ARM => ARM.to_string(),
            FlatpakArchitecture::ARMEB => ARMEB.to_string(),
            FlatpakArchitecture::MIPSEL => MIPSEL.to_string(),
            FlatpakArchitecture::MIPS64EL => MIPS64EL.to_string(),
        }
    }

    pub fn from_string(arch: &str) -> Result<FlatpakArchitecture, String> {
        if arch == X86 {
            return Ok(FlatpakArchitecture::X86);
        }
        if arch == I386 {
            return Ok(FlatpakArchitecture::I386);
        }
        if arch == AARCH64 {
            return Ok(FlatpakArchitecture::AARCH64);
        }
        if arch == ARM {
            return Ok(FlatpakArchitecture::ARM);
        }
        if arch == ARMEB {
            return Ok(FlatpakArchitecture::ARMEB);
        }
        if arch == MIPSEL {
            return Ok(FlatpakArchitecture::MIPSEL);
        }
        if arch == MIPS64EL {
            return Ok(FlatpakArchitecture::MIPS64EL);
        }
        Err(format!("Invalid architecture {}.", arch))
    }
}
