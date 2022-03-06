use std::sync::mpsc::channel;
// use std::env;
use tokio::task;

mod rpc;
use rpc::{Ipc, types::MinerJob};

mod mining;
use mining::{Miner, WorkerFound};

#[tokio::main]
async fn main() {
	let path = "/Users/delef/Projects/crypto/.ironfish/ironfish.ipc";
	let mut client = Ipc::connect(path).await;
	
	// println!("{:?}", env::args());

	// base config (temporarily hardcode)
	let threads = 2;
	let batch_size = 1_000_000;

	// channels
	let (tasks_sender, tasks_reciver) = channel::<MinerJob>();
	let (found_sender, found_reciver) = channel::<WorkerFound>();

	// subscribe to node
	let tnode = task::spawn(async move {
		client.new_blocks_stream(tasks_sender).await;
	});

	// miner
	let tmining = task::spawn(async move {
		let miner = Miner::new(threads, batch_size);
		miner.run(tasks_reciver, found_sender);
	});

	// miner found receiver
	let tfound = task::spawn(async move {
		loop {
			let found = found_reciver.recv();
			println!("err: {:?}", found);
		}
	});

	let _ = tokio::join!(tnode, tmining, tfound);
}