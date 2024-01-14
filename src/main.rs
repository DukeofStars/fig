use std::fs::File;

use clap::{command, Parser, Subcommand};
use color_eyre::{eyre::Context, Result};
use tracing::Level;
use tracing_subscriber::{fmt, fmt::writer::MakeWriterExt, layer::SubscriberExt, Registry};

use fig::repository::RepositoryBuilder;
pub use fig::*;

use crate::commands::{
    add::AddOptions, clone::CloneOptions, cmd::CmdOptions, deploy::DeployOptions,
    info::InfoOptions, init::InitOptions, list::ListOptions, namespace::NamespaceOptions,
};

/// A powerful and cross-platform configuration manager.
#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Add a file to the configuration repository.
    Add(AddOptions),
    /// Clone another repository.
    Clone(CloneOptions),
    /// Run a command in the configuration repository directory.
    #[command(alias = "sh")]
    Cmd(CmdOptions),
    /// Deploy files from the configuration repository to your system.
    Deploy(DeployOptions),
    /// Display information about your configuratino repository.
    #[command(alias = "status")]
    Info(InfoOptions),
    /// Initialise a configuration repository.
    Init(InitOptions),
    /// Print all files that are in the configuration repository.
    List(ListOptions),
    /// Manage your namespaces
    #[command(alias = "ns")]
    Namespace(NamespaceOptions),
    /// Completely delete your configuration repository.
    Purge,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse_from(wild::args());

    // Initialise logging
    let log_path = project_dirs().data_local_dir().join("fig-log.txt");
    if !log_path.parent().unwrap().exists() {
        std::fs::create_dir_all(log_path.parent().unwrap())
            .context("Failed to create log directory")?;
    }
    let file = File::options()
        .create(true)
        .write(true)
        .open(log_path)
        .context("Failed to open log file")?;

    let subscriber = Registry::default()
        .with(fmt::Layer::default().with_writer(file.with_max_level(Level::TRACE)))
        .with(
            fmt::Layer::default().with_writer(std::io::stderr.with_max_level(match cli.verbose {
                0 => Level::WARN,
                // -v
                1 => Level::DEBUG,
                // -vv
                2.. => Level::TRACE,
            })),
        );
    tracing::subscriber::set_global_default(subscriber)
        .context("Unable to set global subscriber")?;

    let repo_builder = RepositoryBuilder::new(project_dirs().data_dir().to_path_buf());

    match &cli.command {
        Command::Add(options) => {
            commands::add::add(repo_builder, options)?;
        }
        Command::Clone(options) => {
            commands::clone::clone(repo_builder, options)?;
        }
        Command::Cmd(options) => {
            commands::cmd::cmd(repo_builder, options)?;
        }
        Command::Deploy(options) => {
            commands::deploy::deploy(repo_builder, options)?;
        }
        Command::Info(options) => {
            commands::info::info(repo_builder, options)?;
        }
        Command::List(options) => {
            commands::list::list(repo_builder, options)?;
        }
        Command::Namespace(options) => {
            commands::namespace::namespace_cli(repo_builder, options)?;
        }
        Command::Purge => {
            commands::purge::purge(repo_builder)?;
        }
        Command::Init(options) => {
            commands::init::init(repo_builder, options)?;
        }
    }

    Ok(())
}
