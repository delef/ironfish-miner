#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::sync::mpsc::channel;

use tokio::task;

mod rpc;
use rpc::{Ipc, types::{NewBlocksResponse, NewBlocksBytes}};

mod mining;
use mining::Miner;

use tokio::runtime;

#[tokio::main]
async fn main() {
	let path = "/Users/delef/Projects/crypto/.ironfish/ironfish.ipc";
	let mut client = Ipc::connect(path).await;
	
	// base config (temporarily hardcode)
	let threads = 2;
	let batch_size = 1_000_000;

	// channel
	let (tasks_sender, tasks_reciver) = channel::<NewBlocksResponse>();

	// subscribe to node
	let node_thread = task::spawn(async move {
		client.new_blocks_stream(tasks_sender).await;
	});

	// miner
	let mining_thread = task::spawn(async move {
		let miner = Miner::new(threads, batch_size);
		miner.start(tasks_reciver);
	});

	let _ = tokio::join!(node_thread, mining_thread);
}