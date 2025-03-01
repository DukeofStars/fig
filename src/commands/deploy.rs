use std::path::{Path, PathBuf};

use clap::Args;
use color_eyre::{eyre::Context, Result};
use tracing::{debug, error, info};

use crate::{
    plugin::{self},
    repository::RepositoryBuilder,
};

#[derive(Debug, Args)]
pub struct DeployOptions {}

pub fn deploy(repo_builder: RepositoryBuilder, _options: &DeployOptions) -> Result<()> {
    info!("Deploying files");

    let repository = repo_builder.open()?;

    let namespaces = repository.namespaces()?;

    let plugin_map = repository.load_plugins()?;

    info!("Deploying files");

    for plugin in plugin_map.repository {
        plugin::call_on_repository(&plugin.cmd, repository.path())
            .context("Failed to call plugin")?;
    }

    for namespace in &namespaces {
        let mut files = vec![];
        get_files(&namespace.location, &namespace.location, &mut files, 20)?;
        'floop: for file in files {
            for target in &namespace.targets {
                debug!(
                    "Deploying file '{}' to '{}'",
                    file.display(),
                    target.display()
                );

                let mut dest = target.join(&file);
                let mut src = namespace.location.join(&file);

                // Make sure dest directory exists
                if let Some(parent) = dest.parent() {
                    if !parent.exists() {
                        crate::create_dir_all!(parent)?;
                    }
                }

                // Run the file contents through plugins.
                let mut contents = std::fs::read(&src)?;
                while let Some(plugin) = src
                    .extension()
                    .and_then(|ext| plugin_map.file.get(ext.to_str().unwrap()))
                {
                    contents = match plugin::call_on_file(&plugin.cmd, contents) {
                        Ok(contents) => contents,
                        Err(err) => {
                            error!(%err, "Calling plugin '{}' failed", &plugin.cmd);
                            continue 'floop;
                        }
                    };

                    // Trim back extension.
                    dest = dest.with_extension("");
                    src = src.with_extension("");
                }

                std::fs::write(&dest, contents)
                    .wrap_err(format!("Failed to write to '{}'", dest.display()))?;
                // crate::copy_file!(&src, &dest).context(format!(
                //     "Failed to copy '{}' to '{}'",
                //     src.display(),
                //     dest.display()
                // ))?;
            }
        }
    }

    info!("Deploying files successful");

    Ok(())
}

fn get_files(
    parent_dir: &PathBuf,
    entry: &Path,
    files: &mut Vec<PathBuf>,
    depth: u8,
) -> Result<()> {
    if depth == 0 {
        return Ok(());
    } else if entry.is_dir() {
        for entry in entry.read_dir()? {
            let entry = entry?.path();
            get_files(parent_dir, &entry, files, depth - 1)?;
        }

        return Ok(());
    } else if let Some(extension) = entry.extension() {
        if extension == "fig" {
            return Ok(());
        }
    }
    // Strip fig dir
    let path = entry
        .strip_prefix(parent_dir)
        .context("Failed to strip path prefix")?;
    files.push(path.to_path_buf());
    Ok(())
}
