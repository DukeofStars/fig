use std::path::PathBuf;

use crate::namespace::Namespace;
use crate::repository::Repository;
use clap::Args;
use color_eyre::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Args)]
pub struct InfoOptions {
    #[clap(long)]
    repo_dir: bool,
    #[clap(long)]
    log_dir: bool,
    #[clap(long)]
    json: bool,
}

// TODO: move into another file
#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    namespaces: Vec<Namespace>,
    repository_dir: PathBuf,
    log_dir: PathBuf,
}
impl Info {
    pub fn gather(repo: &Repository) -> Result<Self> {
        Ok(Self {
            namespaces: { repo.namespaces()? },
            repository_dir: { repo.dir.clone() },
            log_dir: { crate::project_dirs().data_local_dir().join("fig-log.txt") },
        })
    }
}

pub fn info(repo: &Repository, options: &InfoOptions) -> Result<()> {
    let info = Info::gather(repo)?;

    if options.json {
        let json = serde_json::to_string_pretty(&info).expect("Failed to serialize Info");
        println!("{json}");
        return Ok(());
    }

    if options.repo_dir {
        println!("repository dir: {}", info.repository_dir.display())
    }
    if options.log_dir {
        println!("{}", info.log_dir.display())
    }

    Ok(())
}
