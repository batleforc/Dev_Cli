use clap::Subcommand;
use crd::dev_work_space::DevWorkspace;
use devfile::lifecycle::{
    ask_if_pod_should_up::ask_if_pod_should_up, find_pod_by_ws_name::find_pod_by_ws_name,
    start_stop::start_stop_devworkspace, wait_for_status::wait_for_status,
};
use helper::{select_pod::select_pod, Helper};
use vscode::{extensions::Extensions, healthcheck, open_code};

#[derive(Subcommand, Debug)]
#[command(
    name = "Code",
    about = "Handle the code subcommand",
    arg_required_else_help = true
)]
pub enum Code {
    /// Open the selected workspace in vscode
    Open {
        /// The name of the container to spawn the vscode in, by default it will use the first one
        #[arg(long)]
        name: Option<String>,

        /// Port for the healthcheck
        #[arg(long, default_value_t = 3333)]
        port: u16,

        /// Path in wich the workspace will be opened
        #[arg(long, short, default_value = "/projects/")]
        path: String,

        /// Workspace name
        #[arg(long, short)]
        workspace_name: String,

        /// The namespace where your workspace is
        #[arg(long, short)]
        namespace: Option<String>,

        /// Kubernetes context, should be imported in vscode first
        #[arg(long, short)]
        context: Option<String>,
    },
    /// Check if the needed extensions are installed
    Check {
        /// If the extensions should be installed, by default it will only check
        #[arg(long)]
        install: bool,
    },
}

impl Code {
    /// Run the subcommand
    #[tracing::instrument(level = "trace")]
    pub async fn run(&self) {
        match self {
            Code::Open {
                name,
                port,
                path,
                workspace_name,
                namespace,
                context,
            } => {
                Self::open(
                    name.clone(),
                    *port,
                    path.clone(),
                    workspace_name.clone(),
                    namespace.clone(),
                    context.clone(),
                )
                .await;
            }
            Code::Check { install } => {
                Self::check(*install).await;
            }
        }
    }

    #[tracing::instrument(level = "trace")]
    pub async fn check(install: bool) {
        // Check if the needed extensions are installed
        let extensions = Extensions::new();
        let missing_extensions = extensions.check_missing_extensions();
        match missing_extensions {
            Ok(missing_extensions) => {
                if missing_extensions.is_empty() {
                    tracing::info!("All mandatory extensions are installed");
                } else {
                    tracing::info!("Missing extensions: {:?}", missing_extensions);
                    if install {
                        for extension in missing_extensions {
                            tracing::info!("Installing extension: {}", extension);
                            match extensions.install_extension(&extension) {
                                Ok(_) => {
                                    tracing::info!("Extension {} installed", extension);
                                }
                                Err(err) => {
                                    tracing::error!("Error: {:?}", err);
                                }
                            };
                        }
                    }
                }
            }
            Err(err) => {
                tracing::error!("Error: {:?}", err);
            }
        }
    }

    #[tracing::instrument(level = "trace")]
    pub async fn open(
        name: Option<String>,
        port: u16,
        path: String,
        workspace_name: String,
        namespace: Option<String>,
        context: Option<String>,
    ) {
        tracing::info!("Opening workspace {} in vscode, please dont kill this terminal or the workspaces will close itself due to inactivity", workspace_name);
        let client = match Helper::get_client().await {
            Some(client) => client,
            None => return,
        };
        let pod = match find_pod_by_ws_name(
            client.clone(),
            Some(workspace_name.clone()),
            namespace.clone(),
        )
        .await
        {
            Some(pod) => pod,
            None => {
                if ask_if_pod_should_up().await {
                    start_stop_devworkspace(
                        client.clone(),
                        workspace_name.clone(),
                        namespace.clone(),
                        true,
                    )
                    .await;
                    if wait_for_status(
                        Helper::get_api::<DevWorkspace>(client.clone(), namespace.clone()),
                        workspace_name.clone(),
                        "Running".to_string(),
                        2000,
                        150, // Fail after 5 minutes
                    )
                    .await
                    .is_none()
                    {
                        return;
                    }
                    match find_pod_by_ws_name(
                        client.clone(),
                        Some(workspace_name.clone()),
                        namespace,
                    )
                    .await
                    {
                        Some(pod) => pod,
                        None => return,
                    }
                } else {
                    return;
                }
            }
        };
        let container_name = match select_pod(name, pod.clone()) {
            Some(container_name) => container_name,
            None => return,
        };

        let open_code = open_code::OpenCode {
            context,
            pod_name: Some(pod.metadata.name.clone().unwrap()),
            namespace: Some(pod.metadata.namespace.clone().unwrap()),
            container_name: Some(container_name),
            container_image: Some(
                pod.spec.clone().unwrap().containers[0]
                    .image
                    .clone()
                    .unwrap(),
            ),
            path: Some(path),
        };
        tracing::info!("Opening VsCode");
        open_code.open();
        tracing::info!("Healthcheck started");
        healthcheck::healthcheck(
            client,
            pod.metadata.name.unwrap(),
            port,
            pod.metadata.namespace,
        )
        .await;
    }
}
