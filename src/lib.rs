mod config;
mod infra;
mod install;
mod model;
mod sync;

use std::path::PathBuf;

use crate::config::cli::{Args, Command};
use crate::infra::err;

const DEFAULT_CONFIG_PATH: &str = ".tool.toml";

pub fn run() {
    let args: Args = facet_args::from_std_args().expect("parsable args");

    //     // TODO: this is redundant for the `default-config` command
    //     // See: https://github.com/chshersh/tool-sync/issues/75
    let config_path = resolve_config_path(args.config_path);

    match args.command {
        Command::DefaultConfig { path } => match path {
            true => print_default_path(),
            false => config::template::generate_default_config(),
        },
        Command::Sync => sync::sync_from_path(config_path, args.tool, args.proxy),
        Command::Install => {
            install::install(config_path, args.tool.expect("tool name"), args.proxy)
        }
    }
}

fn resolve_config_path(config_path: Option<PathBuf>) -> PathBuf {
    match config_path {
        Some(path) => path,
        None => get_default_config_path(),
    }
}

fn get_default_config_path() -> PathBuf {
    match dirs::home_dir() {
        Some(home_path) => {
            let mut path = PathBuf::new();
            path.push(home_path);
            path.push(DEFAULT_CONFIG_PATH);
            path
        }
        None => {
            err::abort_suggest_issue("Unable to find $HOME directory");
        }
    }
}

fn print_default_path() {
    println!("{}", get_default_config_path().display());
}
