use std::fs::File;

use clap::{command, Parser, Subcommand};
use color_eyre::{eyre::Context, Result};
use log::LevelFilter;

use fig::repository::RepositoryBuilder;
pub use fig::*;

use crate::commands::clone::CloneOptions;
use crate::commands::status::StatusOptions;
use crate::commands::{
    add::AddOptions, cmd::CmdOptions, deploy::DeployOptions, info::InfoOptions, init::InitOptions,
    list::ListOptions, namespace::NamespaceOptions,
};

mod commands;

/// A powerful and cross-platform configuration manager.
#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Add(AddOptions),
    Clone(CloneOptions),
    #[command(alias = "sh")]
    Cmd(CmdOptions),
    Deploy(DeployOptions),
    Info(InfoOptions),
    Init(InitOptions),
    List(ListOptions),
    #[command(alias = "ns")]
    Namespace(NamespaceOptions),
    /// Completely delete your configuration repository.
    Purge,
    Status(StatusOptions),
}

fn main() -> Result<()> {
    color_eyre::install()?;

    // Initialise logging
    let log_path = project_dirs().data_local_dir().join("fig-log.txt");
    let file = File::options()
        .create(true)
        .write(true)
        .open(log_path)
        .context("Failed to open log file")?;
    let mut config_builder = simplelog::ConfigBuilder::new();
    simplelog::WriteLogger::init(LevelFilter::Off, config_builder.build(), file)
        .context("Failed to initialise logger")?;

    let cli = Cli::parse_from(wild::args());

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
        Command::Status(options) => {
            commands::status::status(repo_builder, options)?;
        }
        Command::Init(options) => {
            commands::init::init(repo_builder, options)?;
        }
    }

    Ok(())
}
