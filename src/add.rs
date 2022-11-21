use std::{fs, path::PathBuf};

use clap::Args;
use miette::Diagnostic;
use miette::Result;
use thiserror::Error;

use crate::{
    namespace::{determine_namespace, strip_namespace},
    repository::Repository,
    Error::*,
};

#[derive(Error, Diagnostic, Debug)]
pub enum Error {}

#[derive(Args, Debug)]
pub struct AddOptions {
    file: PathBuf,
}

pub fn add(repository: &Repository, options: AddOptions) -> Result<()> {
    let repo_dir = Repository::dir()?;

    let file = options.file.canonicalize().map_err(IoError)?;

    let (name, path) = determine_namespace(repository, &repo_dir)?;

    let namespace_path = repo_dir.join(&name);
    let new_path = namespace_path.join(strip_namespace(path, &file).ok_or(PathConversionFail)?);
    if let Some(parent) = new_path.parent() {
        fs::create_dir_all(namespace_path.join(parent)).map_err(IoError)?;
    }

    fs::copy(&file, &namespace_path.join(new_path)).map_err(IoError)?;

    Ok(())
}
