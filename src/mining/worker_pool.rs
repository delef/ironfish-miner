use crossbeam_channel::{unbounded, Receiver, Sender, TryRecvError};
use log::info;
use rand::Rng;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::thread;

use super::mine::mine_batch;

#[derive(Clone, Debug)]
pub struct WorkerJobData {
    pub bytes: Vec<u8>,
    pub target: [u8; 32],
    pub mining_request_id: u32,
    pub sequence: u64,
    pub randomness: Arc<AtomicUsize>,
}

#[derive(Copy, Clone, Debug)]
pub struct BlockFound {
    pub mining_request_id: u32,
    pub randomness: usize,
}

#[derive(Debug)]
enum WorkerCmd {
    Job { job_data: WorkerJobData },
    Stop,
}

#[derive(Debug)]
pub struct WorkerPool {
    job_channels: Vec<Sender<WorkerCmd>>,
}

impl WorkerPool {
    pub fn new(
        num_threads: usize,
        batch_size: usize,
        found_sender: Sender<BlockFound>,
        metric_sender: Sender<usize>,
    ) -> WorkerPool {
        let mut job_channels: Vec<Sender<WorkerCmd>> = Vec::with_capacity(num_threads);

        for _thread_id in 0..num_threads {
            let (job_sndr, job_rcvr) = unbounded::<WorkerCmd>();
            let found_sndr = found_sender.clone();
            let metric_sndr = metric_sender.clone();

            thread::Builder::new()
                .name(format!("worker pool thread"))
                .spawn(move || worker_thread(job_rcvr, found_sndr, metric_sndr, batch_size))
                .expect("worker thread handle");

            job_channels.push(job_sndr);
        }

        WorkerPool {
            job_channels: job_channels,
        }
    }

    pub fn new_job(&self, bytes: Vec<u8>, target: [u8; 32], req_id: u32, sequence: u64) {
        let rnd: usize = rand::thread_rng().gen();
        let initial_randomness = Arc::new(AtomicUsize::new(rnd));

        for ch in self.job_channels.iter() {
            ch.send(WorkerCmd::Job {
                job_data: WorkerJobData {
                    bytes: bytes.clone(),
                    target: target.clone(),
                    mining_request_id: req_id,
                    sequence: sequence,
                    randomness: initial_randomness.clone(),
                },
            })
            .expect("sending new job command");
        }
    }

    #[allow(unused)]
    pub fn stop(&self) {
        info!("stopping workers");

        for tx in self.job_channels.iter() {
            tx.send(WorkerCmd::Stop)
                .expect("job_channel receiver dropped");
        }
    }
}

fn worker_thread(
    job_receiver: Receiver<WorkerCmd>,
    found_sender: Sender<BlockFound>,
    metric_sender: Sender<usize>,
    batch_size: usize,
) {
    let thread_id = thread::current().id();
    let mut job: Option<WorkerJobData> = None;

    info!("Worker thread started. Thread ID: {:?}", thread_id);

    loop {
        match job_receiver.try_recv() {
            Ok(value) => match value {
                WorkerCmd::Job { job_data } => job = Some(job_data),
                WorkerCmd::Stop => break, // stop thread
            },
            Err(TryRecvError::Empty) => {
                if job.is_none() {
                    continue;
                }
            }
            Err(TryRecvError::Disconnected) => panic!("job channel was dropped"),
        };

        if let Some(job_data) = &job {
            // todo: check for overflow
            let randomness = job_data.randomness.fetch_add(batch_size, Ordering::SeqCst);

            let match_found = mine_batch(
                &mut job_data.bytes.clone(),
                &job_data.target,
                randomness,
                batch_size,
            );
            metric_sender.send(batch_size).unwrap();

            if let Some(randomness_found) = match_found {
                info!("found. randomness: {}", randomness_found);

                found_sender
                    .send(BlockFound {
                        mining_request_id: job_data.mining_request_id,
                        randomness: randomness_found,
                    })
                    .expect("block found receiver dropped");
            }
        }
    }

    info!("Worker thread stopped. Thread ID: {:?}", thread_id);
}
