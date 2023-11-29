use std::path::PathBuf;

use crate::repository::Repository;

pub trait Plugin {
    type Err;

    fn info(&self) -> PluginInfo;

    fn run_on_repository(&mut self, repository: Repository) -> Result<(), Self::Err>;
    fn run_on_file(&mut self, file: PathBuf) -> Result<(), Self::Err>;
}
pub struct PluginInfo {
    pub name: String,
    pub triggers: Vec<Trigger>,
}
pub enum Trigger {
    Repository,
    File(FileTrigger),
}
pub struct FileTrigger {
    pub ext: String,
}
