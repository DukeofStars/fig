use std::path::PathBuf;

use clap::Args;
use color_eyre::{eyre::Context, Result};
use serde::{Deserialize, Serialize};

use crate::repository::Error;
use crate::{namespace::Namespace, repository, repository::RepositoryBuilder};

#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub initialised: bool,
    pub namespaces: Vec<Namespace>,
    pub floating_namespaces: Vec<String>,
    pub repository_path: PathBuf,
    pub log_path: PathBuf,
}

impl Info {
    pub fn gather(repo_builder: RepositoryBuilder) -> Result<Self, repository::Error> {
        let repository_path = repo_builder.path().clone();
        let log_path = crate::project_dirs().data_local_dir().join("fig-log.txt");
        match repo_builder.open() {
            Ok(repository) => Ok(Self {
                initialised: true,
                namespaces: repository.namespaces()?,
                floating_namespaces: repository.floating_namespaces()?,
                repository_path,
                log_path,
            }),
            Err(Error::NotInitialised) => Ok(Self {
                initialised: false,
                namespaces: vec![],
                floating_namespaces: vec![],
                repository_path,
                log_path,
            }),
            Err(e) => Err(e),
        }
    }
}

/// Get information about the configuration repository.
#[derive(Debug, Args)]
pub struct InfoOptions {
    #[clap(long)]
    json: bool,
}

pub fn info(repo_builder: RepositoryBuilder, options: &InfoOptions) -> Result<()> {
    let info = Info::gather(repo_builder)?;

    if options.json {
        let json = serde_json::to_string_pretty(&info).context("Failed to serialize Info")?;
        println!("{json}");
        return Ok(());
    }

    println!("=== Fig repository information ===");

    println!("initialised: {}", info.initialised);
    println!("location: {}", info.repository_path.display());
    println!("log file: {}", info.log_path.display());

    println!();

    println!("== Namespaces {} ==", info.namespaces.len());
    for namespace in &info.namespaces {
        let file_name = namespace.location.file_name().unwrap().to_str().unwrap();
        println!("{}: {}", file_name, namespace.target.display());
    }

    if !info.floating_namespaces.is_empty() {
        println!(
            "== Floating namespaces {} ==",
            info.floating_namespaces.len()
        );
        for ns in info.floating_namespaces {
            println!("{ns}");
        }
    }

    Ok(())
}
