use k8s_openapi::{api::core::v1::Pod, NamespaceResourceScope};
use kube::{
    api::{AttachParams, AttachedProcess},
    client, Api, Client, Resource,
};
use std::collections::HashMap;
use tokio::io::AsyncReadExt;

pub mod error;
pub mod select_pod;

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
        container: String,
    ) -> Result<HashMap<String, String>, error::GetPodEnvvarsError> {
        let pods: Api<Pod> = Api::namespaced(client, &namespace);
        let mut result = match Self::get_attach_process(
            pods,
            podname,
            container,
            vec!["env".to_string()],
            AttachParams::interactive_tty(),
        )
        .await
        {
            Ok(result) => result,
            Err(err) => {
                tracing::error!("Could not exec into pod : {:?}", err);
                return Err(error::GetPodEnvvarsError::AttachError(err));
            }
        };
        let mut stdout = match result.stdout() {
            Some(stdout) => stdout,
            None => {
                tracing::error!("Could not get stdout from exec");
                return Err(error::GetPodEnvvarsError::StdoutAttachError);
            }
        };
        let mut buff = String::new();
        match stdout.read_to_string(&mut buff).await {
            Ok(_) => {}
            Err(err) => {
                tracing::error!("Could not read stdout : {:?}", err);
                return Err(error::GetPodEnvvarsError::IoError(err));
            }
        }
        tracing::trace!("Envvars : {:?}", buff);
        if buff.is_empty() || buff == "\n" {
            tracing::error!("Empty return from exec");
            return Err(error::GetPodEnvvarsError::EmptyReturn);
        }
        let envvars: HashMap<String, String> = buff
            .split("\n")
            .map(|line| {
                let mut parts = line.split("=");
                let key = parts.next().unwrap().to_string();
                let value = parts.next().unwrap_or("").to_string().replace("\r", "");
                (key, value)
            })
            .collect();

        Ok(envvars)
    }

    #[tracing::instrument(level = "trace", skip(pods))]
    pub async fn get_attach_process(
        pods: Api<Pod>,
        podname: String,
        container: String,
        command: Vec<String>,
        attach_params: AttachParams,
    ) -> Result<AttachedProcess, error::GetAttachProcess> {
        let mut attach = attach_params;
        if command.is_empty() {
            tracing::error!("Empty command");
            return Err(error::GetAttachProcess::EmptyCommand);
        }
        attach = attach.container(container);
        match pods.exec(&podname, command, &attach).await {
            Ok(result) => Ok(result),
            Err(err) => {
                tracing::error!("Could not exec into pod : {:?}", err);
                return Err(error::GetAttachProcess::AttachError(err));
            }
        }
    }
}
