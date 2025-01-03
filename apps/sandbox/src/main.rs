use std::collections::HashMap;

use devfile::lifecycle::find_pod_by_ws_name::find_pod_by_ws_name;
use helper::Helper;
use tokio::io::AsyncReadExt;
use tool_tracing::{
    init::init_tracing,
    level::VerboseLevel,
    tracing_kind::{Tracing, TracingKind},
};

use k8s_openapi::{api::core::v1::Pod, NamespaceResourceScope};
use kube::{api::AttachParams, client, Api, Client, Resource};

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
    let pods: Api<Pod> = Api::namespaced(kube_client, &namespace);
    let mut attach = AttachParams::interactive_tty().stderr(false).stdin(true);
    attach.container = Some("tools".to_string());
    let mut result = match pods.exec(&podname, vec!["env"], &attach).await {
        Ok(result) => result,
        Err(err) => match err {
            kube::Error::UpgradeConnection(err) => {
                tracing::error!("Could not exec into pod (ws error) : {:?}", err.to_string());
                return;
            }
            _ => {
                tracing::error!("Could not exec into pod : {:?}", err);
                return;
            }
        },
    };
    let mut stdout = result.stdout().unwrap();
    let mut buff = String::new();
    stdout.read_to_string(&mut buff).await.unwrap();
    tracing::trace!("Envvars : {:?}", buff);
    if buff.is_empty() || buff == "\n" {
        return;
    }
    let envvars: HashMap<String, String> = buff
        .split("\n")
        .map(|line| {
            let mut parts = line.split("=");
            let key = parts.next().unwrap().to_string();
            let value = parts.next().unwrap_or("").to_string();
            (key, value)
        })
        .collect();
    println!("{:?}", envvars);
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
