use clap::{Parser, Subcommand};
use tracing::info;

use notemine::args::{MineArgs, SellArgs};
use notemine::service::{mine, serve};

#[derive(Parser, Debug)]
#[command(
    name = "notemine_hw",
    about = "nostr note miner written in rust, aiming at hardware acceleration"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Mine(MineArgs),
    Sell(SellArgs),
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    info!("🗒⛏ notemine_hw ⚡⚙️");

    let cli = Cli::parse();

    match cli.command {
        Commands::Mine(args) => {
            mine(args).await;
        }
        Commands::Sell(args) => {
            serve(args).await;
        }
    }

    // exit
    info!("exiting...");
    std::process::exit(0);
}
