use std::io::{prelude::*, Error, ErrorKind, Result};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::str;

use serde::Serialize;

use super::types::{Message, Request};

const IPC_DELIMITER: char = '\u{c}';

pub struct Ipc {
    socket: UnixStream,
}

impl Ipc {
    pub fn connect<P: AsRef<Path>>(path: P) -> Ipc {
        let socket = UnixStream::connect(path).expect("UnixStram client isn't connected");
        socket
            .set_nonblocking(true)
            .expect("couldn't set nonblocking");

        Self { socket }
    }

    pub fn request<T: Serialize>(&mut self, route: &str, data: T) -> Result<()> {
        let req = Request::<T> {
            _type: route.to_string(),
            mid: 0,
            data: data,
        };
        self.emit("message", req)?;
        Ok(())
    }

    pub fn emit<T: Serialize>(&mut self, name: &str, data: Request<T>) -> Result<()> {
        let message = Message::<T> {
            _type: name.to_string(),
            data: data,
        };
        let mut json = serde_json::to_string(&message).unwrap();
        json.push(IPC_DELIMITER);

        self.socket.write_all(json.as_bytes())?;
        Ok(())
    }

    // todo: add timeout
    pub fn read_json(&mut self) -> Result<String> {
        // non-blocking read from socket
        let mut buf = [0u8; 4096];
        self.socket.read(&mut buf[..])?;

        // JSON is a UTF-8 string
        let json = str::from_utf8(&buf).expect("invalid UTF-8");

        // not a complete answer
        let last_char = json.chars().last().unwrap();
        if last_char != '\u{0}' && last_char != IPC_DELIMITER {
            return Err(Error::new(ErrorKind::OutOfMemory, "buffer overflow"));
        }

        // trim whitespace
        let v: Vec<&str> = json.split(IPC_DELIMITER).collect();

        return Ok(v[0].to_string());
    }
}
