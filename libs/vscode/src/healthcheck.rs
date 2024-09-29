use helper::Helper;
use http::Request;
use hyper_util::rt::TokioIo;
use k8s_openapi::api::core::v1::Pod;
use kube::Client;
use tokio::time::Duration;
use tokio_util::bytes::Bytes;

const ACTIVITY_INTERVAL_MS: u64 = 60 * 1000; // 1 minute

// Now we can open workspace in vscode through the open_code module.
// But the goal of the next part is to prevent the workspace from closing from the lack of activity.
// https://github.com/che-incubator/che-code/blob/6e0a908d58cacb380c216dde3af544d75e3913d5/build/scripts/entrypoint-volume.sh#L26
// https://github.com/kube-rs/kube/blob/main/examples/pod_portforward_hyper_http.rs#L39-L63
// https://github.com/che-incubator/che-code/blob/main/code/extensions/che-activity-tracker/src/activity-tracker-service.ts#L21
// https://github.com/eclipse/che/issues/22812

#[tracing::instrument(level = "trace", skip(client))]
pub async fn healthcheck(
    client: Client,
    pod_name: String,
    target_port: u16,
    pod_namespace: Option<String>,
) {
    let pod_api = Helper::get_api::<Pod>(client, pod_namespace);
    let http_req = Request::builder()
        .uri("/activity/tick")
        .header("Connection", "close")
        .header("Host", format!("127.0.0.1:{}", target_port).as_str())
        .method("POST")
        .body(http_body_util::Empty::<Bytes>::new())
        .unwrap();
    let mut interval = tokio::time::interval(Duration::from_millis(ACTIVITY_INTERVAL_MS));
    loop {
        interval.tick().await;
        tracing::info!("Healthcheck");
        // We need to take the stream from the port forwarder to send the request each time because the stream is closed after the request is sent.
        let mut port_forward = match pod_api.portforward(&pod_name, &[target_port]).await {
            Ok(port_forward) => port_forward,
            Err(err) => {
                tracing::error!(?err, "Could not create port forward");
                return;
            }
        };
        let port = match port_forward.take_stream(target_port) {
            Some(port) => port,
            None => {
                tracing::error!("Could not take stream: Does not exist");
                return;
            }
        };
        let (mut sender, connection) =
            match hyper::client::conn::http1::handshake(TokioIo::new(port)).await {
                Ok((sender, connection)) => (sender, connection),
                Err(err) => {
                    tracing::error!(?err, "Could not handshake");
                    return;
                }
            };
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                tracing::error!(?e, "Error in connection");
            } else {
                tracing::trace!("Connection closed");
            }
        });
        match sender.send_request(http_req.clone()).await {
            Ok(_) => {
                tracing::trace!("Request sent");
            }
            Err(err) => {
                tracing::error!(?err, "Could not send request");
                break;
            }
        };
    }
}
