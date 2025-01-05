use devfile::lifecycle::find_pod_by_ws_name::find_pod_by_ws_name;
use helper::{select_pod, Helper};
use k8s_openapi::api::core::v1::Pod;
use shell::start_it_shell::start_it_shell;

#[tracing::instrument(level = "trace")]
pub async fn spawn_shell(
    namespace: String,
    workspace_name: String,
    container_name: Option<String>,
    shell: String,
) {
    let client = match Helper::get_client().await {
        Some(iencli) => iencli,
        None => return,
    };
    let pod = match find_pod_by_ws_name(
        client.clone(),
        Some(workspace_name),
        Some(namespace.clone()),
    )
    .await
    {
        Some(pod) => pod,
        None => {
            tracing::error!("Pod's not found");
            return;
        }
    };
    let container_cible = match select_pod::select_pod(container_name, pod.clone()) {
        Some(c_name) => c_name,
        None => return,
    };

    let pod_api = Helper::get_api::<Pod>(client, Some(namespace));

    let _ = start_it_shell(pod.metadata.name.unwrap(), container_cible, shell, pod_api).await;
}
