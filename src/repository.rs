use std::path::PathBuf;

use color_eyre::{
    eyre::{bail, Context},
    Result,
};
use tracing::{debug, error, info, instrument, warn};

use crate::{
    namespace::Namespace,
    plugin::{self, PluginTriggerLookup},
    template,
};

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
    pub fn open(self) -> Result<Repository> {
        info!("Opening repository");
        match self {
            RepositoryBuilder::Unopened(path) => {
                if !path.exists() {
                    error!("Repository not initialised");
                    bail!("Repository not initialised");
                }
                let repository = git2::Repository::open(&path);
                debug!("Opening git repository");
                match repository {
                    Ok(repository) => Ok(Repository {
                        git_repository: repository,
                        path: path.clone(),
                    }),
                    Err(_) => {
                        error!("Failed to open git repository, maybe it isn't initialised.");
                        bail!("Failed to open git repository, maybe it isn't initialised. Try re-initialising it with `fig cmd -- git init`")
                    }
                }
            }
            RepositoryBuilder::Opened(repository) => Ok(repository),
        }
    }
    #[instrument(skip(self), fields(repository = % self.path().display()))]
    pub fn init(self) -> Result<Repository> {
        info!("Creating repository");
        match self {
            RepositoryBuilder::Unopened(path) => {
                if path.exists() {
                    warn!("Repository is already initialised");
                    bail!("Repository already initialised");
                }

                crate::create_dir_all!(&path)
                    .wrap_err(format!("Failed to create directory '{}'", path.display()))?;

                template::generate(&path)?;

                let dot_gitignore = "namespace.fig";
                let dot_gitignore_path = path.join(".gitignore");
                std::fs::write(&dot_gitignore_path, dot_gitignore).wrap_err(format!(
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
            RepositoryBuilder::Opened(_) => {
                error!("Repository already initialised");
                bail!("Repository already initialised");
            }
        }
    }

    #[instrument(skip(self))]
    pub fn clone(self, url: &str) -> Result<Repository> {
        info!("Cloning repository");
        match self {
            RepositoryBuilder::Unopened(path) => {
                let git_repository = git2::Repository::clone_recurse(url, &path)
                    .wrap_err("Failed to clone git repository")?;

                // Fill in namespaces
                debug!("Generating default namespaces");
                template::generate(&path).wrap_err("Failed to populate namespaces")?;
                let repository = Repository {
                    git_repository,
                    path,
                };

                Ok(repository)
            }
            RepositoryBuilder::Opened(_) => {
                error!("Repository already initialised");
                bail!("Cannot clone into repository that already exists")
            }
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
    pub fn namespaces(&self) -> Result<Vec<Namespace>> {
        let mut out = vec![];
        for entry in self.path.read_dir().wrap_err("Failed to read directory")? {
            let entry = entry?;
            if entry.file_type()?.is_dir() && entry.path().join("namespace.fig").exists() {
                let text = std::fs::read_to_string(entry.path().join("namespace.fig"))?;
                let targets = text
                    .lines()
                    .into_iter()
                    .map(str::trim)
                    .filter(|l| !l.is_empty())
                    .map(PathBuf::from)
                    .collect();
                let namespace = Namespace {
                    targets,
                    location: entry.path(),
                };
                out.push(namespace);
            }
        }
        Ok(out)
    }

    /// List of all directories in repository that do not have a namespace.fig file.
    pub fn floating_namespaces(&self) -> Result<Vec<String>> {
        let mut floating_namespaces = Vec::new();
        for entry in self.path().read_dir()?.flatten() {
            if entry.file_type()?.is_dir() && !entry.path().join("namespace.fig").exists() {
                let file_name = entry.file_name().to_str().unwrap().to_string();
                // Ignore hidden directories (e.g. ".git")
                if file_name.starts_with(".") {
                    continue;
                }
                floating_namespaces.push(file_name);
            }
        }
        Ok(floating_namespaces)
    }

    pub fn push(&self) -> Result<()> {
        info!("Pushing repository to origin");
        self.git_repository
            .find_remote("origin")?
            .push(&["master"], None)?;
        Ok(())
    }

    pub fn load_plugins(&self) -> Result<PluginTriggerLookup> {
        let path = self.path().join("plugins.toml");
        if !path.exists() {
            return Ok(PluginTriggerLookup::default());
        }

        plugin::load_plugins(path).wrap_err("Failed to load plugins")
    }
}
