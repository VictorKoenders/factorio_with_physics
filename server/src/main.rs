use shared::{ClientToServer, Receiver};
use std::{
    collections::HashMap,
    io::ErrorKind,
    net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, 12345)).unwrap();
    let mut clients = HashMap::new();
    listener.set_nonblocking(true).unwrap();
    loop {
        let mut had_update = false;
        match listener.accept() {
            Ok((socket, addr)) => {
                had_update = true;
                println!("[{:?}] connected", addr);
                clients.insert(addr, Client::new(addr, socket));
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {}
            Err(e) => {
                eprintln!("Could not receive new clients: {:?}", e);
                eprintln!("This is a bug");
            }
        }

        let mut clients_to_remove = Vec::new();

        for client in clients.values_mut() {
            match client.receive() {
                Ok(Some(e)) => {
                    had_update = true;
                    println!("[{:?}] {:?}", client.addr, e);
                }
                Ok(None) => {}
                Err(e) => {
                    eprintln!("Client {:?} disconnected: {:?}", client.addr, e);
                    clients_to_remove.push(client.addr);
                }
            }
        }

        for addr in clients_to_remove {
            clients.remove(&addr);
        }

        if !had_update {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }
}

struct Client {
    addr: SocketAddr,
    stream: TcpStream,
    receiver: Receiver<ClientToServer>,
}

impl Client {
    pub fn new(addr: SocketAddr, stream: TcpStream) -> Client {
        Client {
            addr,
            stream,
            receiver: Default::default(),
        }
    }

    pub fn receive(&mut self) -> Result<Option<ClientToServer>, ()> {
        self.receiver.receive(&mut self.stream)
    }
}
