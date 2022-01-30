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
pub enum FlatpakArchiveType {
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

impl Default for FlatpakArchiveType {
    fn default() -> Self {
        FlatpakArchiveType::Rpm
    }
}
impl FlatpakArchiveType {
    pub fn to_string(&self) -> String {
        match &self {
            FlatpakArchiveType::Rpm => RPM.to_string(),
            FlatpakArchiveType::SevenZip => SEVENZIP.to_string(),
            FlatpakArchiveType::Tar => TAR.to_string(),
            FlatpakArchiveType::TarGzip => TAR_GZIP.to_string(),
            FlatpakArchiveType::TarCompress => TAR_COMPRESS.to_string(),
            FlatpakArchiveType::TarBzip2 => TAR_BZIP2.to_string(),
            FlatpakArchiveType::TarLzip => TAR_LZIP.to_string(),
            FlatpakArchiveType::TarLzma => TAR_LZMA.to_string(),
            FlatpakArchiveType::TarLzop => TAR_LZOP.to_string(),
            FlatpakArchiveType::TarXz => TAR_XZ.to_string(),
            FlatpakArchiveType::Zip => ZIP.to_string(),
        }
    }
    pub fn from_string(archive_type: &str) -> Result<FlatpakArchiveType, String> {
        if archive_type == RPM {
            return Ok(FlatpakArchiveType::Rpm);
        }
        if archive_type == SEVENZIP {
            return Ok(FlatpakArchiveType::SevenZip);
        }
        if archive_type == ZIP {
            return Ok(FlatpakArchiveType::Zip);
        }
        if archive_type == TAR {
            return Ok(FlatpakArchiveType::Tar);
        }
        if archive_type == TAR_XZ {
            return Ok(FlatpakArchiveType::TarXz);
        }
        if archive_type == TAR_LZOP {
            return Ok(FlatpakArchiveType::TarLzop);
        }
        if archive_type == TAR_COMPRESS {
            return Ok(FlatpakArchiveType::TarCompress);
        }
        if archive_type == TAR_BZIP2 {
            return Ok(FlatpakArchiveType::TarBzip2);
        }
        if archive_type == TAR_GZIP {
            return Ok(FlatpakArchiveType::TarGzip);
        }
        if archive_type == TAR_LZIP {
            return Ok(FlatpakArchiveType::TarLzip);
        }
        if archive_type == TAR_LZMA {
            return Ok(FlatpakArchiveType::TarLzma);
        }
        if archive_type == TAR_LZOP {
            return Ok(FlatpakArchiveType::TarLzop);
        }
        Err(format!("Invalid archive type {}.", archive_type))
    }

    /// Detects the archive type from a path or a URL, using
    /// the extension only.
    pub fn from_path(path: &str) -> Option<FlatpakArchiveType> {
        let path = path.to_lowercase();
        if path.ends_with(".tar") {
            return Some(FlatpakArchiveType::Tar);
        }
        if path.ends_with(".tar.gz") || path.ends_with(".tgz") || path.ends_with(".taz") {
            return Some(FlatpakArchiveType::TarGzip);
        }
        if path.ends_with(".tar.z") || path.ends_with(".taz") {
            return Some(FlatpakArchiveType::TarCompress);
        }
        if path.ends_with(".tar.bz2") || path.ends_with(".tz2") {
            return Some(FlatpakArchiveType::TarBzip2);
        }
        if path.ends_with(".tbz2") || path.ends_with(".tbz") {
            return Some(FlatpakArchiveType::TarBzip2);
        }
        if path.ends_with(".tar.lz") {
            return Some(FlatpakArchiveType::TarLzip);
        }
        if path.ends_with(".tar.lzma") || path.ends_with(".tlz") {
            return Some(FlatpakArchiveType::TarLzma);
        }
        if path.ends_with(".tar.lzo") {
            return Some(FlatpakArchiveType::TarLzop);
        }
        if path.ends_with(".tar.xz") || path.ends_with(".txz") {
            return Some(FlatpakArchiveType::TarXz);
        }
        if path.ends_with(".zip") {
            return Some(FlatpakArchiveType::Zip);
        }
        if path.ends_with(".rpm") {
            return Some(FlatpakArchiveType::Rpm);
        }
        if path.ends_with(".7z") {
            return Some(FlatpakArchiveType::SevenZip);
        }
        None
    }
}

pub fn serialize_to_string<S>(x: &Option<FlatpakArchiveType>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(build_system) = x {
        return s.serialize_str(&build_system.to_string());
    }
    panic!("This should not happen.");
}

pub fn deserialize_from_string<'de, D>(deserializer: D) -> Result<Option<FlatpakArchiveType>, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;

    match FlatpakArchiveType::from_string(&buf) {
        Ok(b) => Ok(Some(b)),
        Err(e) => Err(e).map_err(serde::de::Error::custom),
    }
}
