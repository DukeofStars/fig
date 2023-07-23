use clap::{command, Parser, Subcommand};
use color_eyre::Result;

pub use fig::*;
use fig::repository::Repository;

use crate::{
    commands::add::AddOptions, commands::cmd::CmdOptions, commands::deploy::DeployOptions,
    commands::info::InfoOptions, commands::init::InitOptions, commands::list::ListOptions,
    commands::namespace::NamespaceOptions,
};

mod commands;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Add(AddOptions),
    Cmd(CmdOptions),
    Deploy(DeployOptions),
    Info(InfoOptions),
    Init(InitOptions),
    List(ListOptions),
    Namespace(NamespaceOptions),
    Purge,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse_from(wild::args());

    let repository = if let Command::Init(options) = &cli.command {
        commands::init::init(options, project_dirs().data_dir().to_path_buf())?
    } else {
        Repository::open(project_dirs().data_dir().to_path_buf())?
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
        Command::Init(_) => {}
    }

    Ok(())
}
