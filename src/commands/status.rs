use clap::Args;

use crate::{commands::namespace, commands::namespace::NamespaceOptions, repository::Repository};

#[derive(Debug, Args)]
pub struct StatusOptions {}

pub fn status(repository: &Repository, _options: &StatusOptions) -> color_eyre::Result<()> {
    // Namespaces that do not have a target.
    let mut floating_namespaces = Vec::new();
    for entry in repository.dir.read_dir()?.flatten() {
        if entry.file_type()?.is_dir() && !entry.path().join("namespace.fig").exists() {
            let file_name = entry.file_name();
            floating_namespaces.push(file_name.to_str().unwrap().to_string());
        }
    }

    println!("=== Fig repository health check ===");
    println!("location: {}", repository.dir.display());

    println!("== Namespaces {} ==", repository.namespaces()?.len());
    namespace::namespace_cli(
        repository,
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
