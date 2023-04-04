use std::path::PathBuf;

use clap::Args;
use glob::glob;
use log::error;
use log::trace;
use miette::Diagnostic;
use miette::Result;
use thiserror::Error;

use fig::{determine_namespace, repository::Repository, strip_namespace, Error::*, ManyError};
use Error::*;

use crate::log_utils;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("An error occurred iterating glob paths")]
    #[diagnostic(code(fig::add::glob_error))]
    GlobError(#[source] glob::GlobError),
    #[error("An error occurred parsing glob strings")]
    #[diagnostic(code(fig::add::pattern_error))]
    PatternError(#[source] glob::PatternError),
    #[error(transparent)]
    SuperError(#[from] fig::Error),
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
        let path = file.as_ref().unwrap().to_str().ok_or(PathConversionFail);
        let file: PathBuf = if let Ok(path) = path {
            path.trim_start_matches("\\\\?\\").into()
        } else if let Err(err) = path {
            many_error.add(Error::SuperError(err));
            continue;
        } else {
            unreachable!()
        };

        if !file.exists() {
            many_error.add(FileDoesntExist(file));
            continue;
        }
        let (name, path) = determine_namespace(repository, &file)?;

        let namespace_path = repo_dir.join(&name).canonicalize().unwrap();
        let new_path = namespace_path.join(strip_namespace(path, &file).ok_or(PathConversionFail)?);
        if let Some(parent) = new_path.parent() {
            let parent = namespace_path.join(parent);
            if !parent.exists() {
                if let Err(err) = log_utils::create_dir_all!(&parent).map_err(IoError) {
                    many_error.add(Error::SuperError(err));
                    continue;
                }
            }
        }

        let output_path = &namespace_path.join(new_path);
        if options.mock {
            println!("{} -> {}", file.display(), output_path.display())
        } else if file.is_file() {
            log_utils::copy_file!(&file, &output_path).map_err(IoError)?;
        } else if file.is_dir() {
            log_utils::copy_dir!(&file, &output_path);
        }
    }

    Ok(many_error.to_result()?)
}
