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
            ArchiveType::SevenZip => SEVENZIP.to_string(),
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
}
