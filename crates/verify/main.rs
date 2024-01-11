//! # verify cli tool

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use verify::{AizuOnlineJudge, Service};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[clap(name=env!("CARGO_PKG_NAME"))]
struct Cli {
    name: Option<String>,
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Generate,
    List,
}

fn main() {
    let cli = Cli::parse();
    env_logger::init();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::List) => list(),
        Some(Commands::Generate) => generate(),
        None => {}
    }
}

fn list() {
    log::info!("list");
}

fn generate() {
    let list = verify_core::load_verify_info::<AizuOnlineJudge>().unwrap();
    for item in list {
        AizuOnlineJudge::fetch_testcases(item.problem_id).unwrap();
    }
}
