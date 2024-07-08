use clap::{Parser, Subcommand};
use log::LevelFilter;
use simplelog::*;
use std::env;
use std::fs::File;
use std::path::PathBuf;

mod commands;
mod session_config;

use session_config::{SessionConfig, TabConfig};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help = true)]
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
    setup_logging();
    log::info!("Starting zelp cli...");
    let cli = Cli::parse();

    /*
    match cli.debug {
        0 => {
            // Set up no logging
        }
        1 => {
            // Low log level: INFO
            tracing_subscriber::fmt()
                .with_max_level(tracing::Level::INFO)
                .init();
        }
        _ => {
            // Verbose mode: DEBUG
            tracing_subscriber::fmt()
                .with_max_level(tracing::Level::DEBUG)
                .init();
        }
    }
    */

    match &cli.command {
        Some(Commands::Init) => {
            let default_confg = SessionConfig {
                session_name: "deafult".into(),
                shell_command_before: "ls".into(),
                tabs: vec![TabConfig {
                    name: "pane1".into(),
                    focus: true,
                    commands: vec!["nano".into()],
                }],
            };
            let default_config_path = PathBuf::from(".zelp.ron");
            session_config::save_config(&default_config_path, &default_confg)
                .unwrap();
        }
        Some(Commands::Load { config_path }) => {
            let default_config_path = PathBuf::from(".zelp.ron");
            let path = match config_path {
                Some(path) => path.as_path(),
                None => default_config_path.as_path(),
            };
            log::info!("Loading config: {:?}", path);
            let conf = SessionConfig::new(path);
            log::info!("Config: {:?}", &conf);
            commands::start_session(&conf);
        }
        Some(Commands::Check { config_path }) => {
            let default_config_path = PathBuf::from(".zelp.ron");
            let path = match config_path {
                Some(path) => path.as_path(),
                None => default_config_path.as_path(),
            };
            let conf = SessionConfig::new(path);
            log::info!("Config: {:?}", &conf);
        }
        None => {}
    }
}

fn setup_logging() {
    let home_dir = env::var("HOME").expect("Unable to get HOME directory");
    let log_path = PathBuf::from(home_dir).join(".zelp.log");

    let file = File::create(log_path).unwrap();

    let config = ConfigBuilder::new().build();

    WriteLogger::init(LevelFilter::Info, config, file)
        .expect("Failed to initialize logger");
}
