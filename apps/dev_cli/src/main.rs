use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use level::VerboseSwapLevel;
use tool_tracing::{init, level::VerboseLevel, tracing_kind::Tracing};

pub mod code;
pub mod level;
pub mod workspace;

#[derive(Parser)]
#[command(
    name = "DevCli",
    about = "A simple cli to ease the dev process with EclipseChe/Devspaces",
    long_about = None,
    version,
    arg_required_else_help = true
)]
struct Cli {
    /// Set log level
    #[arg(short, long, global = true, value_enum)]
    verbose: Option<VerboseSwapLevel>,

    /// Enable trace logging, push trace to trace.log in a json format
    #[arg(short, long, global = true)]
    trace: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Workspaces {
        /// The namespace where your workspace is
        #[arg(short, long, global = true)]
        namespace: Option<String>,
        /// The name of the workspace
        #[arg(short, long, global = true)]
        workspace_name: Option<String>,

        #[command(subcommand)]
        workspace: Option<workspace::Workspace>,
    },
    Code {
        #[command(subcommand)]
        code: Option<code::Code>,
    },
    /// Generate completion scripts for your shell
    Completion {
        #[arg(short, long)]
        shell: Shell,
    },
}

#[tokio::main]
async fn main() {
    println!(include_str!("dev-cli.art"));
    let cli = Cli::parse();
    let debug_level = match cli.verbose {
        Some(level) => {
            if level == VerboseSwapLevel::TRACE {
                println!("Verbose is set to TRACE");
            }
            let level: VerboseLevel = level.into();
            println!("Verbose is set to {:?}", level);
            level
        }
        _ => VerboseLevel::INFO,
    };

    init::init_tracing(
        vec![Tracing {
            kind: tool_tracing::tracing_kind::TracingKind::Console,
            level: debug_level,
            additional: Default::default(),
            name: "DevCli".to_string(),
        }],
        "DevCli".to_string(),
    );
    match &cli.command {
        Some(Commands::Workspaces {
            namespace,
            workspace_name,
            workspace,
        }) => {
            workspace
                .as_ref()
                .unwrap()
                .run(namespace.clone(), workspace_name.clone())
                .await;
        }
        Some(Commands::Code { code }) => {
            code.as_ref().unwrap().run().await;
        }
        Some(Commands::Completion { shell }) => {
            let mut cmd = Cli::command();
            generate(*shell, &mut cmd, "dev_cli", &mut std::io::stdout());
        }
        None => tracing::info!("No command provided"),
    };
}
