use anyhow::{Context, Result, anyhow};
use plist::Value;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct AppInfo {
    pub path: PathBuf,
    pub name: String,
    pub bundle_id: String,
    pub bundle_name: String,
    pub organization: String,
}

impl AppInfo {
    /// Construct AppInfo from .app path
    pub fn from_path(app_path: &Path) -> Result<Self> {
        let plist_path = Path::new(app_path).join("Contents").join("Info.plist");

        if !plist_path.exists() {
            anyhow::bail!("Info.plist not found in {}", app_path.display());
        }

        let plist = Value::from_file(&plist_path)
            .with_context(|| format!("Failed to read plist: {}", plist_path.display()))?;

        let bundle_id = plist
            .as_dictionary()
            .and_then(|d| d.get("CFBundleIdentifier"))
            .and_then(|v| v.as_string())
            .ok_or_else(|| {
                anyhow::anyhow!("CFBundleIdentifier not found in {}", plist_path.display())
            })?;

        let app_name = Path::new(&app_path)
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .into_owned();

        let executable_name = plist
            .as_dictionary()
            .and_then(|d| d.get("CFBundleExecutable"))
            .and_then(|v| v.as_string())
            .ok_or_else(|| anyhow!("CFBundleExecutable not found in {}", plist_path.display()))?;

        let organization = bundle_id.split('.').nth(1).unwrap_or("").to_string();

        Ok(Self {
            path: app_path.to_path_buf(),
            name: app_name.to_string(),
            bundle_id: bundle_id.to_string(),
            bundle_name: executable_name.to_string(),
            organization,
        })
    }
}

impl Default for AppInfo {
    fn default() -> Self {
        Self {
            path: PathBuf::new(),
            name: String::new(),
            bundle_id: String::new(),
            bundle_name: String::new(),
            organization: String::new(),
        }
    }
}
