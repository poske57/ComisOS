use std::env;
use std::fs;
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Settings {
    pub repositories_path: PathBuf,
    pub rebuild_command: String,
    pub agent: String,
}

// デフォルト値はここにまとめる。パラメータを追加するときは
// Settings のフィールドと Default の両方に追記する。
impl Default for Settings {
    fn default() -> Self {
        Self {
            repositories_path: PathBuf::from("$HOME/nix"),
            rebuild_command: "sudo nixos-rebuild switch --flake ~/nix#nixos --impure"
                .into(),
            agent: "opencode".into(),
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        let path = match config_dir() {
            Some(dir) => dir.join("comis").join("settings.json"),
            None => return Self::default().expand(),
        };
        let raw = match fs::read_to_string(&path) {
            Ok(raw) => raw,
            Err(_) => return Self::default().expand(),
        };
        serde_json::from_str::<Self>(&raw).unwrap_or_default().expand()
    }

    fn expand(mut self) -> Self {
        self.repositories_path = expand_env(&self.repositories_path);
        self
    }
}

fn config_dir() -> Option<PathBuf> {
    if let Some(dir) = env::var_os("XDG_CONFIG_HOME").filter(|dir| !dir.is_empty()) {
        return Some(PathBuf::from(dir));
    }
    env::var_os("HOME").map(|home| PathBuf::from(home).join(".config"))
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
