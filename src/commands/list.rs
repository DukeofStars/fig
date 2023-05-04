use clap::Args;
use fig::repository::Repository;
use miette::Result;
use owo_colors::OwoColorize;

use crate::list::get_all_files;

#[derive(Args)]
pub struct ListOptions {
    #[clap(short, long)]
    tree: bool,
}

pub fn list(repository: &Repository, options: ListOptions) -> Result<()> {
    let namespaces = repository.namespaces()?;
    for (namespace, files) in get_all_files(repository)? {
        if options.tree {
            println!(
                "{namespace:12}: {path}",
                namespace = namespace.blue(),
                path = namespaces.get(&namespace).unwrap().display().bright_blue()
            );
        }
        for file in files {
            println!(
                "{}{path}",
                if options.tree {
                    " ".repeat(12)
                } else {
                    "".to_string()
                },
                path = file.display().green()
            );
        }
    }

    Ok(())
}
