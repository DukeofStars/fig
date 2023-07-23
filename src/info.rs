use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::namespace::Namespace;
use crate::repository;
use crate::repository::Repository;


#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub namespaces: Vec<Namespace>,
    pub repository_dir: PathBuf,
    pub log_dir: PathBuf,
}
impl Info {
    pub fn gather(repo: &Repository) -> Result<Self, repository::Error> {
        Ok(Self {
            namespaces: { repo.namespaces()? },
            repository_dir: { repo.dir.clone() },
            log_dir: { crate::project_dirs().data_local_dir().join("fig-log.txt") },
        })
    }
}