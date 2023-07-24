use std::path::{Path, PathBuf};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    // If it fails to retrieve user home directory.
    #[error("Failed to obtain base directories for your operating system")]
    NoBaseDirs,
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Failed to write to namespace.fig")]
    WritingNamespaceFig(#[source] std::io::Error),
}

pub fn generate(path: impl AsRef<Path>) -> Result<PathBuf, Error> {
    let path = path.as_ref().to_path_buf();

    let base_dirs = directories::BaseDirs::new().ok_or(Error::NoBaseDirs)?;

    // /home
    crate::create_dir_all!(path.join("home"))?;
    std::fs::write(
        path.join("home").join("namespace.fig"),
        base_dirs.home_dir().display().to_string(),
    )
    .map_err(Error::WritingNamespaceFig)?;
    // /config
    crate::create_dir_all!(path.join("config"))?;
    std::fs::write(
        path.join("config").join("namespace.fig"),
        base_dirs.config_dir().display().to_string(),
    )
    .map_err(Error::WritingNamespaceFig)?;
    // /data
    crate::create_dir_all!(path.join("data"))?;
    std::fs::write(
        path.join("data").join("namespace.fig"),
        base_dirs.data_dir().display().to_string(),
    )
    .map_err(Error::WritingNamespaceFig)?;
    // /data-local
    crate::create_dir_all!(path.join("data-local"))?;
    std::fs::write(
        path.join("data-local").join("namespace.fig"),
        base_dirs.data_local_dir().display().to_string(),
    )
    .map_err(Error::WritingNamespaceFig)?;
    // /preferences
    crate::create_dir_all!(path.join("preferences"))?;
    std::fs::write(
        path.join("preferences").join("namespace.fig"),
        base_dirs.preference_dir().display().to_string(),
    )
    .map_err(Error::WritingNamespaceFig)?;

    Ok(path)
}
