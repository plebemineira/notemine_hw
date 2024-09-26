use clap::{Parser, Subcommand};
use tracing::info;

use tokio::runtime::Builder;

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

fn main() {
    tracing_subscriber::fmt::init();
    info!("🗒⛏ notemine_hw ⚡⚙️");

    let cli = Cli::parse();

    // Set the number of worker threads dynamically based on the command-line arguments.
    let worker_threads = match &cli.command {
        Commands::Publish(args) => args.n_workers as usize,
        Commands::Sell(args) => args.n_workers as usize,
    };

    // Create a custom runtime with the specified number of worker threads
    let runtime = Builder::new_multi_thread()
        .worker_threads(worker_threads)
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime");

    // Run the entire application logic inside this custom runtime
    runtime.block_on(async move {
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
    });
}
