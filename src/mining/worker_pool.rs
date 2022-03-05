use std::{thread, time::Duration};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
    mpsc::{channel, Sender, Receiver, TryRecvError}
};
use parking_lot::RwLock;
use rand::Rng;

use super::mine_batch;

#[derive(Debug)]
pub struct WorkerPool {
    threads: Vec<thread::JoinHandle<()>>,
    channels: Vec<Sender<WorkerCmd>>,
}

#[derive(Clone, Debug)]
pub struct WorkerJobData {
    pub bytes: Vec<u8>,
    pub target: [u8; 32],
    pub request_id: u32,
    pub sequence: u64,
    pub randomness: Arc<AtomicUsize>,
}

#[derive(Debug)]
enum WorkerCmd {
    Job { job_data: WorkerJobData },
    Stop,
}

#[derive(Debug)]
enum WorkerExit {
    NonceSpaceExhausted,
    Stopped,
}

impl WorkerPool {
    pub fn new(num_threads: usize, batch_size: usize) -> WorkerPool {
        let mut threads: Vec<thread::JoinHandle<()>> = Vec::with_capacity(num_threads);
        let mut channels: Vec<Sender<WorkerCmd>> = Vec::with_capacity(num_threads);
        
        for thread_id in 0..num_threads {
            let (sndr, rcvr): (Sender<WorkerCmd>, Receiver<WorkerCmd>) = channel();

            let thread = thread::Builder::new()
                .name(format!("worker thread {}", thread_id))
                .spawn(move || worker_thread(rcvr, thread_id, batch_size))
                .expect("worker thread handle");
    
            threads.push(thread);
            channels.push(sndr);
        }
    
        WorkerPool {
            threads: threads,
            channels: channels,
        }
    }

    pub fn num_threads(&self) -> usize {
        self.threads.len()
    }

    pub fn new_job(&self, bytes: Vec<u8>, target: [u8; 32], request_id: u32, sequence: u64) {
        let rnd: usize = rand::thread_rng().gen();
        let initial_randomness = Arc::new(AtomicUsize::new(rnd));

        for ch in self.channels.iter() {
            ch.send(WorkerCmd::Job {
                job_data: WorkerJobData {
                    bytes: bytes.clone(),
                    target: target.clone(),
                    request_id: request_id,
                    sequence: sequence,
                    randomness: initial_randomness.clone(),
                }
            }).expect("sending new job command");
        }
    }
}

fn worker_thread(
    receiver: Receiver<WorkerCmd>,
    thread_id: usize,
    batch_size: usize,
) {
    let mut job: Option<WorkerJobData> = None;

    loop {
        match receiver.try_recv() {
            Ok(value) => match value {
                WorkerCmd::Job { job_data } => job = Some(job_data),
                WorkerCmd::Stop => break, // stop thread
            },
            Err(TryRecvError::Empty) => if job.is_none() { continue },
            Err(TryRecvError::Disconnected) => panic!("job channel was dropped"),
        };

        if let Some(job_data) = &job {
            let initial_randomness = job_data.randomness.fetch_add(batch_size, Ordering::SeqCst);

            let match_found = mine_batch(
                &mut job_data.bytes.clone(),
                &job_data.target,
                initial_randomness,
                batch_size,
            );
            
            if let Some(randomness) = match_found {
                println!("found. randomness: {}", randomness);
            }
        }
    }

    println!("Worker thread stopped. Thread ID: {}", thread_id);
}