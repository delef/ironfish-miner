use std::{
    os::unix::net::UnixStream,
    io::prelude::*,
    io::Result,
    str,
    path::Path,
    time::Duration,
};

use super::types::{Request, Message, StreamResponse, MinerJob};

const IPC_DELIMITER: char = '\u{c}';
const READ_TIMEOUT_SECS: u64 = 1;

pub struct Ipc {
    socket: UnixStream,
}

impl Ipc {
	pub fn connect<P: AsRef<Path>>(path: P) -> Ipc {
        let socket = UnixStream::connect(path).expect("UnixStram client isn't connected");
        socket.set_read_timeout(Some(Duration::new(READ_TIMEOUT_SECS, 0))).expect("couldn't set read timeout");

        Self {
            socket: socket,
        }
	}

    pub fn request(&mut self, route: &str) -> Result<()> {
        let req = Request { _type: route.to_string(), mid: 0, data: None };
        self.emit("message", req)?;
        Ok(())
	}

    pub fn emit(&mut self, name: &str, data: Request) -> Result<()> {
        let message = Message { _type: name.to_string(), data: data };
        let mut json = serde_json::to_string(&message).unwrap();
        json.push(IPC_DELIMITER);

        self.socket.write_all(json.as_bytes())?;
        Ok(())
    }

    // todo: add timeout
    pub fn read_json_response(&mut self) -> Result<String> {
        let mut json = String::new();

        loop {
            // read from socket
            let mut buf = [0u8; 2048];
            self.socket.read(&mut buf[..])?;

            // save chank
            let s = str::from_utf8(&buf).expect("invalid UTF-8");
            json.push_str(&s);

            // not a complete answer
            let last_char = json.chars().last().unwrap();
            if last_char != '\u{0}' && last_char != IPC_DELIMITER {
                continue;
            }

            // trim whitespace
            let v: Vec<&str> = json.split(IPC_DELIMITER).collect();
            
            return Ok(String::from(v[0]));
        }
    }
}