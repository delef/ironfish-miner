#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use tokio::io::{AsyncWriteExt, AsyncReadExt};
use parity_tokio_ipc::{Endpoint, Connection};
use serde::{Deserialize, Serialize};
use std::str;

// todo: можно сделать свой Serialize/Deserialize с делимитером \f
#[derive(Debug, Serialize, Deserialize)]
pub struct IpcMessage {
    #[serde(rename = "type")]
    _type: String,
    data: IpcRequest, // пока не дженерик
}
#[derive(Debug, Deserialize, Serialize)]
pub struct IpcRequest {
	mid: usize,
	#[serde(rename = "type")]
	_type: String,
    #[serde(skip)] // временно скипаю т.к. с ним не работает
	data: Option<Vec<u8>>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct IpcResponse {
	id: u64,
	status: String,
	data: Vec<u8>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct IpcError {
	code: String,
	message: String,
    stack: String,
}

// part of stream.rs for test
// todo: пока не дженерик
#[derive(Debug, Deserialize, Serialize)]
pub struct IpcStreamResponse {
    #[serde(rename = "type")]
    _type: String,
    data: IpcStreamWrapper,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct IpcStreamWrapper {
    id: u64,
    data: IpcStreamData,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct IpcStreamData {
    bytes: IpcStreamBytes,
    #[serde(rename = "miningRequestId")]
    mining_request_id: u64,
    sequence: u64,
    target: String,
}
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct IpcStreamBytes {
    #[serde(rename = "type")]
    _type: String,
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
}

const IPC_DELIMITER: char = '\x0C';

// #[derive(Debug)]
pub struct Ipc {
    conn: Connection,
    message_id: usize,
}

impl Ipc {
	pub async fn connect(path: &'static str) -> Self {
		Self {
            conn: Endpoint::connect(path).await.expect("Failed to connect client."),
            message_id: 1,
        }
	}

    pub async fn request(&mut self, route: &str) -> &Connection {
        let req = IpcRequest { _type: route.to_string(), mid: self.message_id, data: None };
        self.emit("message", req).await;
        &self.conn
	}

    pub async fn emit(&mut self, name: &str, data: IpcRequest) {
        let message = IpcMessage { _type: name.to_string(), data: data };
        let mut json = serde_json::to_string(&message).unwrap();
        json.push(IPC_DELIMITER);
        println!("debug: {}", json);
        self.conn.write_all(json.as_bytes()).await.expect("Unable to write message to client");
    }

    // нужно как-то останавливать стрим по запросу и таймауту (если долго нет данных)
    pub async fn stream(&mut self, callback: fn(IpcStreamResponse)) {
        let mut json = String::new();

        loop {
            // read from socket
            let mut buf = [0u8; 2048];
            self.conn.read(&mut buf[..]).await.expect("Unable to read buffer");

            // save chank
            let s = str::from_utf8(&buf).expect("Found invalid UTF-8");
            json.push_str(&s);

            // local delimiter
            let ldelim = '\u{c}';

            // not a complete answer
            let last_char = json.chars().last().unwrap();
            if last_char != '\u{0}' && last_char != ldelim {
                continue;
            }

            // clear of debris
            let v: Vec<&str> = json.split(ldelim).collect();
            println!("\n\nlen: {}\n\n", v[0].len());
            json = String::from(v[0]);

            let stream: IpcStreamResponse = match serde_json::from_str(&json) {
                Ok(stream) => stream,
                Err(_) => {
                    json = String::new();
                    continue;
                },
            };
            callback(stream);

            // reset data
            json = String::new();
        }
    }
}