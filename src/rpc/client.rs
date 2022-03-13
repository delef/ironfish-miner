use crossbeam_channel::{Receiver, Sender, TryRecvError};
use std::path::PathBuf;
use std::io::ErrorKind;

use super::ipc::Ipc;
use super::types::{MinerJob, StreamResponse};
use crate::mining::BlockFound;

pub struct Client {
    adapter: Ipc,
}

pub trait ClientVariants {
    fn init(self) -> Client;
}

impl Client {
    pub fn new<A>(args: A) -> Client
    where
        A: ClientVariants,
    {
        args.init()
    }

    // subscribe to events
    pub fn stream_request(&mut self) {
        self.adapter
            .request("miner/newBlocksStream")
            .expect("unable to write message to client");
    }

    pub fn get_job(&mut self, job_sender: &Sender<MinerJob>) {
        // try to get json from unix stream
        let json = match self.adapter.read_json() {
            // temporary
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => return,
            Err(e) => panic!("Can't read from soket. Err {}, kind {:?}", e, e.kind()),
            Ok(v) => v,
        };

        // deserialize json
        let response: StreamResponse<MinerJob> = match serde_json::from_str(&json) {
            Ok(result) => result,
            Err(_) => return, // isn't MinerJob response
        };

        let new_job = response.data.data;
        log::info!("new job: {:?}", &new_job);
        job_sender.send(new_job).expect("new job receiver dropped");
    }

    pub fn send_mining_solution(&mut self, found_reciver: &Receiver<BlockFound>) {
        loop {
            let block_found: BlockFound = match found_reciver.try_recv() {
                Ok(v) => v,
                Err(e) => {
                    if e == TryRecvError::Empty {
                        break;
                    }
                    panic!("block_found err: {}", e);
                }
            };

            println!("found block: {:?}", block_found);
            break;
        }
    }
}

/// IPC connection (UnixStream)
impl ClientVariants for PathBuf {
    fn init(self) -> Client {
        Client {
            adapter: Ipc::connect(self),
        }
    }
}

/// RPC connection
impl ClientVariants for String {
    fn init(self) -> Client {
        panic!("currently unsupported")
    }
}
