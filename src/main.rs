#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use num256::uint256::Uint256;

mod rpc;
use rpc::{Ipc, types::{NewBlocksResponse, NewBlocksBytes}};

mod mining;
use mining::Miner;

#[tokio::main(flavor = "current_thread")]
async fn main() {
	let path = "/Users/delef/Projects/crypto/.ironfish/ironfish.ipc";
	let mut client = Ipc::connect(path).await;
	
	// from ironfish config
	let threads = 1;
	let batch_size = 10_000;
	let miner = Miner::new(threads, batch_size);
	
	client.new_blocks_stream(move |next_block| {
		println!("new block: {:?}", next_block);
		let result = miner.mine(next_block);
		println!("Successfuly mined: {:?}", result);
	}).await;
}