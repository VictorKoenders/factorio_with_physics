use shared::ServerToClient;
use std::net::TcpStream;

pub struct Shared {
    pub status: ConnectionStatus,
    pub stream: Option<TcpStream>,
    pub incoming: Vec<ServerToClient>,
    pub is_running: bool,
}

impl Default for Shared {
    fn default() -> Self {
        Shared {
            status: ConnectionStatus::Disconnected,
            stream: None,
            incoming: Vec::new(),
            is_running: true,
        }
    }
}

impl Shared {
    pub fn clear_stream(&mut self) {
        self.status = ConnectionStatus::Disconnected;
        self.stream = None;
    }
    pub fn set_stream(&mut self, stream: Option<TcpStream>) {
        self.status = ConnectionStatus::Connected;
        self.stream = stream;
    }
    pub fn set_connecting(&mut self) {
        debug_assert!(self.stream.is_none());
        self.status = ConnectionStatus::Connecting;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ConnectionStatus {
    Connected,
    Connecting,
    Disconnected,
}
