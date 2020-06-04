use serde::{de::DeserializeOwned, Serialize};
use std::{
    io::{ErrorKind, Read, Write},
    marker::PhantomData,
    net::TcpStream,
};

pub extern crate uuid;
pub extern crate serde;

mod ids;
pub use self::ids::*;

pub mod to_server;
pub use self::to_server::ClientToServer;

pub mod to_client;
pub use self::to_client::ServerToClient;

pub struct Receiver<T: DeserializeOwned> {
    buffer: Vec<u8>,
    _pd: PhantomData<T>,
}

impl<T: DeserializeOwned> Default for Receiver<T> {
    fn default() -> Self {
        Self {
            buffer: Vec::new(),
            _pd: PhantomData,
        }
    }
}

impl<T: DeserializeOwned> Receiver<T> {
    pub fn receive(&mut self, stream: &mut TcpStream) -> Result<Option<T>, ()> {
        loop {
            if let Ok(Some(msg)) = self.consume() {
                return Ok(Some(msg));
            }
            let mut buffer = [0u8; 1024];
            match stream.read(&mut buffer) {
                Ok(0) => return Ok(None),
                Ok(n) => {
                    self.buffer.extend(buffer.iter().take(n));
                    continue;
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => return Ok(None),
                Err(e) => {
                    eprintln!("Recv error {:?}", e);
                    return Err(());
                }
            }
        }
    }

    fn consume(&mut self) -> Result<Option<T>, ()> {
        use byteorder::{ByteOrder, NetworkEndian};

        if self.buffer.len() < 4 {
            return Ok(None);
        }
        let len = NetworkEndian::read_u32(&self.buffer[..4]) as usize;
        if len > self.buffer.len() {
            return Ok(None);
        }

        let msg = self.buffer.drain(..len).skip(4).collect::<Vec<_>>();

        match bincode::deserialize(&msg) {
            Ok(v) => Ok(Some(v)),
            Err(_) => Err(()),
        }
    }
}

pub fn encode_into<E: Serialize>(e: E, writer: &mut dyn Write) -> Result<(), ()> {
    use byteorder::{NetworkEndian, WriteBytesExt};

    let bytes = bincode::serialize(&e).map_err(|_| ())?;

    writer
        .write_u32::<NetworkEndian>(bytes.len() as u32 + 4)
        .map_err(|_| ())?;

    if writer.write_all(&bytes).is_err() {
        Err(())
    } else {
        Ok(())
    }
}

pub fn encode<E: Serialize>(e: E) -> Result<Vec<u8>, ()> {
    let mut buffer = Vec::new();
    encode_into(e, &mut buffer)?;
    Ok(buffer)
}
