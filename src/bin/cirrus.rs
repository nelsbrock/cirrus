use cirrus::config::Config;
use cirrus::database::Database;
use clap::{Parser, Subcommand};
use log::error;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(clap::Parser)]
#[command(author, version, about)]
struct Args {
    /// Use the specified configuration file.
    ///
    /// If this is not specified, cirrus will look for the configuration file in various places,
    /// depending on the OS.
    #[arg(long, value_parser, value_name = "PATH")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run the cirrus server.
    Run,
    /// Create the configuration file.
    CreateConfig {
        #[arg(long)]
        path: PathBuf,

        /// Overwrite the configuration file if it already exists
        #[arg(long)]
        overwrite: Option<bool>,
    },
}

async fn run(config: Config, database: Database) -> ExitCode {
    todo!()
}

async fn create_config(path: PathBuf, overwrite: Option<bool>) -> ExitCode {
    match Config::create(&path, overwrite) {
        Err(err) => {
            error!("{err}");
            ExitCode::FAILURE
        }
        Ok(()) => ExitCode::SUCCESS,
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    env_logger::init();

    let args = Args::parse();

    match args.command {
        Command::Run => {
            let config = match Config::parse(args.config.as_deref()) {
                Err(err) => {
                    error!("{err}");
                    return ExitCode::FAILURE;
                }
                Ok(config) => config,
            };

            let database = match Database::connect(config.database().url()) {
                Err(err) => {
                    error!("{err}");
                    return ExitCode::FAILURE;
                }
                Ok(database) => database,
            };

            run(config, database).await
        }
        Command::CreateConfig { path, overwrite } => create_config(path, overwrite).await,
    }
}
