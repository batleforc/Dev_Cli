use clap::{Subcommand, ValueEnum};
use helper::Helper;

pub mod get;
pub mod get_container;
pub mod list;
pub mod restart;
pub mod start;
pub mod stop;

#[derive(Debug, Clone, ValueEnum)]
pub enum Format {
    Json,
    Yaml,
}

#[derive(Subcommand, Debug)]
#[command(
    name = "Workspace",
    about = "Handle the workspace subcommand",
    arg_required_else_help = true
)]
pub enum Workspace {
    /// Get the status of a workspace, if no info provided it will get the status of the current workspace if in one
    Get {
        /// The format to output the data
        #[arg(short, long)]
        format: Option<Format>,
    },
    /// Get the container of a workspace
    GetContainer {
        /// The format to output the data
        #[arg(short, long)]
        format: Option<Format>,
    },
    /// List all workspaces
    List {
        /// The format to output the data
        #[arg(short, long)]
        format: Option<Format>,
    },
    /// Start a workspace by name
    Start {
        /// Wait for the workspace to be started
        #[arg(long)]
        wait: bool,
    },
    /// Stop a workspace by name
    Stop {
        /// Wait for the workspace to be stopped
        #[arg(long)]
        wait: bool,
    },
    /// Restart a workspace by name
    Restart {
        /// Wait for the workspace to be started
        #[arg(long)]
        wait: bool,
    },
}

impl Workspace {
    /// Run the subcommand
    pub async fn run(&self, namespace: Option<String>, workspace_name: Option<String>) {
        let base_info = Helper::get_base_info(namespace, workspace_name).await;
        let info = match base_info {
            Ok(base_info) => base_info,
            Err(err) => {
                tracing::error!("Could not get base info : {:?}", err);
                return;
            }
        };
        match self {
            Workspace::Get { format } => self::Workspace::get(info, format.clone()).await,
            Workspace::GetContainer { format } => {
                self::Workspace::get_container(info, format.clone()).await;
            }
            Workspace::List { format } => {
                self::Workspace::list(info, format.clone()).await;
            }
            Workspace::Start { wait } => {
                self::Workspace::start(info, *wait).await;
            }
            Workspace::Stop { wait } => {
                self::Workspace::stop(info, *wait).await;
            }
            Workspace::Restart { wait } => {
                self::Workspace::restart(*wait, info).await;
            }
        }
    }
}
