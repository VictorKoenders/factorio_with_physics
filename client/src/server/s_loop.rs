use super::Shared;
use parking_lot::RwLock;
use shared::{Receiver, ServerToClient};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream},
    sync::Arc,
    thread,
    time::Duration,
};

#[derive(Default)]
pub struct ServerLoop {
    shared: Arc<RwLock<Shared>>,
    stream: Option<TcpStream>,
    receiver: Receiver<ServerToClient>,
}

impl ServerLoop {
    pub fn spawn(shared: Arc<RwLock<Shared>>) {
        thread::spawn(move || {
            let mut s_loop = ServerLoop {
                shared,
                ..Default::default()
            };

            while s_loop.shared.read().is_running {
                s_loop.update();
            }
            eprintln!("Server connection loop shut down");
        });
    }

    fn update(&mut self) {
        if let Some(stream) = &mut self.stream {
            while let Ok(msg) = self.receiver.receive(stream) {
                if let Some(msg) = msg {
                    self.shared.write().incoming.push(msg);
                }
            }

            self.stream = None;
            self.shared.write().clear_stream();
        } else {
            eprintln!("Attempting to connect to server...");
            self.shared.write().set_connecting();

            let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 12345);
            match TcpStream::connect_timeout(&addr, Duration::from_secs(30)) {
                Ok(s) => {
                    s.set_read_timeout(Some(Duration::from_millis(10))).unwrap();
                    self.shared.write().set_stream(s.try_clone().ok());
                    self.stream = Some(s);
                }
                Err(e) => {
                    // Don't need to reset `shared.stream` because it should already be None
                    self.shared.write().clear_stream();
                    eprintln!(
                        "Cannot connect to server, trying again in 5 seconds ({:?})",
                        e
                    );
                    thread::sleep(Duration::from_secs(5));
                }
            }
        }
    }
}
