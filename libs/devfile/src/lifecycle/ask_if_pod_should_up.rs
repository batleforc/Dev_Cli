use std::io;

#[tracing::instrument(level = "trace")]
pub async fn ask_if_pod_should_up() -> bool {
    tracing::info!("No pod found for workspace, should i start the workspace? (y/N)");
    let mut answer = String::new();
    match io::stdin().read_line(&mut answer) {
        Ok(_) => answer.to_lowercase().contains('y'),
        Err(e) => {
            tracing::error!(?e, "Error reading input",);
            false
        }
    }
}
