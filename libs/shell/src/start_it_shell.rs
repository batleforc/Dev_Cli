use super::{create_attach_process::create_attach_process, shell_size::handle_terminal_size};
use futures::StreamExt;
use k8s_openapi::api::core::v1::Pod;
use kube::{api::AttachParams, Api};
use tokio::{io::AsyncWriteExt, select};

#[tracing::instrument(level = "trace")]
pub async fn start_it_shell(
    pod_name: String,
    container_name: String,
    start_shell: String,
    pod_api: Api<Pod>,
) -> anyhow::Result<()> {
    let mut attach_param = AttachParams::interactive_tty();
    attach_param.container = Some(container_name);
    let mut process =
        match create_attach_process(pod_name, attach_param, start_shell, pod_api).await {
            Ok(process) => process,
            Err(_) => return Ok(()),
        };
    crossterm::terminal::enable_raw_mode()?;
    let mut stdin = tokio_util::io::ReaderStream::new(tokio::io::stdin());
    let mut stdout = tokio::io::stdout();

    let mut output = tokio_util::io::ReaderStream::new(process.stdout().unwrap());
    let mut input = process.stdin().unwrap();

    let term_tx = process.terminal_size().unwrap();

    let mut handle_terminal_size_handle = tokio::spawn(handle_terminal_size(term_tx));
    loop {
        select! {
            message = stdin.next() => {
                match message {
                    Some(Ok(message)) => {
                        input.write_all(&message).await?;
                    }
                    _ => {
                        break;
                    },
                }
            },
            message = output.next() => {
                match message {
                    Some(Ok(message)) => {
                        stdout.write_all(&message).await?;
                        stdout.flush().await?;
                    },
                    _ => {
                        break
                    },
                }
            },
            result = &mut handle_terminal_size_handle => {
                match result {
                    Ok(status) => {
                        tracing::info!(?status,"End of terminal size stream");
                    },
                    Err(e) => {
                        tracing::error!(?e,"Error while handling terminal size");
                    }
                }
            },
        };
    }
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
