use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::error::{GetInstalledExtensionsError, InstallExtensionError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extensions {}

impl Default for Extensions {
    fn default() -> Self {
        Self::new()
    }
}

impl Extensions {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_mendatory_extensions(&self) -> Vec<String> {
        vec![
            "ms-kubernetes-tools.vscode-kubernetes-tools".to_string(),
            "ms-vscode-remote.remote-ssh".to_string(),
            "ms-vscode.remote-server".to_string(),
        ]
    }

    #[tracing::instrument(level = "trace")]
    pub fn get_installed_extensions(
        &self,
    ) -> Result<HashMap<String, String>, GetInstalledExtensionsError> {
        let mut installed_extensions = HashMap::new();
        let default_command = "code --list-extensions --show-versions";
        tracing::trace!("Getting installed extensions");
        let output = if cfg!(target_os = "windows") {
            std::process::Command::new("cmd")
                .args(["/C", default_command])
                .output()
                .expect("failed to execute process")
        } else {
            std::process::Command::new("sh")
                .arg("-c")
                .arg(default_command)
                .output()
                .expect("failed to execute process")
        };
        let output = String::from_utf8_lossy(&output.stdout);
        for line in output.lines() {
            let mut parts = line.split('@');
            let name = parts.next().unwrap();
            let version = parts.next().unwrap();
            installed_extensions.insert(name.to_string(), version.to_string());
        }
        Ok(installed_extensions)
    }

    #[tracing::instrument(level = "trace")]
    pub fn check_missing_extensions(&self) -> Result<Vec<String>, GetInstalledExtensionsError> {
        let installed_extensions = self.get_installed_extensions()?;
        let mendatory_extensions = self.get_mendatory_extensions();
        let mut missing_extensions = Vec::new();
        for extension in mendatory_extensions {
            if !installed_extensions.contains_key(&extension) {
                missing_extensions.push(extension);
            }
        }
        Ok(missing_extensions)
    }

    #[tracing::instrument(level = "trace")]
    pub fn install_extension(&self, extension: &str) -> Result<(), InstallExtensionError> {
        let command = format!("code --install-extension {}", extension);
        tracing::trace!("Installing extension: {}", extension);
        let output = if cfg!(target_os = "windows") {
            std::process::Command::new("cmd")
                .args(["/C", &command])
                .output()
                .expect("failed to execute process")
        } else {
            std::process::Command::new("sh")
                .arg("-c")
                .arg(command)
                .output()
                .expect("failed to execute process")
        };
        let out = String::from_utf8_lossy(&output.stdout);
        let err = String::from_utf8_lossy(&output.stderr);
        tracing::trace!("output: {:?} : {:?}", out, err);
        if err != "" {
            return Err(InstallExtensionError::FailedToInstallExtension(
                err.to_string(),
            ));
        }
        Ok(())
    }
}
