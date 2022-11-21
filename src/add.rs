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

    dbg!(&name, &path, &repo_dir, &file);

    let namespace_path = repo_dir.join(&name);
    dbg!(&namespace_path);
    let new_path = namespace_path.join(strip_namespace(path, &file).ok_or(PathConversionFail)?);

    dbg!(&new_path);
    fs::copy(&file, &namespace_path.join(new_path)).map_err(IoError)?;

    Ok(())
}
