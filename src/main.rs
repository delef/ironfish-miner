use clap::Parser;
use crossbeam_channel::{unbounded, Receiver, Sender};
use futures::future::join_all;
use log;
use num_cpus;
use tokio::task;

mod rpc;
use rpc::{types::MinerJob, Client};

mod mining;
use mining::{BlockFound, Miner};

mod cli;
use cli::Cli;

#[tokio::main]
async fn main() {
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
            client.parse_job_from_stream(&job_sender);
            client.found_handler(&found_reciver);
        }
    }));

    // miner thread
    let num_threads = if config.threads > 0 {
        config.threads as usize
    } else {
        num_cpus::get()
    };
    let batch_size = config.batch_size;
    handles.push(task::spawn(async move {
        log::info!(
            "Start miner with number of threads: {} and batch size = {}",
            num_threads,
            batch_size
        );

        let miner = Miner::new(num_threads, batch_size);
        miner.run(job_reciver, found_sender);
    }));

    join_all(handles).await;
}
