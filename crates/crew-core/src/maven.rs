use crate::error::CrewError;

/// Parsed Maven coordinate `group:artifact:version[:classifier][@extension]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MavenCoordinate {
    pub group: String,
    pub artifact: String,
    pub version: String,
    pub classifier: Option<String>,
    pub extension: String,
}

impl MavenCoordinate {
    /// Parse a Maven coordinate string like:
    /// - `net.fabricmc:fabric-loader:0.16.9`
    /// - `org.lwjgl:lwjgl:3.3.3:natives-windows@jar`
    /// - `org.ow2.asm:asm-tree:9.7`
    pub fn parse(coordinate_str: &str) -> Result<Self, CrewError> {
        let (coordinates, extension) = match coordinate_str.split_once('@') {
            Some((c, ext)) => (c, ext.trim().to_string()),
            None => (coordinate_str, "jar".to_string()),
        };

        let parts: Vec<&str> = coordinates.split(':').collect();
        let (group, artifact, version, classifier) = match parts.as_slice() {
            [g, a, v] => (*g, *a, *v, None),
            [g, a, v, c] => (*g, *a, *v, Some((*c).to_string())),
            _ => return Err(CrewError::MavenParseError(coordinate_str.to_string())),
        };

        if group.is_empty() || artifact.is_empty() || version.is_empty() {
            return Err(CrewError::MavenParseError(coordinate_str.to_string()));
        }

        Ok(Self {
            group: group.to_string(),
            artifact: artifact.to_string(),
            version: version.to_string(),
            classifier,
            extension,
        })
    }

    /// Convert coordinate into relative repository file path:
    /// `net/fabricmc/fabric-loader/0.16.9/fabric-loader-0.16.9.jar`
    pub fn to_path(&self) -> String {
        let file_name = match &self.classifier {
            Some(c) => format!("{}-{}-{}.{}", self.artifact, self.version, c, self.extension),
            None => format!("{}-{}.{}", self.artifact, self.version, self.extension),
        };
        format!(
            "{}/{}/{}/{}",
            self.group.replace('.', "/"),
            self.artifact,
            self.version,
            file_name
        )
    }

    /// Convert coordinate to full download URL given a base Maven repository URL.
    pub fn to_url(&self, repository_base: &str) -> String {
        let base = repository_base.trim_end_matches('/');
        format!("{}/{}", base, self.to_path())
    }
}

/// Helper function directly converting coordinate string to repository file path.
pub fn maven_path(coordinate_str: &str) -> Result<String, CrewError> {
    MavenCoordinate::parse(coordinate_str).map(|c| c.to_path())
}
