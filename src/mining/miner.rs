#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::thread;
use std::sync::{Arc, Mutex};
use futures::future::join_all;
use tokio::sync::mpsc;

use rand::Rng;
use num256::Uint256;
use blake3;

use super::{mine_header, MineRusult};
use crate::rpc::types::NewBlocksResponse;

#[derive(Debug)]
pub struct Miner {
    threads: u8,
    batch_size: u64,
    // hashrate: Meter,
    randomness: u64,
}

impl Miner {
    pub fn new(threads: u8, batch_size: u64) -> Self {
        Miner {
            threads: threads,
            batch_size: batch_size,
            randomness: 0,
        }
    }

    pub async fn start(&self, mut ch_receiver: mpsc::Receiver<NewBlocksResponse>) {
        println!("start mining pool with {} threads.", self.threads);

        while let Some(next_block) = ch_receiver.recv().await {
            let mut workers = Vec::with_capacity(self.threads as usize);

            // find space
            let mut rnd = rand::thread_rng();
            let initial_randomness: u64 = rnd.gen();
            let randomness = Arc::new(Mutex::<u64>::new(initial_randomness));

            println!("new block. {:?}", next_block);

            for _ in 0..self.threads {
                let next_block_clone = next_block.clone();
                let batch_size = self.batch_size;

                // diffrent initials
                *randomness.lock().unwrap() += batch_size;
                let randomness = Arc::clone(&randomness);
                println!("rand: {:?}", initial_randomness);

                workers.push(
                    thread::spawn(move || {
                        println!("calculate!");
                        let result = Self::mine(&next_block_clone, randomness, batch_size);

                        if result.found() {
                            println!("FOUND BLOCK! {:?}", result);
                        }
                    })
                );
            }

            for child in workers {
                let _ = child.join();
            }
        }
    }

    fn mine(next_block: &NewBlocksResponse, randomness: Arc<Mutex<u64>>, batch_size: u64) -> MineRusult {
        let mut result = MineRusult::empty();

        loop {
            if result.found() {
                break;
            }

            let mut num = randomness.lock().unwrap();

            result = mine_header(
                next_block.mining_request_id,
                &next_block.bytes.data,
                *num,
                &next_block.target,
                batch_size);

            *num += batch_size;

            // нужно выходить из цикла
            println!("randomless: {}", num)
        }
    
        result
    }
}