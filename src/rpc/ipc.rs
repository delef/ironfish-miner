#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]


use std::{thread, str, path::Path};
use std::sync::mpsc::{Sender, Receiver};

use tokio::io::{AsyncWriteExt, AsyncReadExt};
use parity_tokio_ipc::{Endpoint, Connection};
use serde::{Deserialize, Serialize};
// use futures::executor::block_on;

use super::types::{IpcRequest, IpcMessage, IpcStreamResponse, NewBlocksResponse};
// use super::stream::{Stream};

const IPC_DELIMITER: char = '\u{c}';

// #[derive(Debug)]
pub struct Ipc {
    conn: Connection,
    pub message_id: usize,
}

impl Ipc {
	pub async fn connect<P: AsRef<Path>>(path: P) -> Ipc {
        Self {
            conn: Endpoint::connect(path).await.expect("UnixStram client isn't connected"),
            message_id: 0,
        }
	}

    pub async fn new_blocks_stream(&mut self, ch_sender: Sender<NewBlocksResponse>) {
        self.request("miner/newBlocksStream").await;

        loop {
            let json = self.read_json_from_socket().await;
            let s: IpcStreamResponse<NewBlocksResponse> = match serde_json::from_str(&json) {
                Ok(result) => result,
                Err(_) => panic!("invalid json: {}", json),
            };

            let new_block = s.data.data;
            if let Err(_) = ch_sender.send(new_block) {
				panic!("Error: new blocks receiver dropped");
			}
        }
    }

    async fn request(&mut self, route: &str) {
        let req = IpcRequest { _type: route.to_string(), mid: self.message_id, data: None };
        self.emit("message", req).await;
	}

    async fn emit(&mut self, name: &str, data: IpcRequest) {
        let message = IpcMessage { _type: name.to_string(), data: data };
        let mut json = serde_json::to_string(&message).unwrap();
        json.push(IPC_DELIMITER);
        self.conn.write_all(json.as_bytes()).await.expect("Unable to write message to client");
    }

    // нужно как-то останавливать стрим по запросу и таймауту (если долго нет данных)
    async fn read_json_from_socket(&mut self) -> String {
        let mut json = String::new();

        loop {
            // read from socket
            let mut buf = [0u8; 2048];
            self.conn.read(&mut buf[..]).await.expect("Unable to read buffer");

            // save chank
            let s = str::from_utf8(&buf).expect("Found invalid UTF-8");
            json.push_str(&s);

            // not a complete answer
            let last_char = json.chars().last().unwrap();
            if last_char != '\u{0}' && last_char != IPC_DELIMITER {
                continue;
            }

            // trim whitespace
            let v: Vec<&str> = json.split(IPC_DELIMITER).collect();
            
            return String::from(v[0]);
        }
    }
}