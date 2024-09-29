use k8s_openapi::NamespaceResourceScope;
use kube::{client, Api, Client, Resource};

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
}
