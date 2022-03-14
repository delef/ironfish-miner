use crossbeam_channel::{Receiver, Sender};
use std::io::{Error, ErrorKind, Result};
use std::path::PathBuf;

use super::ipc::Ipc;
use super::types::{MinerJob, MinerSuccessfullyMined, StreamResponse};
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
    pub fn stream_request(&mut self) -> Result<()> {
        self.adapter
            .request("miner/newBlocksStream", "".to_string())
    }

    pub fn get_job(&mut self, job_sender: &Sender<MinerJob>) -> Result<()> {
        // try to get json from unix stream
        let json = match self.adapter.read_json() {
            Ok(v) => v,
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => return Ok(()), // wait next
            Err(e) => return Err(e),
        };

        // deserialize json
        let response: StreamResponse<MinerJob> = match serde_json::from_str(&json) {
            Ok(v) => v,
            Err(_) => return Ok(()), // isn't MinerJob response
        };

        let new_job = response.data.data;
        log::info!("new job: {:?}", &new_job);
        job_sender.send(new_job).expect("new job receiver dropped");

        Ok(())
    }

    pub fn send_mining_solution(&mut self, found_reciver: &Receiver<BlockFound>) -> Result<()> {
        let block_found: BlockFound = match found_reciver.try_recv() {
            Ok(v) => v,
            Err(ref e) if e.is_empty() => return Ok(()),
            Err(_) => {
                return Err(Error::new(
                    ErrorKind::ConnectionAborted,
                    "block_found channel was dropped",
                ))
            }
        };

        // message { mid: 2, type: 'miner/successfullyMined', data: { randomness: 12312312312, miningRequestId: 1 } }
        let mined_block = MinerSuccessfullyMined {
            randomness: block_found.randomness,
            mining_request_id: block_found.mining_request_id,
        };
        self.adapter
            .request("miner/successfullyMined", mined_block)?;

        Ok(())
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
