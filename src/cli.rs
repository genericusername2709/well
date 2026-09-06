use std::{env::args, rc::Rc};

use clap::{ArgMatches, Args, CommandFactory, FromArgMatches, Parser, Subcommand};
use log::{LevelFilter, info};

use crate::plugins::{Config, Plugin, load_plugins};

pub fn parse() -> Action {
    let config = get_config_from_args();
    env_logger::builder()
        .filter(
            Option::None,
            if config.verbosity {
                LevelFilter::Info
            } else {
                LevelFilter::Warn
            },
        )
        .init();
    let plugins = load_plugins(&config);
    info!(
        "Plugins available: \n{:?}",
        &plugins
            .iter()
            .map(|plugin| plugin.display_name())
            .collect::<String>()
    );
    let mut cli = Cli::command();
    for plugin in plugins.iter() {
        cli = cli.subcommand(plugin.command());
    }
    let cli_args = cli.get_matches();

    if let Some((subcmd, arg_matches)) = cli_args.subcommand() {
        if let Some(plugin) = plugins.iter().find(|p| p.command_name() == subcmd) {
            // Find if the sub-cmd if it is from from a plugin
            return Action::new(
                ActionType::Plugin(plugin.clone(), Box::new(arg_matches.clone())),
                plugins,
                &config,
            );
        } else {
            // The sub-cmd should be one-of well's supported subcommand
            let parsed_cli = Cli::parse();
            let action_type = get_action_type_from_native_sub_commands(&parsed_cli, &plugins);
            return Action {
                action_type,
                plugins,
                config,
            };
        }
    }
    // No sub-cmd specified, opening list plugins menu
    Action {
        action_type: ActionType::ListPlugins,
        plugins,
        config,
    }
}

fn get_action_type_from_native_sub_commands(
    parsed_cli: &Cli,
    plugins: &Vec<Rc<dyn Plugin>>,
) -> ActionType {
    // TODO
    ActionType::ListPlugins
}

fn get_config_from_args() -> Config {
    let args = args().collect::<Vec<String>>();
    let matches = Cli::command()
        .ignore_errors(true)
        .try_get_matches_from_mut(&args)
        .unwrap_or_else(|e| e.exit());
    let cli_args = Cli::from_arg_matches(&matches).unwrap_or_else(|e| e.exit());
    Config {
        verbosity: cli_args.verbose,
        cli_only: cli_args.cli_only,
    }
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
    InstallPlugin(InstallPluginArgs),
    UninstallPlugin(UninstallPluginArgs),
}

#[derive(Args, Debug)]
struct StartArgs {
    /// Ordered list of plugins to be executed in each run.
    /// Input should be of form: "plugin1 arg1 arg2 arg3;plugin2 arg1;plugin 3;plugin 4 arg1"
    #[arg(index = 1)]
    ordered_plugins: String,
}

#[derive(Args, Debug)]
struct InstallPluginArgs {}

#[derive(Args, Debug)]
struct UninstallPluginArgs {}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Action {
    /// List of actions to perform, the actions are performed in-order.
    pub action_type: ActionType,
    pub plugins: Vec<Rc<dyn Plugin>>,
    pub config: Config,
}

impl Action {
    pub fn new(action_type: ActionType, plugins: Vec<Rc<dyn Plugin>>, config: &Config) -> Action {
        //TODO: Create subactions based on the ordered list of plugins received
        Action {
            action_type: action_type,
            plugins: plugins,
            config: *config,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ActionType {
    Start(Vec<Rc<dyn Plugin>>),
    Plugin(Rc<dyn Plugin>, Box<ArgMatches>),
    ListPlugins,
    InstallPlugin,
    UninstallPlugin,
}
