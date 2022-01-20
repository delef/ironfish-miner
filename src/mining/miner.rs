#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::{thread, time};
use std::collections::HashMap;

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
    tasks: HashMap<u64, MineRusult>,
    randomness: u64,
}

impl Miner {
    pub fn new(threads: u8, batch_size: u64) -> Self {
        Miner {
            threads: threads,
            batch_size: batch_size,
            tasks: HashMap::new(),
            randomness: 0,
        }
    }

    pub fn mine(&self, next_block: NewBlocksResponse) -> MineRusult {
        let mut rnd = rand::thread_rng();
        let mut result = MineRusult::empty();
        let mut initial_randomness: u64 = rnd.gen();

        loop {
            if result.found() {
                break;
            }

            result = mine_header(
                next_block.mining_request_id,
                &next_block.bytes.data,
                initial_randomness,
                &next_block.target,
                self.batch_size);

            initial_randomness += self.batch_size;

            // нужно выходить из цикла
            println!("randomless: {}", initial_randomness)
        }
    
        result
    }
}