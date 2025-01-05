use crd::dev_work_space::DevWorkspace;
use helper::{base_info::BaseInfo, Helper};
use k8s_openapi::serde_json;
use kube::api::ListParams;
use serde::{Deserialize, Serialize};

use super::{Format, Workspace};

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ListOutput {
    pub namespace: Option<String>,
    pub workspace_name: Option<String>,
    pub workspace_id: Option<String>,
    pub status: Option<String>,
}

impl Workspace {
    #[tracing::instrument(level = "trace")]
    pub async fn list(base_info: BaseInfo, format: Option<Format>) {
        let client = match Helper::get_client().await {
            Some(iencli) => iencli,
            None => return,
        };
        let api = Helper::get_api::<DevWorkspace>(client, base_info.namespace.clone());
        let list = match api.list(&ListParams::default()).await {
            Ok(list) => list,
            Err(err) => {
                tracing::error!("Could not get workspace: {:?}", err);
                return;
            }
        };
        let mut workspaces = vec![];
        for workspace in list {
            let status = workspace.status.unwrap();
            workspaces.push(ListOutput {
                namespace: base_info.namespace.clone(),
                workspace_name: workspace.metadata.name,
                workspace_id: Some(status.devworkspace_id),
                status: status.phase,
            });
        }

        match format {
            Some(Format::Json) => {
                println!("{}", serde_json::to_string_pretty(&workspaces).unwrap());
            }
            Some(Format::Yaml) => {
                println!("{}", serde_yaml::to_string(&workspaces).unwrap());
            }
            None => {
                for workspace in workspaces {
                    tracing::info!("Namespace: {:?}", workspace.namespace);
                    tracing::info!("Workspace name: {:?}", workspace.workspace_name);
                    tracing::info!("Workspace id: {:?}", workspace.workspace_id);
                    tracing::info!("Status: {:?}", workspace.status);
                    tracing::info!("----------------------");
                }
            }
        }
    }
}
