use reqwest::blocking::Client;
use std::fs::{self, File};
use std::io::copy;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct MavenPackage {
    pub group_id: String,
    pub artifact_id: String,
    pub version: String,
}

impl MavenPackage {
    pub fn new(group_id: &str, artifact_id: &str, version: &str) -> Self {
        Self {
            group_id: group_id.to_string(),
            artifact_id: artifact_id.to_string(),
            version: version.to_string(),
        }
    }

    pub fn file_name(&self) -> String {
        format!("{}-{}.jar", self.artifact_id, self.version)
    }

    pub fn url(&self) -> String {
        format!(
            "https://repo1.maven.org/maven2/{}/{}/{}/{}",
            self.group_id.replace('.', "/"),
            self.artifact_id,
            self.version,
            self.file_name()
        )
    }

    pub fn fetch(&self, folder: &Path) -> Result<(), Box<dyn std::error::Error>> {
        eprintln!(
            "Fetching package: {}:{}:{}",
            self.group_id, self.artifact_id, self.version
        );
        fs::create_dir_all(folder)?;
        let dest_path = folder.join(self.file_name());

        if dest_path.exists() {
            eprintln!("Already exists: {}", dest_path.display());
            return Ok(());
        }

        eprintln!("Downloading from URL: {}", self.url());
        let client = Client::new();
        let resp = client
            .get(self.url())
            .header("User-Agent", "Mozilla/5.0")
            .send()?;

        if !resp.status().is_success() {
            return Err(format!("Failed to fetch {}: {}", self.file_name(), resp.status()).into());
        }

        let mut out = File::create(&dest_path)?;
        let mut content = resp;
        copy(&mut content, &mut out)?;
        eprintln!("Saved to: {}", dest_path.display());
        Ok(())
    }
}
