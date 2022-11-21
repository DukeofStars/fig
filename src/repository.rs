use std::{collections::BTreeMap, fs, path::PathBuf};

use miette::{bail, Diagnostic, Result};
use thiserror::Error;

use self::Error::*;
use crate::{template, Error::*};

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("Repository already initialised")]
    #[diagnostic(code(fig::repository::already_initialised))]
    AlreadyInitialised,
    #[error("Repository not yet initialised")]
    #[diagnostic(code(fig::repository::not_initialised))]
    #[help("Try `fig init` to intialise")]
    NotInitialised,
}

pub struct Repository {
    git_repository: git2::Repository,
}

#[derive(clap::Args)]
pub struct RepositoryInitOptions {
    #[clap(short, long)]
    force: bool,
}

impl Repository {
    pub fn namespaces(&self) -> Result<BTreeMap<String, PathBuf>> {
        let mut out = BTreeMap::new();
        for entry in Repository::dir()?.read_dir().map_err(IoError)? {
            let entry = entry.map_err(IoError)?;
            if entry.file_type().map_err(IoError)?.is_dir()
                && entry.path().join("namespace.fig").exists()
            {
                let path =
                    fs::read_to_string(entry.path().join("namespace.fig")).map_err(IoError)?;
                out.insert(
                    String::from(
                        entry
                            .file_name()
                            .into_string()
                            .map_err(|_| PathConversionFail)?,
                    ),
                    path.into(),
                );
            }
        }
        Ok(out)
    }

    pub fn dir() -> Result<PathBuf> {
        Ok(crate::project_dirs()?.data_dir().to_path_buf())
    }

    /// Create a repository
    pub fn init(options: RepositoryInitOptions) -> Result<Self> {
        let project_dir = Repository::dir()?;

        if project_dir.exists() && !options.force {
            bail!(AlreadyInitialised)
        }

        fs::create_dir_all(&project_dir).map_err(IoError)?;

        template::generate(&project_dir)?;

        let dot_gitignore = "";
        fs::write(project_dir.join(".gitignore"), dot_gitignore).map_err(IoError)?;

        // Initialise git
        let repository = git2::Repository::init(&project_dir).map_err(GitError)?;

        Ok(Self {
            git_repository: repository,
        })
    }

    /// Open already existing repository
    pub fn open() -> Result<Self> {
        let project_dirs = crate::project_dirs()?;
        let project_dir = project_dirs.data_dir();
        let repository = git2::Repository::open(project_dir).map_err(GitError)?;
        Ok(Self {
            git_repository: repository,
        })
    }

    pub fn push(&self) -> Result<()> {
        self.git_repository
            .find_remote("origin")
            .map_err(GitError)?
            .push(&["master"], None)
            .map_err(GitError)?;
        Ok(())
    }
}
