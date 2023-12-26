use clap::Args;
use color_eyre::Result;
use url::Url;

use crate::repository::RepositoryBuilder;

#[derive(Debug, Args)]
pub struct CloneOptions {
    url: Url,
}

pub fn clone(repo_builder: RepositoryBuilder, options: &CloneOptions) -> Result<()> {
    // Perform initial clone.
    let repository = repo_builder.clone(options.url.as_str())?;
    println!("Repository cloned successfully");

    // Any user-made namespaces must be added manually.
    let floating_namespaces = repository.floating_namespaces()?;
    if !floating_namespaces.is_empty() {
        println!();
        println!("The following namespaces could not be auto-generated, and must be set manually.");
        println!("To do this, run `fig namespace set <namespace> <path>`");
        for ns in repository.floating_namespaces()? {
            println!("\t{ns}");
        }
    }

    Ok(())
}
