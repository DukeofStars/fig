use std::path::PathBuf;

use clap::Args;
use fig::repository::Repository;
use miette::Result;
use serde::{Deserialize, Serialize};

#[derive(Args)]
pub struct InfoOptions {
    #[clap(long)]
    repo_dir: bool,
    #[clap(long)]
    log_dir: bool,
    #[clap(long)]
    json: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Info {
    namespaces: Vec<(String, PathBuf)>,
    repository_dir: PathBuf,
    log_dir: PathBuf,
}
impl Info {
    pub fn gather(repo: &Repository) -> Result<Self> {
        Ok(Self {
            namespaces: { repo.namespaces()?.into_iter().collect() },
            repository_dir: { repo.dir.clone() },
            log_dir: { fig::project_dirs()?.data_local_dir().join("fig-log.txt") },
        })
    }
}

pub fn info(repo: &Repository, options: InfoOptions) -> Result<()> {
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
