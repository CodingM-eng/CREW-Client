use crate::checksum::{sha1_hex, sha256_hex, verify_sha1, verify_sha256};
use crate::error::CrewError;
use std::time::Duration;

/// Standard CREW Client HTTP Client.
pub struct HttpClient {
    agent: ureq::Agent,
    user_agent: String,
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new("CREW-Client/2.1.0")
    }
}

impl HttpClient {
    pub fn new(user_agent: &str) -> Self {
        let agent = ureq::AgentBuilder::new()
            .timeout_connect(Duration::from_secs(10))
            .timeout_read(Duration::from_secs(20))
            .build();

        Self {
            agent,
            user_agent: user_agent.to_string(),
        }
    }

    /// Fetch raw bytes from URL.
    pub fn get_bytes(&self, url: &str) -> Result<Vec<u8>, CrewError> {
        let response = self
            .agent
            .get(url)
            .set("User-Agent", &self.user_agent)
            .call()
            .map_err(|e| CrewError::Network(format!("{url}: {e}")))?;

        let mut bytes = Vec::new();
        use std::io::Read;
        response
            .into_reader()
            .read_to_end(&mut bytes)
            .map_err(|e| CrewError::Io(format!("{url}: {e}")))?;

        Ok(bytes)
    }

    /// Fetch and deserialize JSON from URL.
    pub fn get_json<T: serde::de::DeserializeOwned>(&self, url: &str) -> Result<T, CrewError> {
        let bytes = self.get_bytes(url)?;
        serde_json::from_slice(&bytes).map_err(|e| {
            CrewError::Json(format!("{url}: {e} (boyut: {} bayt)", bytes.len()))
        })
    }

    /// Fetch and verify bytes against expected SHA-1 or SHA-256 if provided.
    pub fn download_checked(
        &self,
        url: &str,
        expected_sha1: Option<&str>,
        expected_sha256: Option<&str>,
    ) -> Result<Vec<u8>, CrewError> {
        let bytes = self.get_bytes(url)?;

        if let Some(sha1) = expected_sha1 {
            if !verify_sha1(&bytes, sha1) {
                return Err(CrewError::ChecksumMismatch {
                    file: url.to_string(),
                    expected: sha1.to_string(),
                    actual: sha1_hex(&bytes),
                });
            }
        }

        if let Some(sha256) = expected_sha256 {
            if !verify_sha256(&bytes, sha256) {
                return Err(CrewError::ChecksumMismatch {
                    file: url.to_string(),
                    expected: sha256.to_string(),
                    actual: sha256_hex(&bytes),
                });
            }
        }

        Ok(bytes)
    }
}
