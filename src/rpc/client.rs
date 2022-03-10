use std::path::PathBuf;
use crossbeam_channel::{Receiver, Sender};
use std::io::ErrorKind;

use super::ipc::Ipc;
use super::types::{StreamResponse, MinerJob};
use crate::mining::BlockFound;

pub struct Client {
    adapter: Ipc,
}

pub trait ClientVariants {
    fn init(self) -> Client;
}

impl Client {
    pub fn new<A>(args: A) -> Client
        where A: ClientVariants
    {
        args.init()
    }

    // subscribe to events
    pub fn stream_request(&mut self) {
        self.adapter.request("miner/newBlocksStream").expect("unable to write message to client");
    }

    pub fn parse_job_from_stream(&mut self, job_sender: &Sender<MinerJob>) {
        // try to get json from unix stream
        let json = match self.adapter.read_json_response() {
            Err(e) => {
                if e.kind() == ErrorKind::WouldBlock {
                    println!("skiped");
                    return
                }
                panic!("Can't read from soket. Err {}, kind {:?}", e, e.kind());
            },
            Ok(v) => v,
        };

        // deserialize json
        let response: StreamResponse<MinerJob> = match serde_json::from_str(&json) {
            Ok(result) => result,
            Err(_) => return, // isn't MinerJob response
        };

        let new_job = response.data.data;
        println!("new job: {:?}", &new_job);
        job_sender.send(new_job).expect("new job receiver dropped");
    }

    pub fn found_handler(&mut self, found_reciver: &Receiver<BlockFound>) {
        loop {
            let block_found = found_reciver.recv();

            println!("send found: {:?}", block_found);
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