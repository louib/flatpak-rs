use serde::Deserialize;

pub const RPM: &str = "rpm";
pub const TAR: &str = "tar";
pub const TAR_GZIP: &str = "tar-gzip";
pub const TAR_COMPRESS: &str = "tar-compress";
pub const TAR_BZIP2: &str = "tar-bzip2";
pub const TAR_LZIP: &str = "tar-lzip";
pub const TAR_LZMA: &str = "tar-lzma";
pub const TAR_LZOP: &str = "tar-lzop";
pub const TAR_XZ: &str = "tar-xz";
pub const ZIP: &str = "zip";
pub const SEVENZIP: &str = "7z";

#[derive(Clone)]
#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Hash)]
pub enum ArchiveType {
    Rpm,
}

impl Default for ArchiveType {
    fn default() -> Self {
        ArchiveType::Rpm
    }
}
impl ArchiveType {
    pub fn to_string(&self) -> String {
        match &self {
            ArchiveType::Rpm => RPM.to_string(),
        }
    }
    pub fn from_string(archive_type: &str) -> Result<ArchiveType, String> {
        if archive_type == RPM {
            return Ok(ArchiveType::Rpm);
        }
        Err(format!("Invalid archive type {}.", archive_type))
    }
}
