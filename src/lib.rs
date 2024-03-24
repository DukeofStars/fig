use directories::ProjectDirs;

pub mod commands;
mod log_utils;
pub mod namespace;
pub mod plugin;
pub mod repository;
pub mod template;

pub fn project_dirs() -> ProjectDirs {
    ProjectDirs::from("", "", "fig")
        .expect("Failed to find home directory, maybe your operating system is unsupported?")
}
