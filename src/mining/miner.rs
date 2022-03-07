use std::sync::mpsc::{Sender, Receiver};
use num_bigint::BigUint;

use super::worker_pool::{WorkerPool, BlockFound};
use crate::rpc::types::MinerJob;

#[derive(Debug)]
pub struct Miner {
    num_threads: usize,
    batch_size: usize,
}

impl Miner {
    pub fn new(num_threads: usize, batch_size: usize) -> Self {
        Miner {
            num_threads: num_threads,
            batch_size: batch_size,
        }
    }

    pub fn run(&self, job_recv: Receiver<MinerJob>, found_sndr: Sender<BlockFound>) {
        let worker_pool = WorkerPool::new(self.num_threads, self.batch_size, found_sndr);

        loop {
            let job = job_recv.recv().expect("can't get a new job");

            // target String convert into number bytes
            let mut target = [0u8; 32];
            let tbytes = job.target.parse::<BigUint>().unwrap().to_bytes_be();
            if tbytes.len() > target.len() {
                panic!("target num greater than U256::MAX");
            }
            let istart = target.len() - tbytes.len();
            target[istart..].clone_from_slice(&tbytes);
            
            // send a job to worker threads
            worker_pool.new_job(job.bytes.data, target, job.mining_request_id, job.sequence);
        }
    }
}