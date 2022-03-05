use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IpcMessage {
    #[serde(rename = "type")]
    pub _type: String,
    pub data: IpcRequest, // пока не дженерик
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IpcRequest {
	pub mid: usize,
	#[serde(rename = "type")]
	pub _type: String,
    #[serde(skip)] // временно скипаю т.к. с ним не работает
	pub data: Option<Vec<u8>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IpcResponse {
	pub id: u64,
	pub status: String,
	pub data: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IpcError {
	pub code: String,
	pub message: String,
    pub stack: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IpcStreamResponse<T> {
    #[serde(rename = "type")]
    pub _type: String,
    pub data: IpcStreamWrapper::<T>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IpcStreamWrapper<T> {
    pub id: u64,
    pub data: T,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewBlocksResponse {
    pub bytes: NewBlocksBytes,
    #[serde(rename = "miningRequestId")]
    pub mining_request_id: u32,
    pub sequence: u64,
    pub target: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct NewBlocksBytes {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}
