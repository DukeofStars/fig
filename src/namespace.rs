use clap::{Args, Subcommand};
use miette::*;

use crate::repository::Repository;

#[derive(Args)]
pub struct NamespaceOptions {
    #[clap(subcommand)]
    subcommand: Command,
}
#[derive(Subcommand)]
pub enum Command {
    List {
        #[clap(long)]
        json: bool,
    },
}

pub fn namespace_cli(repository: &Repository, options: NamespaceOptions) -> Result<()> {
    match options.subcommand {
        Command::List { json } => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&repository.namespaces()?).unwrap()
                )
            } else {
                for (name, path) in repository.namespaces()? {
                    println!("{:15}: {}", name, path.display());
                }
            }
            Ok(())
        }
    }
}
