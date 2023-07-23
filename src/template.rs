use std::{
    fs,
    path::{Path, PathBuf},
};

use miette::{Diagnostic, Result};
use thiserror::Error;

use self::Error::*;

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("Failed to obtain base directories for your operating system")]
    #[diagnostic(code(fig::no_base_dirs))]
    NoBaseDirs,
    #[error(transparent)]
    #[diagnostic(code(fig::io_error))]
    IoError(#[from] std::io::Error),
}

pub fn generate(path: impl AsRef<Path>) -> Result<PathBuf, Error> {
    let path = path.as_ref().to_path_buf();

    let base_dirs = directories::BaseDirs::new().ok_or(NoBaseDirs)?;

    // /home
    fs::create_dir_all(path.join("home"))?;
    fs::write(
        path.join("home").join("namespace.fig"),
        base_dirs.home_dir().display().to_string(),
    )?;
    // /config
    fs::create_dir_all(path.join("config")).map_err(IoError)?;
    fs::write(
        path.join("config").join("namespace.fig"),
        base_dirs.config_dir().display().to_string(),
    )
    .map_err(IoError)?;
    // /data
    fs::create_dir_all(path.join("data")).map_err(IoError)?;
    fs::write(
        path.join("data").join("namespace.fig"),
        base_dirs.data_dir().display().to_string(),
    )
    .map_err(IoError)?;
    // /data-local
    fs::create_dir_all(path.join("data-local")).map_err(IoError)?;
    fs::write(
        path.join("data-local").join("namespace.fig"),
        base_dirs.data_local_dir().display().to_string(),
    )
    .map_err(IoError)?;
    // /preferences
    fs::create_dir_all(path.join("preferences")).map_err(IoError)?;
    fs::write(
        path.join("preferences").join("namespace.fig"),
        base_dirs.preference_dir().display().to_string(),
    )
    .map_err(IoError)?;

    Ok(path)
}
