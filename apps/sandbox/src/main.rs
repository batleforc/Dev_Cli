use tool_tracing::{
    init::init_tracing,
    level::VerboseLevel,
    tracing_kind::{Tracing, TracingKind},
};
use vscode::extensions;

#[tokio::main]
async fn main() {
    init_tracing(
        vec![Tracing {
            kind: TracingKind::Console,
            level: VerboseLevel::TRACE,
            additional: Default::default(),
            name: "test2".to_string(),
        }],
        "Dev-Cli-Sandbox".to_string(),
    );
    //let namespace = "dev-ws-max-k2l7cd".to_string();
    //let ws_name = "weebodevimage".to_string();
    // test sonar 3
    let extensions = extensions::Extensions::new();
    let installed_extensions = extensions.get_installed_extensions();
    println!("{:?}", installed_extensions);

    let missing_extensions = extensions.check_missing_extensions();
    println!("{:?}", missing_extensions);

    match missing_extensions {
        Ok(missing_extensions) => {
            if missing_extensions.is_empty() {
                println!("All mandatory extensions are installed");
            } else {
                println!("Missing extensions: {:?}", missing_extensions);
                for extension in missing_extensions {
                    println!("Installing extension: {}", extension);
                    match extensions.install_extension(&extension) {
                        Ok(_) => {
                            println!("Extension {} installed", extension);
                        }
                        Err(err) => {
                            println!("Error: {:?}", err);
                        }
                    };
                }
            }
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }
}
