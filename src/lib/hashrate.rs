use std::collections::HashMap;
use tracing::info;

use cli_table::{print_stdout, Cell, Style, Table};

use crate::miner::MinedResult;
use crate::types::{Difficulty, Hash, Hashrate, HashrateAvg, HashrateBuf, Nonce, WorkerId};

pub fn report_hashrate(global_worker_logs: GlobalWorkerLogs) {
    let global_hashrate = global_worker_logs.clone().sample_global_hashrate();

    let worker_samples = global_worker_logs.sample_workers();

    if worker_samples.is_empty() {
        // nothing to report
        return;
    }

    let mut table_worker_rows = Vec::new();

    let mut global_best_nonce = 0;
    let mut global_best_pow = 0;
    let mut global_best_hash = String::new();
    for sample in worker_samples {
        let sample_clone = sample.clone();

        let worker_id = sample_clone.worker_id;
        let hashrate = sample_clone.hashrate;
        let best_nonce = sample_clone.best_nonce;
        let best_pow = sample_clone.best_pow;
        let best_hash = hex::encode(sample_clone.best_hash);

        if sample.best_pow > global_best_pow {
            global_best_nonce = best_nonce;
            global_best_pow = best_pow;
            global_best_hash = best_hash.clone();
        }

        let mut hashrate_str = hashrate.to_string();
        hashrate_str.push_str(" h/s");
        let table_worker_row = vec![
            worker_id.cell(),
            hashrate_str.cell(),
            best_nonce.cell(),
            best_pow.cell(),
            best_hash.cell(),
        ];

        table_worker_rows.push(table_worker_row);
    }

    let mut global_hashrate_str = global_hashrate.to_string();
    global_hashrate_str.push_str(" h/s");
    let global_row = vec![
        "global".cell(),
        global_hashrate_str.cell(),
        global_best_nonce.cell(),
        global_best_pow.cell(),
        global_best_hash.cell(),
    ];

    let header = vec![
        "worker id".cell().bold(true),
        "hashrate".cell().bold(true),
        "best nonce".cell().bold(true),
        "best PoW".cell().bold(true),
        "best hash".cell().bold(true),
    ];

    let mut table = Vec::new();
    table.push(global_row);
    for row in table_worker_rows {
        table.push(row);
    }
    let print_table = table.table().title(header);
    info!("reporting work...");
    print_stdout(print_table).expect("expect successful print_stdout");
}

pub fn hashrate_avg(hashrate_buf: HashrateBuf) -> HashrateAvg {
    let mut hashrate_sum = 0;
    for hashrate_log in &hashrate_buf {
        hashrate_sum += hashrate_log;
    }
    
    hashrate_sum as f32 / hashrate_buf.len() as f32
}

#[derive(Default, Clone, Debug)]
pub struct WorkerLog {
    pub worker_id: WorkerId,
    pub hashrate: Hashrate,
    pub best_nonce: Nonce,
    pub best_pow: Difficulty,
    pub best_hash: Hash,
    pub mined_result: Option<MinedResult>,
}

#[derive(Debug, Clone)]
pub struct GlobalWorkerLogs {
    map: HashMap<WorkerId, WorkerLog>,
}

impl GlobalWorkerLogs {
    pub fn new(n_workers: usize) -> Self {
        Self {
            map: HashMap::with_capacity(n_workers),
        }
    }

    pub fn update(&mut self, worker_log: WorkerLog) {
        let _ = self.map.insert(worker_log.worker_id, worker_log.clone());
    }

    pub fn sample_workers(self) -> Vec<WorkerLog> {
        let mut workers = Vec::new();
        for (_, worker_log) in self.map.iter() {
            let worker_log = worker_log.clone();
            workers.push(worker_log);
        }

        workers.sort_by_key(|worker_log| worker_log.worker_id);

        workers
    }

    pub fn sample_global_hashrate(&self) -> Hashrate {
        let mut global_hashrate = 0;
        for (_, worker_log) in self.map.iter() {
            global_hashrate += worker_log.hashrate;
        }

        global_hashrate
    }
}

#[cfg(test)]
mod test {
    use crate::hashrate::{GlobalWorkerLogs, WorkerLog};
    
    use crate::types::WorkerId;

    #[test]
    fn test_global_hashrate() {
        let n_workers = 100;
        let mut global_worker_logs = GlobalWorkerLogs::new(n_workers);

        for worker_id in 0..n_workers {
            let hashrate = 1;
            let best_nonce = 0;
            let best_pow = 0;
            let best_hash = Vec::<u8>::new();
            let worker_log = WorkerLog {
                worker_id: worker_id as WorkerId,
                hashrate,
                best_nonce,
                best_pow,
                best_hash,
                mined_result: None,
            };
            global_worker_logs.update(worker_log);
        }

        let worker_log_samples = global_worker_logs.clone().sample_workers();
        let mut global_hashrate = 0;
        for sample in worker_log_samples {
            global_hashrate += sample.hashrate;
        }
        assert_eq!(global_hashrate, n_workers as u64);
    }
}
