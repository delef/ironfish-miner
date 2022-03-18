use std::io::{stdout, Write};
use std::iter;
use std::time::Duration;

use clap::Parser;
use crossbeam_channel::unbounded;
use futures::future::join_all;
use log;
use num_cpus;
use tokio::{task, time};

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
    let (metric_sender, metric_reciver) = unbounded::<usize>();

    // handles
    let mut handles: Vec<task::JoinHandle<()>> = Vec::with_capacity(2);

    // node subscriber
    let ipc_path = config.ipc_path();
    handles.push(task::spawn(async move {
        let mut client = Client::new(ipc_path);
        client.stream_request().unwrap();

        loop {
            client.send_mining_solution(&found_reciver).unwrap();
            client.get_job(&job_sender).unwrap();

            // 200 checks per second
            time::sleep(Duration::from_millis(5)).await;
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
            "Starting to mine with {} threads, batch size: {}",
            num_threads,
            batch_size
        );

        let miner = Miner::new(num_threads, batch_size);
        miner.run(job_reciver, found_sender, metric_sender);
    }));

    // metrics
    handles.push(task::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            let mut hr_vals = Vec::<usize>::with_capacity(100);
            for thread_hr in metric_reciver.try_iter() {
                hr_vals.push(thread_hr);
            }
            let sum = hr_vals.iter().filter(|&n| *n > 0).sum::<usize>();

            let mut stdout = stdout();
            let h = match sum {
                n if n < 1_000 => format!("{} H/S", n),
                n if n < 1_000_000 => format!("{} TH/S", (n as f64 / 1_000 as f64)),
                n if n < 1_000_000_000 => format!("{} MH/S", (n as f64 / 1_000_000 as f64)),
                n if n > 1_000_000_000 => format!("{} GH/S", (n as f64 / 1_000_000_000 as f64)),
                _ => panic!("unknown hashrate value"),
            };

            print!("\r {}{}", h, iter::repeat(" ").take(10).collect::<String>());
            stdout.flush().unwrap();
        }
    }));

    join_all(handles).await;
}
