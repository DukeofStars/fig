use std::{fs, path::PathBuf};

use clap::Args;
use glob::glob;
use miette::Diagnostic;
use miette::Result;
use thiserror::Error;

use crate::{determine_namespace, repository::Repository, strip_namespace, Error::*, ManyError};
use Error::*;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("An error occurred iterating glob paths")]
    #[diagnostic(code(fig::add::glob_error))]
    GlobError(#[source] glob::GlobError),
    #[error("An error occurred parsing glob strings")]
    #[diagnostic(code(fig::add::pattern_error))]
    PatternError(#[source] glob::PatternError),
    #[error(transparent)]
    SuperError(#[from] super::Error),
    #[error("The file '{}' does not exist", .0.display())]
    #[diagnostic(code(fig::add::file_doesnt_exist))]
    FileDoesntExist(PathBuf),
}

#[derive(Args, Debug)]
pub struct AddOptions {
    files: Vec<String>,
    #[clap(long)]
    mock: bool,
}

pub fn add(repository: &Repository, options: AddOptions) -> Result<()> {
    let mut files: Vec<PathBuf> = vec![];
    let mut many_error = ManyError::new();
    for file in options.files {
        for entry in glob(file.as_str()).map_err(PatternError)? {
            if let Ok(path) = entry {
                files.push(path);
            } else {
                many_error.add(entry.map_err(GlobError).unwrap_err())
            }
        }
    }

    let repo_dir = Repository::dir()?;
    for file in files {
        let file = file.canonicalize().map_err(IoError);
        if let Err(err) = file {
            many_error.add(SuperError(err));
            continue;
        }
        let file: PathBuf = file
            .unwrap()
            .to_str()
            .ok_or(PathConversionFail)?
            .trim_start_matches("\\\\?\\")
            .into();

        if !file.exists() {
            many_error.add(FileDoesntExist(file));
            continue;
        }
        let (name, path) = determine_namespace(repository, &file)?;

        let namespace_path = repo_dir.join(&name);
        let new_path = namespace_path.join(strip_namespace(path, &file).ok_or(PathConversionFail)?);
        if let Some(parent) = new_path.parent() {
            fs::create_dir_all(namespace_path.join(parent)).map_err(IoError)?;
        }

        let output_path = &namespace_path.join(new_path);
        if options.mock {
            println!("{} -> {}", file.display(), output_path.display())
        } else {
            fs::copy(&file, &output_path).map_err(IoError)?;
        }
    }

    Ok(many_error.to_result()?)
}
