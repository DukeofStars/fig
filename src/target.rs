use std::path::{Path, PathBuf};

use miette::Diagnostic;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("No namespace matches given path")]
    #[diagnostic(code(fig::target::no_namespace_matches))]
    NoNamespaceMatches,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Target {
    namespace: PathBuf,
    repo_path: PathBuf,
}

impl Target {
    pub fn new(namespace: impl AsRef<Path>, repo_path: impl AsRef<Path>) -> Self {
        Self {
            namespace: namespace.as_ref().to_path_buf(),
            repo_path: repo_path.as_ref().to_path_buf(),
        }
    }
}
