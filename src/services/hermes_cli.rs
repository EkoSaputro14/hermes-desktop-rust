//! Native process & filesystem manager for `~/.hermes`
//! Replaces Electron's `hermes.checkInstall` and `hermes.startInstall` IPC handlers.

use std::path::PathBuf;
use tokio::fs;
use tokio::process::Command;

pub struct HermesCliManager;

impl HermesCliManager {
    pub fn hermes_root_dir() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".hermes")
    }

    pub async fn verify_environment() -> String {
        let root = Self::hermes_root_dir();
        let plugins_dir = root.join("plugins");
        let profiles_dir = root.join("profiles");
        let logs_dir = root.join("logs");

        let _ = fs::create_dir_all(&plugins_dir).await;
        let _ = fs::create_dir_all(&profiles_dir).await;
        let _ = fs::create_dir_all(&logs_dir).await;

        let config_path = root.join("config.yaml");
        if !config_path.exists() {
            let default_yaml = "# Hermes Agent Configuration (Managed by Hermes Desktop Rust)\n\
                version: 2\n\
                default_profile: default\n\
                memory:\n\
                  provider: honcho_sqlite\n\
                  auto_consolidate: true\n\
                toolsets:\n\
                  shell_exec: true\n\
                  filesystem: true\n\
                  web_search: true\n";
            let _ = fs::write(&config_path, default_yaml).await;
        }

        let cli_check = Command::new("hermes")
            .arg("--version")
            .output()
            .await;

        match cli_check {
            Ok(out) if out.status.success() => {
                let raw = String::from_utf8_lossy(&out.stdout);
                let first_line = raw
                    .lines()
                    .next()
                    .unwrap_or("hermes-agent")
                    .trim();
                format!("local ({})", first_line)
            }
            _ => "local".to_string(),
        }
    }
}
