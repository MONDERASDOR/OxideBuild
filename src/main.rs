use clap::{Parser, Subcommand};
use anyhow::Result;

mod commands;
mod config;
mod error;
mod platform;
mod utils;

#[derive(Parser)]
#[command(name = "OxideBuild")]
#[command(version)]
#[command(about = "Multi-platform Rust build tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build(commands::build::BuildArgs),
    Clean(commands::clean::CleanArgs),
    Test(commands::test::TestArgs),
    Deploy(commands::deploy::DeployArgs),
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Build(args) => commands::build::execute(args),
        Commands::Clean(args) => commands::clean::execute(args),
        Commands::Test(args) => commands::test::execute(args),
        Commands::Deploy(args) => commands::deploy::execute(args),
    }
}
