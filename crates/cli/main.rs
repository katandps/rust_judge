mod dropbox;
mod services;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Setup,
    SaveMetadata,
    FetchTestcase,
    Verify,
    Run,
    AtcoderList,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Setup) => dropbox::setup()?,
        Some(Commands::SaveMetadata) => services::save_metadata()?,
        Some(Commands::FetchTestcase) => services::fetch_testcases()?,
        Some(Commands::Verify) => services::verify()?,
        Some(Commands::Run) => run()?,
        Some(Commands::AtcoderList) => dropbox::list()?,
        None => (),
    }
    Ok(())
}

fn run() -> anyhow::Result<()> {
    Ok(())
}
