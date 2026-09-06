use std::{fmt::Debug, rc::Rc};

use clap::{ArgMatches, Command};
use log::info;

mod explorer;

pub fn load_plugins(config: &Config) -> Vec<Rc<dyn Plugin>> {
    info!("Loading Plugins ...");
    let plugins = fetch_plugins();
    plugins
        .into_iter()
        .filter(|plugin| {
            plugin.configure(config);
            plugin.get_availibility()
        })
        .collect()
}

fn fetch_plugins() -> Vec<Rc<dyn Plugin>> {
    let default_plugins: Vec<Rc<dyn Plugin>> = vec![Rc::new(default_plugins::EXPLORER)];
    let mut plugins: Vec<Rc<dyn Plugin>> = Vec::new();
    // TODO: Fetch plugins from config files
    plugins.extend(default_plugins);
    plugins
}

pub trait Plugin: Debug {
    fn command_name(&self) -> String;

    fn command(&self) -> Command;

    fn execute(&self, _: &ArgMatches) {}

    fn execute_tui_ctx(&self, _: &ArgMatches) {}

    fn display_name(&self) -> String {
        String::from("Undefined Plugin Name")
    }

    fn configure(&self, _: &Config) {}

    fn get_availibility(&self) -> bool {
        false
    }
}

pub mod default_plugins {
    use crate::plugins::explorer::Explorer;

    pub const EXPLORER: Explorer = Explorer::new();
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Config {
    pub verbosity: bool,
    pub cli_only: bool,
}
