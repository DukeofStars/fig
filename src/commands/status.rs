use clap::Args;

use fig::repository::{Error, RepositoryBuilder};

use crate::{commands::namespace, commands::namespace::NamespaceOptions};

#[derive(Debug, Args)]
pub struct StatusOptions {}

pub fn status(repo_builder: RepositoryBuilder, _options: &StatusOptions) -> color_eyre::Result<()> {
    println!("=== Fig repository health check ===");

    let repository = match repo_builder.open() {
        Ok(repository) => repository,
        Err(Error::NotInitialised) => {
            println!("initialised: false");
            return Ok(());
        }
        Err(err) => Err(err)?,
    };

    println!("initialised: true");
    println!("location: {}", repository.path().display());

    let floating_namespaces = repository.floating_namespaces()?;

    println!("== Namespaces {} ==", repository.namespaces()?.len());
    let repo_builder = repository.into_builder();
    namespace::namespace_cli(
        repo_builder,
        &NamespaceOptions {
            subcommand: namespace::Command::List { json: false },
        },
    )?;

    if !floating_namespaces.is_empty() {
        println!("== Floating namespaces {} ==", floating_namespaces.len());
        for ns in floating_namespaces {
            println!("{ns}");
        }
    }

    Ok(())
}
