use k8s_openapi::api::core::v1::Pod;
use kube::api::AttachedProcess;
use kube::{api::AttachParams, Api};

#[tracing::instrument(level = "trace")]
pub async fn create_attach_process(
    pod_name: String,
    attach_param: AttachParams,
    start_shell: String,
    pod_api: Api<Pod>,
) -> Result<AttachedProcess, ()> {
    match pod_api
        .exec(&pod_name, vec![start_shell], &attach_param)
        .await
    {
        Ok(attached_process) => {
            tracing::trace!("Success createing remote process");
            Ok(attached_process)
        }
        Err(err) => {
            tracing::error!(?err, "Error while creating remote process");
            Err(())
        }
    }
}
