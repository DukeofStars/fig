use std::fs::File;

use clap::{command, Parser, Subcommand};
use color_eyre::{eyre::Context, Result};
use log::LevelFilter;

use fig::repository::Repository;
pub use fig::*;

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

    let repository = if let Command::Init(options) = &cli.command {
        commands::init::init(options, project_dirs().data_dir().to_path_buf())?
    } else {
        Repository::open(project_dirs().data_dir())?
    };

    match &cli.command {
        Command::Add(options) => {
            commands::add::add(&repository, options)?;
        }
        Command::Cmd(options) => {
            commands::cmd::cmd(&repository, options)?;
        }
        Command::Deploy(options) => {
            commands::deploy::deploy(&repository, options)?;
        }
        Command::Info(options) => {
            commands::info::info(&repository, options)?;
        }
        Command::List(options) => {
            commands::list::list(&repository, options)?;
        }
        Command::Namespace(options) => {
            commands::namespace::namespace_cli(&repository, options)?;
        }
        Command::Purge => {
            commands::purge::purge(&repository)?;
        }
        Command::Status(options) => {
            commands::status::status(&repository, options)?;
        }
        Command::Init(_) => {}
    }

    Ok(())
}
