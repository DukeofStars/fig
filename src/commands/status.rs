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

    // Namespaces that do not have a target.
    let mut floating_namespaces = Vec::new();
    for entry in repository.path().read_dir()?.flatten() {
        if entry.file_type()?.is_dir() && !entry.path().join("namespace.fig").exists() {
            let file_name = entry.file_name();
            floating_namespaces.push(file_name.to_str().unwrap().to_string());
        }
    }

    println!("== Namespaces {} ==", repository.namespaces()?.len());
    let repo_builder = repository.into_builder();
    namespace::namespace_cli(
        repo_builder,
        &NamespaceOptions {
            subcommand: namespace::Command::List { json: false },
        },
    )?;

    println!("== Floating namespaces {} ==", floating_namespaces.len());
    for ns in floating_namespaces {
        println!("{ns}");
    }

    Ok(())
}
