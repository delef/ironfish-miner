use clap::Parser;
use crossbeam_channel::unbounded;
use futures::future::join_all;
use log;
use std::time::Duration;
use tokio::{task, time::sleep};
use num_cpus;

mod rpc;
use rpc::{types::MinerJob, Client};

mod mining;
use mining::{BlockFound, Miner};

mod cli;
use cli::Cli;

#[tokio::main]
async fn main() {
    // init logger
    env_logger::init();

    // init cli
    let config = Cli::parse();

    // channels
    let (job_sender, job_reciver) = unbounded::<MinerJob>();
    let (found_sender, found_reciver) = unbounded::<BlockFound>();

    // handles
    let mut handles: Vec<task::JoinHandle<()>> = Vec::with_capacity(2);

    // node subscriber
    let ipc_path = config.ipc_path();
    handles.push(task::spawn(async move {
        let mut client = Client::new(ipc_path);
        client.stream_request();

        loop {
            client.send_mining_solution(&found_reciver);
            client.get_job(&job_sender);

            // 10 checks per second
            sleep(Duration::from_millis(100)).await;
        }
    }));

    // miner thread
    let num_threads = if config.threads > 0 { config.threads as usize } else { num_cpus::get() };
    let batch_size = config.batch_size;
    handles.push(task::spawn(async move {
        log::info!(
            "Starting to mine with {} threads, batch size: {}",
            num_threads,
            batch_size
        );

        let miner = Miner::new(num_threads, batch_size);
        miner.run(job_reciver, found_sender);
    }));

    join_all(handles).await;
}
