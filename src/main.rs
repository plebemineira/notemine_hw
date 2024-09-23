use clap::{Parser, Subcommand};
use tracing::info;

use notemine::args::{PublishArgs, SellArgs};
use notemine::service::{mine, sell};

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
    Publish(PublishArgs),
    Sell(SellArgs),
}

#[tokio::main(flavor = "multi_thread", worker_threads = 100)] // todo use --n-workers to define this
async fn main() {
    tracing_subscriber::fmt::init();

    info!("🗒⛏ notemine_hw ⚡⚙️");

    let cli = Cli::parse();

    match cli.command {
        Commands::Publish(args) => {
            mine(args).await;
        }
        Commands::Sell(args) => {
            sell(args).await;
        }
    }

    // exit
    info!("exiting...");
    std::process::exit(0);
}
