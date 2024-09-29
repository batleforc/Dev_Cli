use serde::{Deserialize, Serialize};
use std::process::Command;
use url::form_urlencoded::byte_serialize;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct OpenCode {
    // is it needed ? Kubernetes context
    pub context: Option<String>,
    pub pod_name: Option<String>,
    pub namespace: Option<String>,
    pub container_name: Option<String>,
    pub container_image: Option<String>,
    pub path: Option<String>,
}

impl OpenCode {
    // https://github.com/vscode-kubernetes-tools/vscode-kubernetes-tools/issues/1207
    #[tracing::instrument(level = "trace")]
    pub fn generate_path(&self) -> String {
        let mut base_uri = String::from("vscode-remote://k8s-container");
        if let Some(context) = &self.context {
            base_uri = format!("{}+context={}", base_uri, context);
        }
        if let Some(pod_name) = &self.pod_name {
            base_uri = format!("{}+podname={}", base_uri, pod_name);
        }
        if let Some(namespace) = &self.namespace {
            base_uri = format!("{}+namespace={}", base_uri, namespace);
        }
        if let Some(container_name) = &self.container_name {
            base_uri = format!("{}+name={}", base_uri, container_name);
        }
        if let Some(container_image) = &self.container_image {
            let parsed_uri: String = byte_serialize(container_image.as_bytes()).collect();
            base_uri = format!("{}+image={}", base_uri, parsed_uri);
        }
        if let Some(path) = &self.path {
            base_uri = format!("{}{}", base_uri, path);
        }
        base_uri
    }

    #[tracing::instrument(level = "trace")]
    pub fn open(&self) {
        let path = self.generate_path();
        tracing::trace!("Opening VsCode with path: {:?}", path);
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", &format!("code --folder-uri {}", path)])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(format!("code --folder-uri {}", path))
                .output()
                .expect("failed to execute process")
        };

        let hello = output.stdout;
        tracing::trace!("output: {:?}", hello);
    }
}
