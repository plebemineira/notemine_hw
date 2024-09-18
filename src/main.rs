use std::fs::File;
use std::io::BufReader;
use std::time::Instant;
use clap::Parser;
use futures::future::select_all;
use tracing::info;
use nostr_sdk::{NostrSigner, Keys, SecretKey, UnsignedEvent, JsonUtil};

use notemine::miner::{mine_event, NostrEvent};
use notemine::client::publish;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, help = "number of workers")]
    n_workers: u64,
    #[arg(short, long, help = "difficulty")]
    difficulty: u32,
    #[arg(short, long, help = "path to event JSON file")]
    event_json: String,
    #[arg(short, long, help = "relay URL")]
    relay_url: String,
    #[arg(short, long, help = "log interval (secs)")]
    log_interval: u64,
    #[arg(long, help = "nsec")]
    nsec: String,
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
            let mined_result = mine_event(i, &event_json_str_clone, args.difficulty, i, nonce_step, args.log_interval);
            return mined_result
        });
        worker_handles.push(worker_handle);
    }

    // await for all workers until one returns
    let (mined_result, _, _) = select_all(worker_handles).await;
    let mined_result = mined_result.expect("expect valid MinedResult");

    // log total mining time
    let duration = Instant::now().duration_since(start_instant).as_secs_f32();

    info!("successfully mined event in {} seconds", duration);
    info!("{:?}", mined_result);

    let mined_event_json = serde_json::to_string(&mined_result.event).expect("expect mined_result to serialize to JSON");
    let mined_event = UnsignedEvent::from_json(mined_event_json.clone()).expect("expect Event to deserialize from JSON");

    // sign mined event
    let nsec = SecretKey::parse(&args.nsec).expect("expect valid nsec");
    let keys = Keys::new(nsec);
    println!("aaaa {:?}", keys);
    let signer = NostrSigner::from(keys);
    let signed_mined_event = signer.sign_event(mined_event).await.expect("expect successful signature");

    // publish signed mined event
    publish(&args.relay_url, signed_mined_event).await.expect("expect successfully publish mined event");

    // exit
    info!("exiting...");
    std::process::exit(0);
}
