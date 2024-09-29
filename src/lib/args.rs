use crate::types::Difficulty;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct PublishArgs {
    #[arg(long, help = "number of workers")]
    pub n_workers: u64,
    #[arg(long, help = "log individual workers")]
    pub log_workers: bool,
    #[arg(short, long, help = "difficulty")]
    pub difficulty: Difficulty,
    #[arg(short, long, help = "path to event JSON file")]
    pub event_json: String,
    #[arg(short, long, help = "relay URL")]
    pub relay_url: String,
    #[arg(long, help = "nsec")]
    pub nsec: String,
}

#[derive(Parser, Debug)]
pub struct SellArgs {
    #[arg(long, help = "number of workers")]
    pub n_workers: u64,
    #[arg(long, help = "log individual workers")]
    pub log_workers: bool,
    #[arg(short, long, help = "RPC port")]
    pub rpc_port: u16,
    #[arg(short, long, help = "PoW price factor")]
    pub pow_price_factor: f64,
}
