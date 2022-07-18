#![feature(fs_try_exists)]
use std::{cell::RefCell, fs, path::PathBuf, rc::Rc};

use clap::{Args, Parser, Subcommand};
use fig::{config_folder_path, FigConfig};

fn main() {
    let cli = Cli::parse();

    let config_path = config_folder_path().unwrap().join("config.toml");

    let config_file = fs::read_to_string(&config_path);

    let config: Rc<RefCell<FigConfig>> = Rc::new(RefCell::new(match config_file {
        Ok(_) => toml::from_str(&config_file.unwrap()).unwrap(),
        Err(_) => FigConfig { configs: vec![] },
    }));

    use SubCommand::*;
    match cli.cmd {
        Add(add) => {
            fig::add(
                config.clone(),
                add.path,
                add.namespace.unwrap_or_default().replace(".", "/").into(),
                add.verbose,
            );
        }
        Forget(forget) => {
            fig::remove(
                config.clone(),
                forget.name,
                forget.namespace.unwrap_or_default(),
                !forget.quiet, // Invert quiet so the fig::remove can treat it as a 'verbose' flag.
            );
        }
        List => {
            fig::list(config.clone());
        }
    }

    let config: FigConfig = config.borrow().clone();

    fs::write(
        config_path,
        toml::to_string_pretty::<FigConfig>(&config).unwrap(),
    )
    .unwrap();
}

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    Add(AddArgs),
    Forget(ForgetArgs),
    List,
}

#[derive(Args)]
struct AddArgs {
    #[clap(short, long)]
    verbose: bool,
    #[clap(parse(from_os_str))]
    path: PathBuf,
    #[clap(short, long)]
    namespace: Option<String>,
}

#[derive(Args)]
struct ForgetArgs {
    #[clap(short, long)]
    quiet: bool,
    #[clap(name = "name", parse(from_str))]
    name: Option<String>,
    #[clap(short, long)]
    namespace: Option<String>,
}
