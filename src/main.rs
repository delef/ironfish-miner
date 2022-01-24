#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use tokio::task;
use tokio::sync::mpsc;

mod rpc;
use rpc::{Ipc, types::{NewBlocksResponse, NewBlocksBytes}};

mod mining;
use mining::Miner;

#[tokio::main(flavor = "current_thread")]
async fn main() {
	let path = "/Users/delef/Projects/crypto/.ironfish/ironfish.ipc";
	let mut client = Ipc::connect(path).await;
	
	// base config
	let threads = 16;
	let batch_size = 10_000;

	// channel
	let (new_blocks_sender, new_blocks_reciver) = mpsc::channel::<NewBlocksResponse>(10);

	// miner
	let miner = Miner::new(threads, batch_size);
	let mining_pool = miner.start(new_blocks_reciver);

	// subscribe to node
	let new_blocks_stream = task::spawn(async move {
		client.new_blocks_stream(new_blocks_sender).await;
	});

	// await threads
	let _ = tokio::join!(
		new_blocks_stream,
		mining_pool);
}