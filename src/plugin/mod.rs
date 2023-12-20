use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Default)]
pub struct PluginTriggerLookup<'a> {
    pub repository: Vec<&'a PluginInfo>,
    pub file: HashMap<String, &'a PluginInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PluginInfo {
    cmd: String,
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
    #[error("Conflicting plugins!: Both {} and {} trigger on '{}'", .plugin1, .plugin2, .ext)]
    ConflictingPluginTriggers {
        ext: String,
        plugin1: String,
        plugin2: String,
    },
}

impl<'a> PluginTriggerLookup<'a> {
    pub fn from_iter<M: IntoIterator<Item = &'a (String, PluginInfo)>>(
        map: M,
    ) -> Result<Self, FromMapError> {
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
                            })
                        }
                        None => {}
                    },
                }
            }
        }

        Ok(me)
    }
}
