use clap::Parser;
use unicomet_core::cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => {
            init::execute(args);
        }
        Commands::Publish(args) => {
            publisher::execute(args);
        }
        Commands::Dev(args) => {
            debugger::execute(args);
        }
    }
}
