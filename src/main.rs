use log::info;

mod cli;
mod plugins;

fn main() {
    let action = cli::parse();
    info!("Action: {:?}", action);
}
