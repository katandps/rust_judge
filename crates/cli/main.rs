mod dropbox;
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Setup,
    List,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Setup) => dropbox::setup()?,
        Some(Commands::List) => dropbox::list()?,
        None => (),
    }
    Ok(())
}
