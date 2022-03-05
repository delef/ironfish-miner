use std::thread;
use std::sync::mpsc::Receiver;
use num_bigint::BigUint;

use super::worker_pool::WorkerPool;
use crate::rpc::types::NewBlocksResponse;

#[derive(Clone, Debug)]
pub struct MineTask {
    pub bytes: [u8; 200],
    pub target: String,
    pub request_id: u32,
    pub sequence: u64,
}

#[derive(Debug)]
pub struct Miner {
    worker_pool: WorkerPool,
}

impl Miner {
    pub fn new(num_threads: usize, batch_size: usize) -> Self {
        Miner {
            worker_pool: WorkerPool::new(num_threads, batch_size),
        }
    }

    pub fn start(&self, recv: Receiver<NewBlocksResponse>) {
        loop {
            let new_job = recv.recv().expect("I can't get a new job");

            // target number () convert into bytes
            let mut target = [0u8; 32];
            let tbytes = new_job.target.parse::<BigUint>().unwrap().to_bytes_be();
            if tbytes.len() > target.len() {
                panic!("target num greater than U256::MAX");
            }
            let istart = target.len() - tbytes.len();
            target[istart..].clone_from_slice(&tbytes);
                        
            self.worker_pool.new_job(
                new_job.bytes.data,
                target,
                new_job.mining_request_id,
                new_job.sequence,
            );
        }
    }
}