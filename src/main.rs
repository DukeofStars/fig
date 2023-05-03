mod commands;
mod log_utils;

use std::{fs::OpenOptions, panic, process};

use clap::Parser;
use log::LevelFilter;
use miette::Result;

use commands::{
    add::{self, AddOptions},
    cmd::{self, CmdOptions},
    deploy::{self, DeployOptions},
    list::{self, ListOptions},
    namespace::{namespace_cli, NamespaceOptions},
    purge::purge,
};
use fig::repository::{Repository, RepositoryInitOptions};
use simplelog::{Config, WriteLogger};

fn main() -> Result<()> {
    // Initialise logger
    let log_file = fig::project_dirs()?.data_local_dir().join("fig-log.txt");

    let log_file_parent = log_file
        .parent()
        .expect("Log file path doesn't have parent");

    if log_file.exists() {
        std::fs::remove_file(&log_file).expect("Failed to remove old log file");
    }

    if !log_file_parent.exists() {
        log_utils::create_dir_all!(&log_file_parent)
            .expect("Failed to create directory for log file");
    }

    let _ = WriteLogger::init(
        LevelFilter::Trace,
        Config::default(),
        OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(&log_file)
            .expect("Failed to create log file"),
    );

    // Set up custom panic hook
    let default_hook = panic::take_hook();

    let log_file_cloned = log_file.clone();

    panic::set_hook(Box::new(move |panic_info| {
        default_hook(panic_info);
        eprintln!(
            "note: log files can be found at {log_file}",
            log_file = log_file.display()
        );
        process::exit(1);
    }));

    let cli = Cli::parse();
    let result = run(cli);

    if result.is_err() {
        eprintln!(
            "note: log files can be found at {log_file}",
            log_file = log_file_cloned.display()
        );
    }

    result
}

fn run(cli: Cli) -> Result<()> {
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
            purge(&repository)?;
        }
        Command::List(options) => {
            let repository = Repository::open()?;
            list::list(&repository, options)?;
        }
        Command::Cmd(options) => {
            let repository = Repository::open()?;
            cmd::cmd(&repository, options)?;
        }
        Command::Namespace(options) => {
            let repository = Repository::open()?;
            namespace_cli(&repository, options)?;
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
    /// Configure namespaces
    #[clap(alias = "ns")]
    Namespace(NamespaceOptions),
}
