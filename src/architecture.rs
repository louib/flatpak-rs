use serde::{Deserialize, Deserializer, Serializer};

pub const X86: &str = "x86_64";

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Hash)]
#[derive(PartialEq)]
pub enum FlatpakArchitecture {
    X86,
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
        }
    }

    pub fn from_string(arch: &str) -> Result<FlatpakArchitecture, String> {
        if arch == X86 {
            return Ok(FlatpakArchitecture::X86);
        }
        Err(format!("Invalid architecture {}.", arch))
    }
}
