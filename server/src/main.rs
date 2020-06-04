mod universe;

use shared::{to_client, to_server, ClientToServer, GalaxyId, Receiver, ServerToClient};
use std::{
    collections::HashMap,
    io::ErrorKind,
    net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    time::{Duration, Instant},
};
use universe::Universe;

fn main() {
    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, 12345)).unwrap();
    let mut universe = if std::path::Path::new("universe.json").exists() {
        Universe::load("universe.json")
    } else {
        Universe::default()
    };
    println!("{:?}", universe);
    let mut clients = HashMap::new();
    let mut last_save_moment = Instant::now();
    const SAVE_INTERVAL: Duration = Duration::from_secs(5);

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
                    let mut context = UpdateContext::default(); // (&mut universe);
                    client.update(e, &mut universe, &mut context);

                    // TODO: Handle replies and broadcasts in context
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

        if last_save_moment.elapsed() > SAVE_INTERVAL {
            universe.save("universe.json");
            last_save_moment = Instant::now();
        }
    }
}

#[derive(Default)]
struct UpdateContext {
    reply: Vec<ServerToClient>,
    broadcast: Vec<ServerToClient>,
}

impl UpdateContext {
    pub fn reply(&mut self, reply: impl Into<ServerToClient>) {
        self.reply.push(reply.into());
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

    pub fn update(
        &mut self,
        event: ClientToServer,
        universe: &mut Universe,
        context: &mut UpdateContext,
    ) {
        match event {
            ClientToServer::Login(login) => universe.update(context, login),
            ClientToServer::RequestGalaxyList => universe.request_galaxy_list(context),
            ClientToServer::RequestSolarSystemList { galaxy_id } => {
                universe.request_solar_list(context, galaxy_id)
            }
        }
    }
}

trait UniverseUpdate<T> {
    fn update(&mut self, context: &mut UpdateContext, ev: T);
}

trait UniverseExtensions {
    fn request_galaxy_list(&self, context: &mut UpdateContext);
    fn request_solar_list(&self, context: &mut UpdateContext, galaxy_id: GalaxyId);
}

impl UniverseUpdate<to_server::Login> for Universe {
    fn update(&mut self, context: &mut UpdateContext, login: to_server::Login) {
        let user = self.players.iter().find(|p| p.login_name == login.username);
        if let Some(user) = user {
            context.reply(to_client::LoginResult::Success {
                player_info: user.into(),
            });
        } else {
            context
                .reply
                .push(ServerToClient::LoginResult(to_client::LoginResult::Failed));
        }
    }
}

impl UniverseExtensions for Universe {
    fn request_galaxy_list(&self, context: &mut UpdateContext) {
        println!("TODO: request galaxy list");
    }
    fn request_solar_list(&self, context: &mut UpdateContext, galaxy_id: GalaxyId) {
        println!("TODO: request solar system list");
    }
}

