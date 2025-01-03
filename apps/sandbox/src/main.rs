use devfile::lifecycle::find_pod_by_ws_name::find_pod_by_ws_name;
use helper::Helper;
use tool_tracing::{
    init::init_tracing,
    level::VerboseLevel,
    tracing_kind::{Tracing, TracingKind},
};

#[tokio::main]
async fn main() {
    init_tracing(
        vec![Tracing {
            kind: TracingKind::Console,
            level: VerboseLevel::DEBUG,
            additional: Default::default(),
            name: "test2".to_string(),
        }],
        "Dev-Cli-Sandbox".to_string(),
    );
    let namespace = "dev-ws-max-k2l7cd".to_string();
    let ws_name = "weebodevimage".to_string();
    let kube_client = Helper::get_client().await.unwrap();
    let pod_obj = find_pod_by_ws_name(
        kube_client.clone(),
        Some(ws_name.clone()),
        Some(namespace.clone()),
    )
    .await
    .unwrap();

    let podname = pod_obj.metadata.name.clone().unwrap();
    let env_vars = Helper::get_pod_envvars(kube_client, namespace, podname, "tools".to_string())
        .await
        .unwrap();
    println!("{:?}", env_vars);
    // let pod_env_var = Helper::get_pod_envvars(
    //     kube_client,
    //     pod_name.metadata.namespace.unwrap(),
    //     pod_name.metadata.name.unwrap(),
    //     Some("tool".to_string()),
    // )
    // .await
    // .unwrap();
    // println!("{:?}", pod_env_var);
}
