use tokio::io::{AsyncWriteExt};
use parity_tokio_ipc::{Endpoint, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct IpcStream {
	id: u64,
	data: Vec<u8>,
}

struct Stream {
    conn: &Connection,
}

impl Stream {
    pub fn new(conn: &Connection) -> Self {
        Self {conn: conn}
    }
}