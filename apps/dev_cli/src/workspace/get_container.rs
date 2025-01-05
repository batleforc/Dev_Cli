use devfile::lifecycle::find_pod_by_ws_name::find_pod_by_ws_name;
use helper::{base_info::BaseInfo, Helper};
use k8s_openapi::serde_json;
use serde::{Deserialize, Serialize};

use super::{Format, Workspace};

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct GetContainerOutput {
    pub namespace: Option<String>,
    pub workspace_name: Option<String>,
    pub workspace_id: Option<String>,
    pub podname: Option<String>,
    pub container: Vec<String>,
}

impl Workspace {
    #[tracing::instrument(level = "trace")]
    pub async fn get_container(base_info: BaseInfo, format: Option<Format>) {
        let client = match Helper::get_client().await {
            Some(iencli) => iencli,
            None => return,
        };
        let pod = find_pod_by_ws_name(
            client,
            base_info.workspace_name.clone(),
            base_info.namespace.clone(),
        )
        .await;
        if pod.is_none() {
            tracing::error!("Pod's not found");
            return;
        }
        let pod = pod.unwrap();
        let id = match base_info.workspace_id {
            Some(id) => id,
            None => {
                match pod
                    .metadata
                    .labels
                    .unwrap()
                    .get("controller.devfile.io/devworkspace_id")
                {
                    Some(id) => id.to_string(),
                    None => {
                        tracing::error!("Could not get workspace id");
                        "".to_string()
                    }
                }
            }
        };
        let output = GetContainerOutput {
            namespace: base_info.namespace,
            workspace_name: base_info.workspace_name,
            workspace_id: Some(id),
            podname: Some(pod.metadata.name.unwrap()),
            container: pod
                .spec
                .unwrap()
                .containers
                .into_iter()
                .map(|c| c.name)
                .collect::<Vec<String>>(),
        };
        match format {
            Some(Format::Json) => {
                println!("{}", serde_json::to_string_pretty(&output).unwrap());
            }
            Some(Format::Yaml) => {
                println!("{}", serde_yaml::to_string(&output).unwrap());
            }
            None => {
                tracing::info!("Namespace: {:?}", output.namespace);
                tracing::info!("Workspace name: {:?}", output.workspace_name);
                tracing::info!("Workspace id: {:?}", output.workspace_id);
                tracing::info!("Podname: {:?}", output.podname);
                tracing::info!("Container: {:?}", output.container);
            }
        }
    }
}
