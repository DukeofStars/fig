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
    #[clap(long)]
    mock: bool,
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
    
    let output_path = &namespace_path.join(new_path);
    if options.mock {
        println!("{}", output_path.display())
    }
    else {
        fs::copy(&file, &output_path).map_err(IoError)?;
    }
    
    Ok(())
}
