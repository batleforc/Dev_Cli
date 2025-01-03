use k8s_openapi::{api::core::v1::Pod, NamespaceResourceScope};
use kube::{api::AttachParams, client, Api, Client, Resource};
use std::collections::HashMap;
use tokio::io::AsyncReadExt;

#[derive(Clone, Debug)]
pub struct Helper {}

impl Helper {
    pub fn get_workspace_from_env() -> Option<String> {
        std::env::var("DEVWORKSPACE_NAME").ok()
    }

    pub fn get_workspace_id_from_env() -> Option<String> {
        std::env::var("DEVWORKSPACE_ID").ok()
    }

    pub fn get_workspace_name_from_env() -> Option<String> {
        std::env::var("DEVWORKSPACE_NAME").ok()
    }

    pub fn get_podname_from_env() -> Option<String> {
        std::env::var("HOSTNAME").ok()
    }

    #[tracing::instrument(level = "trace")]
    pub async fn get_client() -> Option<Client> {
        match client::Client::try_default().await {
            Ok(iencli) => {
                tracing::trace!("Kube client created");
                Some(iencli)
            }
            Err(err) => {
                tracing::error!("Could not instanciate kube Client : {:?}", err);
                None
            }
        }
    }

    #[tracing::instrument(level = "trace", skip(client))]
    pub fn get_api<T: Resource<Scope = NamespaceResourceScope>>(
        client: Client,
        namespace: Option<String>,
    ) -> Api<T>
    where
        <T as kube::Resource>::DynamicType: Default,
    {
        match namespace {
            Some(namespace) => Api::namespaced(client, &namespace),
            None => Api::default_namespaced(client),
        }
    }

    #[tracing::instrument(level = "trace", skip(client))]
    pub async fn get_pod_envvars(
        client: Client,
        namespace: String,
        podname: String,
        container: Option<String>,
    ) -> Option<HashMap<String, String>> {
        let pods: Api<Pod> = Api::namespaced(client, &namespace);
        let mut attach = AttachParams::interactive_tty();
        if let Some(container) = container {
            attach = attach.container(container);
        }
        let mut result = match pods.exec(&podname, vec!["env"], &attach).await {
            Ok(result) => result,
            Err(err) => {
                tracing::error!("Could not exec into pod : {:?}", err);
                return None;
            }
        };
        let mut stdout = result.stdout().unwrap();
        let mut buff = String::new();
        stdout.read_to_string(&mut buff).await.unwrap();
        tracing::trace!("Envvars : {:?}", buff);
        if buff.is_empty() || buff == "\n" {
            return None;
        }
        let envvars: HashMap<String, String> = buff
            .split("\n")
            .map(|line| {
                let mut parts = line.split("=");
                let key = parts.next().unwrap().to_string();
                let value = parts.next().unwrap_or("").to_string();
                (key, value)
            })
            .collect();

        Some(envvars)
    }
}
