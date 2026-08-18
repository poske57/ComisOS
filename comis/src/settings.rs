use std::env;
use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    #[serde(default)]
    pub repositories_path: PathBuf,
    #[serde(default)]
    pub rebuild_command: Option<String>,
    #[serde(default)]
    pub agent: Option<String>,
}

impl Settings {
    pub fn load() -> Result<Self, SettingsError> {
        let path = config_dir()?.join("comis").join("settings.json");
        let raw = fs::read_to_string(&path)
            .map_err(|e| SettingsError::Read { path, source: e })?;
        let mut settings: Self =
            serde_json::from_str(&raw).map_err(SettingsError::Parse)?;
        settings.expand();
        Ok(settings)
    }

    fn expand(&mut self) {
        self.repositories_path = expand_env(&self.repositories_path);
    }
}

fn config_dir() -> Result<PathBuf, SettingsError> {
    match env::var_os("XDG_CONFIG_HOME").filter(|dir| !dir.is_empty()) {
        Some(dir) => Ok(PathBuf::from(dir)),
        None => {
            let home = env::var_os("HOME").ok_or(SettingsError::NoConfigDir)?;
            Ok(PathBuf::from(home).join(".config"))
        }
    }
}

fn expand_env(path: &std::path::Path) -> PathBuf {
    let raw = path.to_string_lossy();
    match raw.strip_prefix("$HOME/") {
        Some(rest) => env::var_os("HOME")
            .map(|home| PathBuf::from(home).join(rest))
            .unwrap_or_else(|| path.to_path_buf()),
        None => path.to_path_buf(),
    }
}

#[derive(Debug)]
pub enum SettingsError {
    NoConfigDir,
    Read {
        path: PathBuf,
        source: std::io::Error,
    },
    Parse(serde_json::Error),
}

impl std::fmt::Display for SettingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingsError::NoConfigDir => {
                write!(f, "XDG_CONFIG_HOME and HOME are both unset")
            }
            SettingsError::Read { path, source } => {
                write!(f, "failed to read {}: {source}", path.display())
            }
            SettingsError::Parse(e) => write!(f, "failed to parse settings.json: {e}"),
        }
    }
}

impl std::error::Error for SettingsError {}
