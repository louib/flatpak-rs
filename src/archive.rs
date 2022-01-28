use serde::{Deserialize, Deserializer, Serializer};

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
#[derive(PartialEq)]
pub enum ArchiveType {
    Rpm,
    Tar,
    TarGzip,
    TarCompress,
    TarBzip2,
    TarLzip,
    TarLzma,
    TarLzop,
    TarXz,
    Zip,
    SevenZip,
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
            ArchiveType::SevenZip => SEVENZIP.to_string(),
            ArchiveType::Tar => TAR.to_string(),
            ArchiveType::TarGzip => TAR_GZIP.to_string(),
            ArchiveType::TarCompress => TAR_COMPRESS.to_string(),
            ArchiveType::TarBzip2 => TAR_BZIP2.to_string(),
            ArchiveType::TarLzip => TAR_LZIP.to_string(),
            ArchiveType::TarLzma => TAR_LZMA.to_string(),
            ArchiveType::TarLzop => TAR_LZOP.to_string(),
            ArchiveType::TarXz => TAR_XZ.to_string(),
            ArchiveType::Zip => ZIP.to_string(),
        }
    }
    pub fn from_string(archive_type: &str) -> Result<ArchiveType, String> {
        if archive_type == RPM {
            return Ok(ArchiveType::Rpm);
        }
        if archive_type == SEVENZIP {
            return Ok(ArchiveType::SevenZip);
        }
        if archive_type == ZIP {
            return Ok(ArchiveType::Zip);
        }
        if archive_type == TAR {
            return Ok(ArchiveType::Tar);
        }
        if archive_type == TAR_XZ {
            return Ok(ArchiveType::TarXz);
        }
        if archive_type == TAR_LZOP {
            return Ok(ArchiveType::TarLzop);
        }
        if archive_type == TAR_COMPRESS {
            return Ok(ArchiveType::TarCompress);
        }
        if archive_type == TAR_BZIP2 {
            return Ok(ArchiveType::TarBzip2);
        }
        if archive_type == TAR_GZIP {
            return Ok(ArchiveType::TarGzip);
        }
        if archive_type == TAR_LZIP {
            return Ok(ArchiveType::TarLzip);
        }
        if archive_type == TAR_LZMA {
            return Ok(ArchiveType::TarLzma);
        }
        if archive_type == TAR_LZOP {
            return Ok(ArchiveType::TarLzop);
        }
        Err(format!("Invalid archive type {}.", archive_type))
    }

    /// Detects the archive type from a path or a URL, using
    /// the extension only.
    pub fn from_path(path: &str) -> Option<ArchiveType> {
        let path = path.to_lowercase();
        if path.ends_with(".tar") {
            return Some(ArchiveType::Tar);
        }
        if path.ends_with(".tar.gz") || path.ends_with(".tgz") || path.ends_with(".taz") {
            return Some(ArchiveType::TarGzip);
        }
        if path.ends_with(".tar.z") || path.ends_with(".taz") {
            return Some(ArchiveType::TarCompress);
        }
        if path.ends_with(".tar.bz2") || path.ends_with(".tz2") {
            return Some(ArchiveType::TarBzip2);
        }
        if path.ends_with(".tbz2") || path.ends_with(".tbz") {
            return Some(ArchiveType::TarBzip2);
        }
        if path.ends_with(".tar.lz") {
            return Some(ArchiveType::TarLzip);
        }
        if path.ends_with(".tar.lzma") || path.ends_with(".tlz") {
            return Some(ArchiveType::TarLzma);
        }
        if path.ends_with(".tar.lzo") {
            return Some(ArchiveType::TarLzop);
        }
        if path.ends_with(".tar.xz") || path.ends_with(".txz") {
            return Some(ArchiveType::TarXz);
        }
        if path.ends_with(".zip") {
            return Some(ArchiveType::Zip);
        }
        if path.ends_with(".rpm") {
            return Some(ArchiveType::Rpm);
        }
        if path.ends_with(".7z") {
            return Some(ArchiveType::SevenZip);
        }
        None
    }
}

pub fn serialize_to_string<S>(x: &Option<ArchiveType>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&x.as_ref().unwrap().to_string())
}

pub fn deserialize_from_string<'de, D>(deserializer: D) -> Result<Option<ArchiveType>, D::Error>
where
    D: Deserializer<'de>,
{
    let archive_type = match String::deserialize(deserializer) {
        Ok(t) => t,
        Err(_e) => return Ok(None),
    };

    match ArchiveType::from_string(&archive_type) {
        Ok(t) => Ok(Some(t)),
        // Err(e) => Err(e).map_err(serde::de::Error::custom),
        Err(_e) => Ok(None),
    }
    // ArchiveType::from_string(&buf).map_err(serde::de::Error::custom)
}
