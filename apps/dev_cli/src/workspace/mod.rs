use clap::{Subcommand, ValueEnum};

pub mod get;

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
    GetContainer {},
    /// List all workspaces
    List {},
    /// Start a workspace by name
    Start {},
    /// Stop a workspace by name
    Stop {},
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
        match self {
            Workspace::Get { format } => {
                self::Workspace::get(namespace, workspace_name, format.clone()).await
            }
            Workspace::GetContainer {} => {
                println!("Get container");
            }
            Workspace::List {} => {
                println!("List workspaces");
            }
            Workspace::Start {} => {
                println!("Start workspace");
            }
            Workspace::Stop {} => {
                println!("Stop workspace");
            }
            Workspace::Restart { wait } => {
                println!("Restart workspace");
            }
        }
    }
}
