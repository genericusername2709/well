use std::{
    cell::{Cell, RefCell},
    path::PathBuf,
};

use clap::{ArgMatches, Command, CommandFactory, FromArgMatches, Parser};
use log::warn;

use crate::plugins::Plugin;

#[derive(Parser)]
#[command(about = "A TUI File Explorer.")]
struct ExplorerCli {
    #[arg(index = 1, help = "Directory to act as root for the explorer")]
    dir: Option<PathBuf>,
}

#[derive(Debug, Default)]
pub struct Explorer {
    available: Cell<bool>,
    dir: RefCell<PathBuf>,
    current_path: RefCell<PathBuf>,
}

impl Explorer {
    pub const fn new() -> Explorer {
        Explorer {
            available: Cell::new(true),
            dir: RefCell::new(PathBuf::new()),
            current_path: RefCell::new(PathBuf::new()),
        }
    }

    fn parse_args(&self, args: &ArgMatches) {
        let explorer_cli = ExplorerCli::from_arg_matches(args).unwrap_or_else(|e| e.exit());
        if let Some(dir) = explorer_cli.dir {
            self.dir.replace(dir.clone());
            self.current_path.replace(dir);
        }
    }
}

// TODO
impl Plugin for Explorer {
    fn command_name(&self) -> String {
        String::from("explorer")
    }

    fn command(&self) -> Command {
        ExplorerCli::command().name(self.command_name())
    }

    fn display_name(&self) -> String {
        String::from("Default File Explorer")
    }

    fn configure(&self, config: &super::Config) {
        if config.cli_only {
            if config.verbosity {
                // While this is a warning - We do not want to show this warning,
                // If the verbosity flag is not set to true
                warn!(
                    "[Explorer] cli-only mode not supported for Explorer plugin,\
                    This plugin will not be available."
                );
            }
            self.available.replace(false);
        }
    }

    fn execute(&self, arg_matches: &ArgMatches) {
        self.parse_args(arg_matches);
    }

    fn execute_tui_ctx(&self, arg_matches: &ArgMatches) {
        self.parse_args(arg_matches);
    }

    fn get_availibility(&self) -> bool {
        self.available.get()
    }
}
