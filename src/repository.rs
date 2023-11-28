use std::path::PathBuf;

use thiserror::Error;
use tracing::instrument;

use crate::macros::generate_wrap_error;
use crate::namespace::Namespace;
use crate::template;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Repository already initialised")]
    AlreadyInitialised,
    #[error("Repository not yet initialised")]
    NotInitialised,
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Could not open git repository")]
    GitError(#[from] git2::Error),
    #[error(transparent)]
    TemplateError(#[from] crate::template::Error),
    #[error("{}", .0)]
    OpenError(#[source] Box<Self>, Vec<Self>),

    // This must exist for generate_wrap_error!() to work.
    #[error("{}", .1)]
    Wrapped(#[source] Box<Self>, String),
}
generate_wrap_error!(Error, Wrap);

/// Create or initialise a repository.
pub enum RepositoryBuilder {
    Unopened(PathBuf),
    Opened(Repository),
}

impl RepositoryBuilder {
    pub fn path(&self) -> &PathBuf {
        match self {
            RepositoryBuilder::Unopened(path) => path,
            RepositoryBuilder::Opened(repository) => repository.path(),
        }
    }

    /// Create a new repository builder.
    pub fn new(path: PathBuf) -> RepositoryBuilder {
        RepositoryBuilder::Unopened(path)
    }

    /// Open already existing repository
    #[instrument(skip(self), fields(repository = % self.path().display()))]
    pub fn open(self) -> Result<Repository, Error> {
        match self {
            RepositoryBuilder::Unopened(path) => {
                tracing::info!("Opening repository at '{}'", path.display());
                if !path.exists() {
                    return Err(Error::NotInitialised);
                }
                let repository = git2::Repository::open(&path);
                match repository {
                    Ok(repository) => {
                        Ok(Repository {
                            git_repository: repository,
                            path: path.to_path_buf(),
                        })
                    }
                    Err(git_error) => {
                        Err(git_error).wrap("Failed to open git repository, maybe it isn't initialised. You can initialise it with `fig cmd -- git init`")?
                    }
                }
            }
            RepositoryBuilder::Opened(repository) => Ok(repository),
        }
    }
    #[instrument(skip(self), fields(repository = % self.path().display()))]
    pub fn init(self) -> Result<Repository, Error> {
        match self {
            RepositoryBuilder::Unopened(path) => {
                tracing::info!("Creating repository at '{}'", path.display());

                if path.exists() {
                    return Err(Error::AlreadyInitialised);
                }

                crate::create_dir_all!(&path)
                    .wrap(format!("Failed to create directory '{}'", path.display()))?;

                template::generate(&path)?;

                let dot_gitignore = "namespace.fig";
                let dot_gitignore_path = path.join(".gitignore");
                std::fs::write(&dot_gitignore_path, dot_gitignore).wrap(format!(
                    "Failed to write to {}",
                    dot_gitignore_path.display()
                ))?;

                // Initialise git
                let git_repository = git2::Repository::init(&path)?;

                Ok(Repository {
                    git_repository,
                    path,
                })
            }
            RepositoryBuilder::Opened(_) => Err(Error::AlreadyInitialised),
        }
    }

    pub fn clone(self, url: &str) -> Result<Repository, Error> {
        match self {
            RepositoryBuilder::Unopened(path) => {
                let git_repository = git2::Repository::clone_recurse(url, &path)?;
                // Fill in namespaces
                template::generate(&path)?;
                let repository = Repository {
                    git_repository,
                    path,
                };

                Ok(repository)
            }
            RepositoryBuilder::Opened(_) => Err(Error::AlreadyInitialised)
                .wrap("Cannot clone into repository that already exists"),
        }
    }
}

pub struct Repository {
    git_repository: git2::Repository,
    path: PathBuf,
}

impl Repository {
    pub fn into_builder(self) -> RepositoryBuilder {
        RepositoryBuilder::Opened(self)
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Returns list of namespaces and the paths they point to.
    pub fn namespaces(&self) -> Result<Vec<Namespace>, Error> {
        let mut out = vec![];
        for entry in self.path.read_dir().wrap("Failed to read directory")? {
            let entry = entry?;
            if entry.file_type()?.is_dir() && entry.path().join("namespace.fig").exists() {
                let path = std::fs::read_to_string(entry.path().join("namespace.fig"))?;
                let namespace = Namespace {
                    target: PathBuf::from(path),
                    location: entry.path(),
                };
                out.push(namespace);
            }
        }
        Ok(out)
    }

    /// List of all directories in repository that do not have a namespace.fig file.
    pub fn floating_namespaces(&self) -> Result<Vec<String>, Error> {
        let mut floating_namespaces = Vec::new();
        for entry in self.path().read_dir()?.flatten() {
            if entry.file_type()?.is_dir() && !entry.path().join("namespace.fig").exists() {
                let file_name = entry.file_name();
                // Ignore .git
                if file_name == ".git" {
                    continue;
                }
                floating_namespaces.push(file_name.to_str().unwrap().to_string());
            }
        }
        Ok(floating_namespaces)
    }

    pub fn push(&self) -> Result<(), Error> {
        self.git_repository
            .find_remote("origin")?
            .push(&["master"], None)?;
        Ok(())
    }
}
