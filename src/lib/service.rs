use std::fs::File;
use std::io::BufReader;
use std::time::Instant;
use futures::future::select_all;
use tracing::info;

use nostr_sdk::{NostrSigner, Keys, SecretKey, UnsignedEvent, JsonUtil};

use crate::client::publish;
use crate::miner::{mine_event, NostrEvent};
use crate::args::{MineArgs, SellArgs};

pub async fn mine(args: MineArgs) {
    if args.n_workers < 1 {
        panic!("n_workers needs to be at least 1");
    }

    info!("starting miner service to mine and publish the JSON event");

    let start_instant = Instant::now();

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
    let (mined_result, _, remaining_handles) = select_all(worker_handles).await;

    // abort all remaining worker handles
    for h in remaining_handles {
        h.abort_handle().abort();
    }

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
    let signer = NostrSigner::from(keys);
    let signed_mined_event = signer.sign_event(mined_event).await.expect("expect successful signature");

    // publish signed mined event
    publish(&args.relay_url, signed_mined_event).await.expect("expect successfully publish mined event");
}

pub async fn serve(args: SellArgs) {
    info!("starting JSON-RPC service to sell PoW for zaps...");
    info!("listening on JSON-RPC port: {}", args.rpc_port);
    info!("selling PoW with Price factor: {}", args.pow_price_factor);
}