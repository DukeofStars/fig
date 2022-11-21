use clap::Parser;
use miette::Result;

use fig::{
    add::{self, AddOptions},
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
    Init(RepositoryInitOptions),
    Add(AddOptions),
}
