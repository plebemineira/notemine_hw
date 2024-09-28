use std::collections::HashMap;
use tracing::info;

use tabled::{
    builder::Builder,
    settings::{Alignment, Style, object::Row, Color}
};

use crate::miner::MinedResult;
use crate::types::{Hashrate, HashrateAvg, HashrateBuf, WorkerId};

pub fn report_hashrate(global_worker_logs: GlobalWorkerLogs) {
    let global_hashrate = global_worker_logs.clone().sample_global_hashrate();

    let worker_samples = global_worker_logs.sample_workers();

    if worker_samples.is_empty() {
        // nothing to report
        return;
    }

    let mut table_worker_rows = Vec::new();

    for sample in worker_samples {
        let sample_clone = sample.clone();

        let worker_id = sample_clone.worker_id;
        let hashrate = sample_clone.hashrate;

        let mut hashrate_str = hashrate.to_string();
        hashrate_str.push_str(" h/s");
        let table_worker_row = vec![
            worker_id.to_string(),
            hashrate_str.to_string(),
        ];

        table_worker_rows.push(table_worker_row);
    }

    let mut global_hashrate_str = global_hashrate.to_string();
    global_hashrate_str.push_str(" h/s");
    let global_row = vec![
        "global".to_string(),
        global_hashrate_str.to_string(),
    ];

    let header = vec![
        "worker id",
        "hashrate",
    ];

    let mut tabled_builder = Builder::default();
    tabled_builder.push_record(header);
    tabled_builder.push_record(global_row);

    for row in table_worker_rows {
        tabled_builder.push_record(row);
    }

    let mut tabled = tabled_builder.build();
    tabled.with(Style::rounded());
    tabled.with(Alignment::center());
    tabled.modify(Row::from(1), Color::BOLD);

    info!("reporting work... \n{}", tabled);
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
            let worker_log = WorkerLog {
                worker_id: worker_id as WorkerId,
                hashrate,
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
