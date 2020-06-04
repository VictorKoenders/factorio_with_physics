use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Universe {
    pub players: Vec<Player>,
    pub galaxies: Vec<Galaxy>,
}

impl Universe {
    pub fn load(file: impl AsRef<Path>) -> Universe {
        serde_json::from_reader(File::open(file).unwrap()).unwrap()
    }

    pub fn save(&self, file: impl AsRef<Path>) {
        let file = File::create(file).unwrap();
        serde_json::to_writer_pretty(file, self).unwrap();
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    pub id: Uuid,
    pub login_name: String,
    pub display_name: String,
    pub password: String,
    pub planets: Vec<PlanetLocation>,
}

impl<'a> Into<shared::to_client::PlayerInfo> for &'a Player {
    fn into(self) -> shared::to_client::PlayerInfo {
        shared::to_client::PlayerInfo {
            id: shared::PlayerId(self.id),
            name: self.display_name.clone(),
            x: 0.0,
            y: 0.0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlanetLocation {
    pub galaxy: (Uuid, usize),
    pub solar_system: (Uuid, usize),
    pub planet: (Uuid, usize),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Galaxy {
    pub id: Uuid,
    pub name: String,
    pub solar_systems: Vec<SolarSystem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SolarSystem {
    pub id: Uuid,
    pub name: String,
    pub planets: Vec<Planet>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Planet {
    pub id: Uuid,
    pub name: String,
    pub player_id: Option<Uuid>,
}
