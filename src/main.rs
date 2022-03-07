use std::sync::mpsc::channel;
use std::env;
use log::{info};
use tokio::task;

mod rpc;
use rpc::{Ipc, types::MinerJob};

mod mining;
use mining::{Miner, BlockFound};

mod config;
use config::Config;

#[tokio::main]
async fn main() {
	let path = "/Users/delef/Projects/crypto/.ironfish/ironfish.ipc";
	let mut client = Ipc::connect(path).await;
	
	// parse env vars
	let env_args: Vec<String> = env::args().collect();
	let config = Config::new(&env_args);

	// channels
	let (tasks_sender, tasks_reciver) = channel::<MinerJob>();
	let (found_sender, found_reciver) = channel::<BlockFound>();

	// subscribe to node
	let tnode = task::spawn(async move {
		client.new_blocks_stream(tasks_sender).await;
	});

	// miner
	let num_threads = config.num_threads;
	let batch_size = config.batch_size;
	let tmining = task::spawn(async move {
		println!("Start miner with number of threads: {} and batch size = {}", num_threads, batch_size);

		let miner = Miner::new(num_threads, batch_size);
		miner.run(tasks_reciver, found_sender);
	});

	// miner found receiver
	let tfound = task::spawn(async move {
		loop {
			let block_found = found_reciver.recv().expect("can't get data from found channel");
			info!("Found block randomness: {:?}", block_found.randomness);
		}
	});

	let _ = tokio::join!(tnode, tmining, tfound);
}