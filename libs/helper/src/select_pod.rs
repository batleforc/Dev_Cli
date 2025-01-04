use k8s_openapi::api::core::v1::Pod;

#[tracing::instrument(level = "trace")]
pub fn select_pod(target_name: Option<String>, pod: Pod) -> Option<String> {
    match target_name {
        Some(container_named) => {
            if !pod
                .spec
                .clone()
                .unwrap()
                .containers
                .into_iter()
                .any(|c| c.name == container_named)
            {
                tracing::error!(
                    "Pod does not have container : {}",
                    container_named.to_string()
                );
                return None;
            }
            Some(container_named)
        }
        None => match pod.spec.unwrap().containers.first() {
            Some(container) => {
                tracing::info!("Using first container : {}", container.name.to_string());
                Some(container.name.to_string())
            }
            None => {
                tracing::error!("No container in the pod ? what did you do?");
                return None;
            }
        },
    }
}
