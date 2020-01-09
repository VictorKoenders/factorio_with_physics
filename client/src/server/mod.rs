mod s_loop;
mod s_shared;

use parking_lot::RwLock;
use shared::{ClientToServer, ServerToClient};
use std::sync::Arc;

pub use self::{s_loop::*, s_shared::*};

pub struct Server {
    shared: Arc<RwLock<Shared>>,
}
impl Default for Server {
    fn default() -> Self {
        let shared = Arc::default();
        let cloned_shared = Arc::clone(&shared);
        ServerLoop::spawn(cloned_shared);
        Self { shared }
    }
}

impl Server {
    pub fn connection_status(&self) -> ConnectionStatus {
        self.shared.read().status
    }
    pub fn incoming(&mut self) -> Vec<ServerToClient> {
        std::mem::replace(&mut self.shared.write().incoming, Vec::new())
    }

    pub fn send(&mut self, message: impl Into<ClientToServer>) {
        if let Some(stream) = self.shared.write().stream.as_mut() {
            shared::encode_into(message.into(), stream);
        }
    }
}
