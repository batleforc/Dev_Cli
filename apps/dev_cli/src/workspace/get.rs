use crd::dev_work_space::DevWorkspace;
use helper::{base_info::BaseInfo, Helper};
use k8s_openapi::serde_json;
use serde::{Deserialize, Serialize};

use super::{Format, Workspace};

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct GetOutput {
    pub namespace: Option<String>,
    pub workspace_name: Option<String>,
    pub workspace_id: Option<String>,
    pub podname: Option<String>,
    pub is_in_pod: bool,
}

impl Workspace {
    #[tracing::instrument(level = "trace")]
    pub async fn get(mut base_info: BaseInfo, format: Option<Format>) {
        if base_info.is_in_pod {
            let client = match Helper::get_client().await {
                Some(iencli) => iencli,
                None => return,
            };
            let devworkspace = Helper::get_api::<DevWorkspace>(client, base_info.namespace.clone());
            let workspace = match devworkspace
                .get(base_info.workspace_name.clone().unwrap().as_str())
                .await
            {
                Ok(workspace) => workspace,
                Err(err) => {
                    tracing::error!("Could not get workspace: {:?}", err);
                    return;
                }
            };
            let id = workspace.status.unwrap().devworkspace_id;
            base_info.workspace_id = Some(id);
        }

        match format {
            Some(Format::Json) => {
                println!("{}", serde_json::to_string_pretty(&base_info).unwrap());
            }
            Some(Format::Yaml) => {
                println!("{}", serde_yaml::to_string(&base_info).unwrap());
            }
            None => {
                println!("Namespace: {:?}", base_info.namespace);
                println!("Workspace name: {:?}", base_info.workspace_name);
                println!("Workspace id: {:?}", base_info.workspace_id);
                println!("Podname: {:?}", base_info.podname);
                println!("Is in pod: {:?}", base_info.is_in_pod);
            }
        }
    }
}
