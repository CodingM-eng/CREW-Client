use std::fmt;

/// CREW Core and Loader standard error enumeration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrewError {
    /// Network or HTTP transport error.
    Network(String),
    /// SHA-1 or SHA-256 checksum verification failed.
    ChecksumMismatch {
        file: String,
        expected: String,
        actual: String,
    },
    /// A requested loader or game version was not found.
    VersionNotFound(String),
    /// A loader does not support the given Minecraft version.
    UnsupportedVersion {
        loader: String,
        mc_version: String,
        reason: String,
    },
    /// Invalid Maven coordinate syntax.
    MavenParseError(String),
    /// File system or I/O failure.
    Io(String),
    /// JSON serialization or deserialization failure.
    Json(String),
    /// General invalid data or operation.
    InvalidData(String),
}

impl fmt::Display for CrewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Network(msg) => write!(f, "Ağ hatası: {msg}"),
            Self::ChecksumMismatch { file, expected, actual } => {
                write!(f, "Sağlama toplamı (checksum) uyuşmazlığı [{file}]: Beklenen {expected}, Gerçek {actual}")
            }
            Self::VersionNotFound(v) => write!(f, "Sürüm bulunamadı: {v}"),
            Self::UnsupportedVersion { loader, mc_version, reason } => {
                write!(f, "{loader} yükleyicisi Minecraft {mc_version} sürümünü desteklemiyor: {reason}")
            }
            Self::MavenParseError(coord) => write!(f, "Geçersiz Maven koordinatı: {coord}"),
            Self::Io(msg) => write!(f, "Giriş/Çıkış (I/O) hatası: {msg}"),
            Self::Json(msg) => write!(f, "JSON biçimlendirme hatası: {msg}"),
            Self::InvalidData(msg) => write!(f, "Geçersiz veri: {msg}"),
        }
    }
}

impl std::error::Error for CrewError {}

impl From<std::io::Error> for CrewError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.to_string())
    }
}

impl From<serde_json::Error> for CrewError {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err.to_string())
    }
}
