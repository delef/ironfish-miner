use serde::{Deserialize, Serialize};

// #[derive]
// pub trait RpcRequest: Serialize {}
// pub trait RpcResponse<'de>: Deserialize<'de> {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "type")]
    pub _type: String,
    pub data: Request, // пока не дженерик
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub mid: usize,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(skip)] // временно скипаю т.к. с ним не работает
    pub data: Option<Vec<u8>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub id: u64,
    pub status: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Error {
    pub code: String,
    pub message: String,
    pub stack: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StreamResponse<T> {
    #[serde(rename = "type")]
    pub _type: String,
    pub data: StreamWrapper<T>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StreamWrapper<T> {
    pub id: u64,
    pub data: T,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MinerJobBytes {
    #[serde(rename = "type")]
    pub _type: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MinerJob {
    pub bytes: MinerJobBytes,
    #[serde(rename = "miningRequestId")]
    pub mining_request_id: u32,
    pub sequence: u64,
    pub target: String,
}
