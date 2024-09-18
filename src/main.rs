use std::fs::File;
use std::io::BufReader;
use std::time::Instant;
use clap::Parser;
use futures::future::select_all;
use tracing::info;

use notemine::miner::{mine_event, NostrEvent};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, help = "number of workers")]
    n_workers: u64,
    #[arg(short, long, help = "difficulty")]
    difficulty: u32,
    #[arg(short, long, help = "path to event JSON file")]
    event_json: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let start_instant = Instant::now();

    let args = Args::parse();
    info!("🗒⛏ notemine_hw ⚡⚙️");

    if args.n_workers < 1 {
        panic!("n_workers needs to be at least 1");
    }

    let event_file = File::open("event.json").expect("expect a valid filepath");
    let event_reader = BufReader::new(event_file);
    let event_json: NostrEvent = serde_json::from_reader(event_reader).expect("expect a valid event JSON");
    let event_json_str = serde_json::to_string(&event_json).expect("expect a valid JSON string");

    let nonce_step = u64::MAX / args.n_workers;

    let mut worker_handles = Vec::new();
    for i in 0..args.n_workers {
        let event_json_str_clone = event_json_str.clone();
        let worker_handle = tokio::spawn(async move {
            let mined_result = mine_event(i, &event_json_str_clone, args.difficulty, i, nonce_step);
            return mined_result
        });
        worker_handles.push(worker_handle);
    }

    // await for all workers until one returns
    let (mined_result, _, _) = select_all(worker_handles).await;
    let mined_result = mined_result.expect("expect valid MinedResult");

    let duration = Instant::now().duration_since(start_instant).as_secs_f32();

    info!("successfully mined event in {} seconds", duration);
    info!("{:?}", mined_result);
    info!("exiting...");
    std::process::exit(0);
}
