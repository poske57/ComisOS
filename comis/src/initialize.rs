use std::fs;
use std::path::Path;
use std::process::Command;

pub fn initialize(repositories_path: &Path) -> std::io::Result<()> {
    if repositories_path.exists() {
        return Ok(());
    }
    fs::create_dir_all(repositories_path)?;

    let username = whoami();

    let flake_nix: &str = include_str!("../assets/flake.nix");
    let configuration_nix: &str = include_str!("../assets/configuration.nix");
    let home_nix: &str = include_str!("../assets/home.nix");
    let agents: &str = include_str!("../assets/AGENTS.md");
    std::fs::write(repositories_path.join("flake.nix"), flake_nix.replace("%username%", &username))?;
    std::fs::write(repositories_path.join("configuration.nix"), configuration_nix.replace("%username%", &username))?;
    std::fs::write(repositories_path.join("home.nix"), home_nix.replace("%username%", &username))?;
    std::fs::write(repositories_path.join("AGENTS.md"), agents)?;

    git_init(repositories_path)?;

    Ok(())
}

fn whoami() -> String {
    let out = Command::new("whoami")
        .output()
        .expect("whoami failed");

    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

fn git_init(dir: &Path) -> std::io::Result<()> {
    let status = Command::new("git")
        .arg("init")
        .current_dir(dir)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("git init failed: {:?}", status),
        ))
    }
}
