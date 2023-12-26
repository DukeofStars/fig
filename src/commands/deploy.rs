use std::path::{Path, PathBuf};

use clap::Args;
use color_eyre::{eyre::Context, Result};

use crate::{
    plugin::{self, PluginTriggerLookup},
    repository::RepositoryBuilder,
};

/// Deploy files from the configuration repository to your system.
#[derive(Debug, Args)]
pub struct DeployOptions {}

pub fn deploy(repo_builder: RepositoryBuilder, _options: &DeployOptions) -> Result<()> {
    let repository = repo_builder.open()?;

    let namespaces = repository.namespaces()?;

    let plugin_map = repository.load_plugins()?;
    let plugin_trigger_lookup = PluginTriggerLookup::from_map(&plugin_map)?;

    for plugin in plugin_trigger_lookup.repository {
        plugin::call_on_repository(&plugin.cmd, repository.path())
            .context("Failed to call plugin")?;
    }

    for namespace in &namespaces {
        let mut files = vec![];
        get_files(&namespace.location, &namespace.location, &mut files, 20)?;
        for file in files {
            let mut dest = namespace.target.join(&file);
            let mut src = namespace.location.join(&file);

            // Make sure dest directory exists
            if let Some(parent) = dest.parent() {
                if !parent.exists() {
                    crate::create_dir_all!(parent)?;
                }
            }

            let mut contents = std::fs::read(&src)?;
            while let Some(plugin) = src
                .extension()
                .and_then(|ext| plugin_trigger_lookup.file.get(ext.to_str().unwrap()))
            {
                contents = plugin::call_on_file(&plugin.cmd, contents)?;

                dest = dest.with_extension("");
                src = src.with_extension("");
            }

            std::fs::write(&dest, contents)?;
            // crate::copy_file!(&src, &dest).context(format!(
            //     "Failed to copy '{}' to '{}'",
            //     src.display(),
            //     dest.display()
            // ))?;
        }
    }

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
    } else if let Some(extension) = entry.extension() {
        if extension == "fig" {
            return Ok(());
        }
    } else if entry.is_dir() {
        for entry in entry.read_dir()? {
            let entry = entry?.path();
            get_files(parent_dir, &entry, files, depth - 1)?;
        }

        return Ok(());
    }
    // Strip fig dir
    let path = entry
        .strip_prefix(parent_dir)
        .context("Failed to strip path prefix")?;
    files.push(path.to_path_buf());
    Ok(())
}
