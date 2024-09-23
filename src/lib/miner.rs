use circular_buffer::CircularBuffer;
use futures::future::select_all;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use sha2::{Digest, Sha256};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tracing::info;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NostrEvent {
    pub pubkey: String,
    pub kind: u32,
    pub content: String,
    pub tags: Vec<Vec<String>>,
    pub id: Option<String>,
    pub created_at: Option<u64>,
    pub sig: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MinedResult {
    pub event: NostrEvent,
    pub total_time: f64,
}

fn serialize_u64_as_number<S>(x: &u64, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    s.serialize_u64(*x)
}

#[derive(Serialize)]
struct HashableEvent<'a>(
    u32,
    &'a str,
    #[serde(serialize_with = "serialize_u64_as_number")] u64,
    u32,
    &'a Vec<Vec<String>>,
    &'a str,
);

#[inline]
fn get_event_hash(event: &NostrEvent) -> Vec<u8> {
    let hashable_event = HashableEvent(
        0u32,
        &event.pubkey,
        event.created_at.unwrap_or_else(|| {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("expect valid duration since UNIX_EPOCH")
                .as_secs()
        }),
        event.kind,
        &event.tags,
        &event.content,
    );

    let serialized_str = match to_string(&hashable_event) {
        Ok(s) => s,
        Err(_) => return vec![],
    };

    let hash_bytes = Sha256::digest(serialized_str.as_bytes()).to_vec();
    hash_bytes
}

#[inline]
fn get_pow(hash_bytes: &[u8]) -> u32 {
    let mut count = 0;
    for &byte in hash_bytes {
        if byte == 0 {
            count += 8;
        } else {
            count += byte.leading_zeros() as u32;
            break;
        }
    }
    count
}

pub async fn spawn_workers(
    n_workers: u64,
    event: NostrEvent,
    difficulty: u32,
    log_interval: u64,
) -> MinedResult {
    let nonce_step = u64::MAX / n_workers;

    let mut worker_handles = Vec::new();
    for i in 0..n_workers {
        let event_clone = event.clone();
        let start_nonce = i * nonce_step;
        let worker_handle = tokio::spawn(async move {
            let mined_result = mine_event(
                i,
                event_clone,
                difficulty,
                start_nonce,
                log_interval,
            );
            return mined_result;
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

    mined_result
}

// hashrate_buf is useful for averaging hashrate
const HASHRATE_BUF_SIZE: usize = 10;

fn hashrate_avg(hashrate_buf: CircularBuffer<HASHRATE_BUF_SIZE, u64>) -> f32 {
    let mut hashrate_sum = 0;
    for hashrate_log in &hashrate_buf {
        hashrate_sum += hashrate_log;
    }
    let hashrate_avg = hashrate_sum as f32 / hashrate_buf.len() as f32;
    hashrate_avg
}
fn mine_event(
    worker_id: u64,
    mut event: NostrEvent,
    difficulty: u32,
    start_nonce: u64,
    log_interval: u64,
) -> MinedResult {

    if event.created_at.is_none() {
        let current_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("expect valid duration since UNIX_EPOCH")
            .as_secs();
        event.created_at = Some(current_timestamp);
    }

    let mut nonce_index = None;
    for (i, tag) in event.tags.iter().enumerate() {
        if tag.len() > 0 && tag[0] == "nonce" {
            nonce_index = Some(i);
            break;
        }
    }
    if nonce_index.is_none() {
        event.tags.push(vec![
            "nonce".to_string(),
            "0".to_string(),
            difficulty.to_string(),
        ]);
        nonce_index = Some(event.tags.len() - 1);
    }

    info!(
        "starting worker with parameters: worker id: {} | difficulty: {} | start_nonce: {}",
        worker_id, difficulty, start_nonce
    );

    let mut nonce: u64 = start_nonce;
    let mut total_hashes: u64 = 0;

    let mut best_pow: u32 = 0;
    #[allow(unused_assignments)]
    let mut best_nonce: u64 = 0;
    #[allow(unused_assignments)]
    let mut best_hash_bytes: Vec<u8> = Vec::new();

    let start_instant = Instant::now();
    let mut last_log_instant = start_instant;

    let mut hashrate_buf = CircularBuffer::<HASHRATE_BUF_SIZE, u64>::new();

    loop {
        // report hashrate every log_interval secs
        if Instant::now().duration_since(last_log_instant) > Duration::from_secs(log_interval) {
            last_log_instant = Instant::now();

            let hashrate = total_hashes / log_interval;
            hashrate_buf.push_back(hashrate);
            total_hashes = 0;

            let hashrate_avg = hashrate_avg(hashrate_buf.clone());

            info!(
                "worker id: {} | hashrate: {:.01} h/s | best pow: {} | best nonce: {} | best hash: {:?}",
                worker_id,
                hashrate_avg,
                best_pow,
                best_nonce,
                hex::encode(best_hash_bytes.clone())
            );
        }

        if let Some(index) = nonce_index {
            if let Some(tag) = event.tags.get_mut(index) {
                if tag.len() >= 3 {
                    tag[1] = nonce.to_string();
                    tag[2] = difficulty.to_string();
                }
            }
        }

        let hash_bytes = get_event_hash(&event);
        if hash_bytes.is_empty() {
            panic!("Failed to compute event hash.")
        }

        let pow = get_pow(&hash_bytes);

        if pow > best_pow {
            best_pow = pow;
            best_nonce = nonce;
            best_hash_bytes = hash_bytes.clone();
        }

        if pow >= difficulty {
            let event_hash = hex::encode(&hash_bytes);
            event.id = Some(event_hash.clone());
            let total_time = start_instant.elapsed().as_secs_f64();

            let result = MinedResult { event, total_time };

            return result;
        }

        nonce = nonce.wrapping_add(1);
        total_hashes += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex::FromHex;

    #[test]
    fn test_mine_event() {
        tracing_subscriber::fmt::init();
        let event = NostrEvent {
            pubkey: "e771af0b05c8e95fcdf6feb3500544d2fb1ccd384788e9f490bb3ee28e8ed66f".to_string(),
            kind: 1,
            content: "hello world".to_string(),
            tags: vec![],
            id: None,
            created_at: Some(1668680774),
            sig: None,
        };

        let difficulty = 18;
        let worker_id = 0;
        let mined_result = mine_event(worker_id, event.clone(), difficulty, 0, 1);

        assert_eq!(mined_result.event.pubkey, event.pubkey);
        assert_eq!(mined_result.event.kind, event.kind);
        assert_eq!(mined_result.event.content, event.content);
        assert_eq!(mined_result.event.created_at, event.created_at);
        assert_eq!(mined_result.event.tags[0][0], "nonce");

        assert_eq!(mined_result.event.tags[0][2], difficulty.to_string());
        let id = mined_result
            .event
            .id
            .expect("expect mined_result.event.id is Some");
        let id_bytes = Vec::from_hex(id).expect("expect valid sha256 as hex");

        assert!(get_pow(&id_bytes) >= difficulty);
    }
    #[test]
    fn test_get_event_hash() {
        let event = NostrEvent {
            pubkey: "e771af0b05c8e95fcdf6feb3500544d2fb1ccd384788e9f490bb3ee28e8ed66f".to_string(),
            kind: 1,
            content: "hello world".to_string(),
            tags: vec![],
            id: None,
            created_at: Some(1668680774),
            sig: None,
        };

        let expected_hash = "bb9727a19e7ed120333e994ada9c3b6e4a360a71739f9ea33def6d69638fff30";

        let hash_bytes = get_event_hash(&event);
        let hash_hex = hex::encode(&hash_bytes);

        assert_eq!(hash_hex, expected_hash);
    }
}
