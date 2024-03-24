use std::{
    collections::HashMap,
    io::{Read, Write},
    path::PathBuf,
    process::Stdio,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Plugin {} failed with code {}", .plugin_name, .code)]
    PluginError { plugin_name: String, code: i32 },
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Default)]
pub struct PluginTriggerLookup<'a> {
    pub repository: Vec<&'a PluginInfo>,
    pub file: HashMap<String, &'a PluginInfo>,
}

pub fn call_on_file(cmd: &String, bytes: Vec<u8>) -> Result<Vec<u8>, Error> {
    tracing::debug!("Running command '{}'", cmd);

    let mut command = std::process::Command::new(cmd);

    command.stdin(Stdio::piped()).stdout(Stdio::piped());

    command.env("FIG_TRIGGER", "FILE");

    let mut child = command.spawn()?;
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();

    stdin.write_all(&bytes)?;

    // TODO: Handle errors in the plugin
    let status = child.wait()?;
    if !status.success() {
        return Err(Error::PluginError {
            plugin_name: cmd.clone(),
            code: status.code().take().unwrap_or(-1),
        });
    }

    let mut buf = Vec::new();
    stdout.read_to_end(&mut buf)?;

    let output = std::str::from_utf8(&buf).unwrap_or("INVALID_UTF8");
    let output = format!("\"\n{}\"", truncate_string(output, 5));

    tracing::debug!(%output, "Command '{}' ran successfully", cmd);

    Ok(buf)
}

pub fn call_on_repository(cmd: &String, repo_path: &PathBuf) -> std::io::Result<()> {
    let mut command = std::process::Command::new(cmd);

    command.arg(repo_path);
    command.env("FIG_TRIGGER", "REPOSITORY");

    let output = command.output();
    output.map(|_| ())
}

fn truncate_string(string: impl AsRef<str>, line_count: usize) -> String {
    let string = string.as_ref();
    let lines = string.lines().collect::<Vec<_>>();
    let output = if lines.len() > line_count {
        let mut output = lines[0..line_count].join("\n");
        output.push_str(&format!("\n... ({} more lines)", lines.len() - line_count));
        output
    } else {
        string.to_string()
    };
    output
}

impl<'a> PluginTriggerLookup<'a> {
    pub fn from_map(map: &'a HashMap<String, PluginInfo>) -> Result<Self, FromMapError> {
        let mut me = Self::default();

        for (_, plugin_info) in map {
            for trigger in &plugin_info.triggers {
                match trigger {
                    Trigger::Repository => {
                        me.repository.push(&plugin_info);
                    }
                    Trigger::File(ext) => match me.file.insert(ext.clone(), &plugin_info) {
                        Some(old) => {
                            return Err(FromMapError::ConflictingPluginTriggers {
                                ext: ext.clone(),
                                plugin1: old.cmd.clone(),
                                plugin2: plugin_info.cmd.clone(),
                            });
                        }
                        None => {}
                    },
                }
            }
        }

        Ok(me)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PluginInfo {
    pub cmd: String,
    triggers: Vec<Trigger>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Trigger {
    Repository,
    File(String),
}

#[derive(Debug, Error)]
pub enum FromMapError {
    #[error("Conflicting plugins!: Both {} and {} trigger on '.{}'", .plugin1, .plugin2, .ext)]
    ConflictingPluginTriggers {
        ext: String,
        plugin1: String,
        plugin2: String,
    },
}
