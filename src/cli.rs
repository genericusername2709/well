use clap::{Args, Parser, Subcommand};

use crate::plugins::{Config, Plugin, default_plugins};

pub fn parse(plugins: &Vec<Box<dyn Plugin>>) -> Action {
    // TODO
    Cli::parse();
    Action::new()
}

#[derive(Parser, Debug)]
#[command(
    name = "Well",
    version = "1.0.0",
    about = "A TUI to help orchestrate executing commands, manage windows, etc.",
    long_about = "A plugin based TUI that allows process and window management"
)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,

    #[arg(
        short,
        long,
        help = "Only runs as an Well as an orchestrator, does not start TUI. This option is only honoured if the command supports it"
    )]
    cli_only: bool,

    #[command(subcommand)]
    commands: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Explicitly specify the plugin(s) you want to start the TUI with.
    Start(StartArgs),

    // Plugin Management Commands
    ListPlugins,
    InstallPlugins(InstallPluginArgs),
    UninstallPlugins(UninstallPluginArgs),
}

#[derive(Args, Debug)]
struct StartArgs {
    #[arg(index = 1)]
    plugin: Option<String>,
}

#[derive(Args, Debug)]
struct InstallPluginArgs {}

#[derive(Args, Debug)]
struct UninstallPluginArgs {}

#[derive(Debug)]
pub struct Action {
    /// List of actions to perform, the actions are performed in-order.
    actions: Vec<SubActions>,
}

impl Action {
    pub fn new() -> Action {
        let action = Action {
            actions: vec![SubActions::Plugin(Box::new(default_plugins::EXPLORER))],
        };
        action.configure_actions(&Config::default());
        action
    }

    pub fn configure_actions(&self, config: &Config) {
        self.actions.iter().for_each(|action| {
            if let SubActions::Plugin(plugin) = action {
                plugin.configure(config);
            }
        });
    }
}

#[derive(Debug)]
enum SubActions {
    Plugin(Box<dyn Plugin>),
    ListPlugins,
}
