use crd::dev_work_space::DevWorkspace;
use helper::Helper;
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
    pub async fn get(
        namespace: Option<String>,
        workspace_name: Option<String>,
        format: Option<Format>,
    ) {
        if namespace.is_none() && workspace_name.is_none() && !Helper::is_in_a_container() {
            tracing::error!("No namespace or workspace name provided and not in a workspace");
            return;
        }
        let output = if namespace.is_none() && workspace_name.is_none() {
            tracing::trace!("No namespace or workspace name provided, using current workspace");
            GetOutput {
                namespace: Helper::get_namespace_from_env(),
                workspace_name: Helper::get_workspace_from_env(),
                workspace_id: Helper::get_workspace_id_from_env(),
                podname: Helper::get_podname_from_env(),
                is_in_pod: Helper::is_in_a_container(),
            }
            // return workspace
        } else {
            // return workspace
            let client = match Helper::get_client().await {
                Some(iencli) => iencli,
                None => return,
            };
            let devworkspace = Helper::get_api::<DevWorkspace>(client, namespace.clone());
            let workspace = match devworkspace
                .get(workspace_name.clone().unwrap().as_str())
                .await
            {
                Ok(workspace) => workspace,
                Err(err) => {
                    tracing::error!("Could not get workspace: {:?}", err);
                    return;
                }
            };
            let id = workspace.status.unwrap().devworkspace_id;
            GetOutput {
                namespace,
                workspace_name,
                workspace_id: Some(id),
                podname: None,
                is_in_pod: false,
            }
        };

        match format {
            Some(Format::Json) => {
                println!("{}", serde_json::to_string_pretty(&output).unwrap());
            }
            Some(Format::Yaml) => {
                println!("{}", serde_yaml::to_string(&output).unwrap());
            }
            None => {
                println!("Namespace: {:?}", output.namespace);
                println!("Workspace name: {:?}", output.workspace_name);
                println!("Workspace id: {:?}", output.workspace_id);
                println!("Podname: {:?}", output.podname);
                println!("Is in pod: {:?}", output.is_in_pod);
            }
        }
    }
}
