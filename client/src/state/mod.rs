mod layout;
mod text_input;

pub use self::{layout::Layout, text_input::TextInput};

use crate::{
    server::Server,
    window::{Color, HorizontalOffset, VerticalOffset, Window},
};
use shared::{to_client, to_server};

pub struct State {
    pub layout: Box<dyn Layout>,
    pub player: Option<Player>,
    pub galaxies: Vec<GalaxySummary>,
    pub galaxy: Option<Galaxy>,
    pub solar_systems: Vec<SolarSystemSummary>,
    pub solar_system: Option<SolarSystem>,
    pub last_mouse_position: Option<(i32, i32)>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            layout: Box::new(self::layout::login::Login::default()),
            player: None,
            galaxies: Vec::new(),
            galaxy: None,
            solar_systems: Vec::new(),
            solar_system: None,
            last_mouse_position: None,
        }
    }
}

impl State {
    pub fn render(&self, window: &mut Window, server: &Server) {
        self.layout.render(&self, window, server);

        if let Some(solar_system) = self.solar_system.as_ref() {
            self.render_solar_system(solar_system, window, server);
        } else if let Some(galaxy) = self.galaxy.as_ref() {
            self.render_galaxy(galaxy, window, server);
        } else {
            self.render_galaxy_overview(window, server);
        }
    }

    fn render_solar_system<'a>(
        &'a self,
        _solar_system: &'a SolarSystem,
        _window: &mut Window,
        _server: &Server,
    ) {
    }

    fn render_galaxy<'a>(&'a self, _galaxy: &'a Galaxy, _window: &mut Window, _server: &Server) {}

    fn render_galaxy_overview(&self, window: &mut Window, _server: &Server) {
        for galaxy in &self.galaxies {
            const SIZE: u32 = 10;
            const POS_OFFSET: f32 = (SIZE as f32) / 2.;
            if self.mouse_near((galaxy.data.x as i32, galaxy.data.y as i32), SIZE) {
                window.rect(
                    Color::white(),
                    (galaxy.data.x - POS_OFFSET * 2.) as i32,
                    (galaxy.data.y - POS_OFFSET * 2.) as i32,
                    SIZE * 2,
                    SIZE * 2,
                );
                window.ui_label_positioned(
                    Color::white(),
                    &galaxy.data.name,
                    galaxy.data.x as i32,
                    galaxy.data.y as i32 + SIZE as i32,
                    HorizontalOffset::Center,
                    VerticalOffset::Top,
                );
            } else {
                window.rect(
                    Color::white(),
                    (galaxy.data.x - POS_OFFSET) as i32,
                    (galaxy.data.y - POS_OFFSET) as i32,
                    SIZE,
                    SIZE,
                );
            }
        }
    }

    fn mouse_near(&self, (x, y): (i32, i32), distance: u32) -> bool {
        if let Some((mx, my)) = self.last_mouse_position.as_ref() {
            let dx = (x - mx).abs() as u32;
            let dy = (y - my).abs() as u32;
            dx < distance && dy < distance
        } else {
            false
        }
    }

    pub fn set_galaxies(&mut self, galaxies: Vec<to_client::GalaxySummary>) {
        self.galaxies.clear();
        self.galaxies.extend(galaxies.into_iter().map(Into::into));
    }

    pub fn set_solar_systems(&mut self, solar_systems: Vec<to_client::SolarSystemSummary>) {
        self.solar_systems.clear();
        self.solar_systems.reserve(solar_systems.len());
        for system in solar_systems {
            self.solar_systems.push(system.into());
        }
    }

    pub fn set_last_mouse_position(&mut self, (x, y): (i32, i32)) {
        self.last_mouse_position = Some((x, y));
    }

    pub fn set_player_info(&mut self, server: &mut Server, player_info: to_client::PlayerInfo){
        self.player = Some(player_info.into());
        server.send(to_server::ClientToServer::RequestGalaxyList);
    }
}

pub struct Player {
    data: to_client::PlayerInfo,
    x: f32,
    y: f32,
}

impl Player {
    pub fn id(&self) -> shared::PlayerId {
        self.data.id
    }

    pub fn name(&self) -> &str {
        &self.data.name
    }
}

impl From<to_client::PlayerInfo> for Player {
    fn from(p: to_client::PlayerInfo) -> Player {
        Player {
            x: p.x,
            y: p.y,
            data: p
        }
    }
}

pub struct Galaxy {}

pub struct GalaxySummary {
    data: to_client::GalaxySummary,
}

impl From<to_client::GalaxySummary> for GalaxySummary {
    fn from(data: to_client::GalaxySummary) -> Self {
        Self { data }
    }
}

pub struct SolarSystem {}

pub struct SolarSystemSummary {
    data: to_client::SolarSystemSummary,
}

impl From<to_client::SolarSystemSummary> for SolarSystemSummary {
    fn from(data: to_client::SolarSystemSummary) -> Self {
        Self { data }
    }
}
