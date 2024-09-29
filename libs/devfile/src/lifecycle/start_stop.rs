use crd::dev_work_space::DevWorkspace;
use helper::Helper;
use kube::{
    api::{Patch, PatchParams},
    Client,
};
use serde_json::from_value;

#[tracing::instrument(level = "trace", skip(client))]
pub async fn start_stop_devworkspace(
    client: Client,
    workspace_name: String,
    namespace: Option<String>,
    running: bool,
) -> Option<DevWorkspace> {
    let devworkspace_api = Helper::get_api::<DevWorkspace>(client, namespace);
    let js_patch = serde_json::json!([{
        "op": "replace",
        "path":"/spec/started",
        "value": running
    }]);
    let p_patch: json_patch::Patch = from_value(js_patch).unwrap();
    let params = PatchParams::apply("dev-cli");
    let patch = Patch::Json::<()>(p_patch);
    let res = devworkspace_api
        .patch(&workspace_name.clone(), &params, &patch)
        .await;
    match res {
        Ok(ws) => {
            tracing::trace!(?ws, "ws updated");
            Some(ws)
        }
        Err(err) => {
            tracing::error!(?err, "Couldn't update ws");
            None
        }
    }
}
