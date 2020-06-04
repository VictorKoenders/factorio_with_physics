use crate::GalaxyId;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum ClientToServer {
    Login(Login),
    RequestGalaxyList,
    RequestSolarSystemList { galaxy_id: GalaxyId },
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Login {
    pub username: String,
    pub password: String,
}

impl Into<ClientToServer> for Login {
    fn into(self) -> ClientToServer {
        ClientToServer::Login(self)
    }
}
