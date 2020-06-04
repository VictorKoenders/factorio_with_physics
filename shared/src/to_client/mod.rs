use crate::{GalaxyId, PlayerId, SolarSystemId};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum ServerToClient {
    LoginResult(LoginResult),
    GalaxyList(Vec<GalaxySummary>),
    SolarSystemList(Vec<SolarSystemSummary>),
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum LoginResult {
    Success { player_info: PlayerInfo },
    Failed,
}

impl Into<ServerToClient> for LoginResult {
    fn into(self) -> ServerToClient {
        ServerToClient::LoginResult(self)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct PlayerInfo {
    pub id: PlayerId,
    pub name: String,
    pub x: f32,
    pub y: f32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GalaxySummary {
    pub id: GalaxyId,
    pub name: String,
    pub x: f32,
    pub y: f32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct SolarSystemSummary {
    pub id: SolarSystemId,
    pub name: String,
    pub angle_to_center: f32,
    pub distance_to_center: f32,
}

