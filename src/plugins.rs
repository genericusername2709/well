use std::fmt::Debug;

mod explorer;

pub fn load_plugins() -> Vec<Box<dyn Plugin>> {
    // TODO
    Vec::new()
}

// TODO
pub trait Plugin: Debug {
    fn configure(&self, config: &Config);
}

pub mod default_plugins {
    use crate::plugins::explorer;

    pub const EXPLORER: explorer::Explorer = explorer::Explorer {};
}

#[derive(Debug, Default)]
pub struct Config {
    verbosity: bool,
    cli_only: bool,
}
