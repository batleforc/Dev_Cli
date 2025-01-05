use crd::dev_work_space::DevWorkspace;
use devfile::lifecycle::{start_stop::start_stop_devworkspace, wait_for_status::wait_for_status};
use helper::{base_info::BaseInfo, Helper};

use super::Workspace;

impl Workspace {
    #[tracing::instrument(level = "trace")]
    pub async fn start(base_info: BaseInfo, wait: bool) {
        tracing::info!("Starting workspace");
        let client = match Helper::get_client().await {
            Some(iencli) => iencli,
            None => return,
        };
        if start_stop_devworkspace(
            client.clone(),
            base_info.workspace_name.clone().unwrap(),
            base_info.namespace.clone(),
            true,
        )
        .await
        .is_some()
        {
            tracing::info!("Workspace starting");
        } else {
            return;
        }
        let devworkspace_api =
            Helper::get_api::<DevWorkspace>(client.clone(), base_info.namespace.clone());
        if wait
            && wait_for_status(
                devworkspace_api.clone(),
                base_info.workspace_name.clone().unwrap(),
                "Running".to_string(),
                2000,
                150, // Fail after 5 minutes
            )
            .await
            .is_some()
        {
            tracing::info!("Workspace started");
        } else if wait {
            tracing::error!("Workspace did not start");
        } else {
            tracing::info!("Not waiting for workspace to start");
        }
    }
}
