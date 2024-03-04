use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;
mod session_config;

use session_config::{SessionConfig, TabConfig};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Debug flag
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // Create default configuration file in current folder
    Init,
    // Load configuration
    Load { config_path: Option<PathBuf> },
    // Check configuration
    Check { config_path: Option<PathBuf> },
}

fn main() {
    let cli = Cli::parse();

    // NOTE: do i even need any logging?
    match cli.debug {
        0 => println!("setup no log"),
        1 => println!("setup low log level "),
        _ => println!("setup verbose mode"),
    }

    match &cli.command {
        Some(Commands::Init) => {
            let default_confg = SessionConfig {
                session_name: "deafult".into(),
                tabs: vec![TabConfig {
                    name: "pane1".into(),
                    focus: true,
                    commands: vec!["nano".into()],
                }],
            };
            let default_config_path = PathBuf::from(".zelp.ron");
            session_config::save_config(&default_config_path, &default_confg).unwrap();
        }
        Some(Commands::Load { config_path }) => {
            let default_config_path = PathBuf::from(".zelp.ron");
            let path = match config_path {
                Some(path) => path.as_path(),
                None => &default_config_path.as_path(),
            };
            println!("Loading config: {:?}", path);
            let conf = SessionConfig::new(path);
            println!("Config: {:?}", &conf);
            commands::start_session(&conf);
        }
        Some(Commands::Check { config_path }) => {
            let default_config_path = PathBuf::from(".zelp.ron");
            let path = match config_path {
                Some(path) => path.as_path(),
                None => &default_config_path.as_path(),
            };
            let conf = SessionConfig::new(path);
            println!("Config: {:?}", &conf);
        }
        None => {}
    }
}
