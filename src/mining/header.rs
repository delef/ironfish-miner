#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use num256::Uint256;
use blake3;

#[derive(Debug)]
pub struct MineRusult {
    pub initial_randomness: u64,
    pub randomness: Option<u64>,
    pub mining_request_id: Option<u32>,
}

impl MineRusult {
    pub fn empty() -> Self {
        MineRusult {
            initial_randomness: 0,
            randomness: None,
            mining_request_id: None,
        }
    }

    pub fn found(&self) -> bool {
        self.randomness.is_some() && self.mining_request_id.is_some()
    }
}

pub fn mine_header(
    mining_request_id: u32,
    header_bytes_without_randomness: &[u8],
    initial_randomness: u64,
    target_value: &Uint256,
    batch_size: u64,
) -> MineRusult {
    let rand_len = 8;
    let header_len = header_bytes_without_randomness.len() + rand_len;
    let mut header_bytes: Vec<u8> = vec![0u8; header_len];

    // buf with empty prefix
    header_bytes.splice(rand_len.., header_bytes_without_randomness.iter().cloned());
    
    for i in 0..batch_size {
        // The intention here is to wrap randomness between 0 inclusive and u64::MAX inclusive
        let randomness =
            if i > u64::MAX {
                i - (u64::MAX - initial_randomness) - 1
            } else {
                initial_randomness + i
            };
        
        // add randomness to begin of slice
        header_bytes.splice(..rand_len, randomness.to_ne_bytes());

        // work with hash
        let block_hash = hash_block_header(&header_bytes);
        let hash_value = Uint256::from_bytes_le(block_hash.as_bytes());

        // successfuly mined
        if hash_value <= *target_value {
            return MineRusult {
                initial_randomness: initial_randomness,
                randomness: Some(randomness),
                mining_request_id: Some(mining_request_id),
            };
        }
    }

    MineRusult {
        initial_randomness: initial_randomness,
        randomness: None,
        mining_request_id: None,
    }
}

fn hash_block_header(header_bytes: &[u8]) -> blake3::Hash {
    blake3::hash(header_bytes)
}