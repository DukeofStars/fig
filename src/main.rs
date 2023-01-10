use clap::Parser;
use miette::Result;

use fig::{
    add::{self, AddOptions},
    cmd::{self, CmdOptions},
    deploy::{self, DeployOptions},
    list::{self, ListOptions},
    purge::{self},
    repository::{Repository, RepositoryInitOptions},
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Init(options) => {
            Repository::init(options)?;
        }
        Command::Add(options) => {
            let repository = Repository::open()?;
            add::add(&repository, options)?;
        }
        Command::Deploy(options) => {
            let repository = Repository::open()?;
            deploy::deploy(&repository, options)?;
        }
        Command::Purge => {
            let repository = Repository::open()?;
            purge::purge(&repository)?;
        }
        Command::List(options) => {
            let repository = Repository::open()?;
            list::list(&repository, options)?;
        }
        // Shell
        Command::Cmd(options) => {
            let repository = Repository::open()?;
            cmd::cmd(&repository, options)?;
        }
    }

    Ok(())
}

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    /// Initialise fig, if not already initialised.
    Init(RepositoryInitOptions),
    /// Add file/s to fig repository (local).
    Add(AddOptions),
    /// Deploy your fig repository to the rest of your system.
    Deploy(DeployOptions),
    /// Completely delete repository folder.
    ///
    /// If this fails, you could corrupt your fig repository (local).
    /// Your deployed files will not be affected by this operation.
    Purge,
    /// List files in fig repository
    List(ListOptions),
    /// Run command in repository directory
    #[clap(alias = "sh", alias = "shell")]
    Cmd(CmdOptions),
}
