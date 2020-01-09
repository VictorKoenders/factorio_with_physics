use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum ServerToClient {
    LoginResult(LoginResult),
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum LoginResult {
    Success { player_info: PlayerInfo },
    Failed,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct PlayerInfo {
    pub id: Uuid,
    pub name: String,
    pub x: f32,
    pub y: f32,
}
