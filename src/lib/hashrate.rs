use std::collections::HashMap;
use crate::miner::MinedResult;
use crate::types::{Difficulty, Hashrate, HashrateAvg, HashrateBuf, Nonce, WorkerId};

pub fn hashrate_avg(hashrate_buf: HashrateBuf) -> HashrateAvg {
    let mut hashrate_sum = 0;
    for hashrate_log in &hashrate_buf {
        hashrate_sum += hashrate_log;
    }
    let hashrate_avg = hashrate_sum as f32 / hashrate_buf.len() as f32;
    hashrate_avg
}

#[derive(Clone, Debug)]
pub struct WorkerLog {
    pub worker_id: WorkerId,
    pub hashrate: Hashrate,
    pub best_nonce: Nonce,
    pub best_pow: Difficulty,
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

    pub fn update(mut self, worker_log: WorkerLog) {
        let _ = self.map.insert(worker_log.worker_id, worker_log.clone());
    }

    pub fn sample_workers(self) -> Vec<WorkerLog> {
        let mut workers = Vec::<WorkerLog>::with_capacity(self.map.len());
        for (worker_id, worker_log) in self.map.iter() {
            let worker_log = worker_log.clone();
            workers.insert(*worker_id as usize, worker_log);
        }

        workers
    }
}

#[cfg(test)]
mod test {
    use crate::hashrate::{GlobalWorkerLogs, WorkerLog};
    use crate::types::WorkerId;

    #[test]
    fn test_global_hashrate() {
        let n_workers = 100;
        let global_worker_logs = GlobalWorkerLogs::new(n_workers);

        for worker_id in 0..n_workers {
            let hashrate = 1;
            let best_nonce = 0;
            let best_pow = 0;
            let worker_log = WorkerLog { worker_id: worker_id as WorkerId, hashrate, best_nonce, best_pow, mined_result: None };
            global_worker_logs.clone().update(worker_log);
        }

        let worker_log_samples = global_worker_logs.sample_workers();
        let mut global_hashrate = 0;
        for sample in worker_log_samples {
            global_hashrate += sample.hashrate;
        }
        assert_eq!(global_hashrate, n_workers as u64);
    }
}