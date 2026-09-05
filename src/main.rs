mod cli;
mod plugins;

fn main() {
    let plugins = plugins::load_plugins();

    let action = cli::parse(&plugins);
}
