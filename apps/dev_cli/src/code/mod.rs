use clap::Subcommand;

#[derive(Subcommand)]
#[command(
    name = "Code",
    about = "Handle the code subcommand",
    arg_required_else_help = true
)]
pub enum Code {
    /// Open the selected workspace in vscode
    Open {
        /// The name of the container to spawn the vscode in
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
    pub async fn run(&self) {}
}
