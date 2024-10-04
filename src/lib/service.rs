use jsonrpc_core::{Error, IoHandler, Params};
use jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, ServerBuilder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;
use tracing::info;

use nostr_sdk::{JsonUtil, Keys, SecretKey, UnsignedEvent};

use crate::args::{PublishArgs, SellArgs};
use crate::client::publish;
use crate::miner::{spawn_workers, PoWEvent};
use crate::sell::pow_price;

pub async fn mine(args: PublishArgs) {
    if args.n_workers < 1 {
        panic!("n_workers needs to be at least 1");
    }

    info!("starting miner service to mine and publish the JSON event");

    let start_instant = Instant::now();

    let event_file = File::open(args.event_json).expect("expect a valid filepath");
    let event_reader = BufReader::new(event_file);
    let event: PoWEvent = serde_json::from_reader(event_reader).expect("expect a valid event JSON");

    let mined_result =
        spawn_workers(args.n_workers, args.log_workers, event, args.difficulty).await;

    // log total mining time
    let duration = Instant::now().duration_since(start_instant).as_secs_f32();

    info!("successfully mined event in {} seconds", duration);
    info!("{:?}", mined_result);

    let mined_event_json = serde_json::to_string(&mined_result.event)
        .expect("expect mined_result to serialize to JSON");
    let mined_event = UnsignedEvent::from_json(mined_event_json.clone())
        .expect("expect Event to deserialize from JSON");

    // gen Keys from nsec
    let nsec = SecretKey::parse(&args.nsec).expect("expect valid nsec");
    let keys = Keys::new(nsec);

    // publish signed mined event
    publish(&args.relay_url, keys, mined_event)
        .await
        .expect("expect successfully publish mined event");
}

#[derive(Serialize, Deserialize)]
struct QuoteRpc {
    difficulty: u32,
}

#[derive(Serialize, Deserialize)]
struct MineRpc {
    event: PoWEvent,
    difficulty: u32,
    zap: String, // todo: use nostr-zapper crate
}

pub async fn sell(args: SellArgs) {
    info!("starting JSON-RPC service to sell PoW for zaps...");
    info!("listening on JSON-RPC port: {}", args.rpc_port);
    info!("selling PoW with Price factor: {}", args.pow_price_factor);

    let mut io = IoHandler::new();

    io.add_method("quote", move |params: Params| async move {
        let parsed: Result<QuoteRpc, _> = serde_json::from_value(params.parse()?)
            .map_err(|_| Error::invalid_params("Invalid params"));
        match parsed {
            Ok(QuoteRpc { difficulty }) => {
                if difficulty < 32 {
                    let pow_price = pow_price(args.pow_price_factor, difficulty);
                    Ok(json!({ "difficulty": difficulty, "pow-price": pow_price, "pow-price-factor": args.pow_price_factor }))
                } else {
                    Err(Error::invalid_params("Invalid params"))
                }
            }
            Err(_) => Err(Error::invalid_params("Invalid params")),
        }
    });

    let addr = format!("127.0.0.1:{}", args.rpc_port);

    let server = ServerBuilder::new(io)
        .cors(DomainsValidation::AllowOnly(vec![
            AccessControlAllowOrigin::Null,
        ]))
        .start_http(&addr.parse().expect("expect successfully start http server"))
        .expect("Unable to start RPC server");
    server.wait();
}
