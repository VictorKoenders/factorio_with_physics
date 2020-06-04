mod server;
mod state;
mod window;

use server::Server;
use shared::{ServerToClient, to_client::LoginResult};
use state::State;
use std::time::Instant;
use window::{Color, Event, Window};

fn main() {
    let mut server = Server::default();
    let mut state = State::default();
    let mut window = Window::default();

    let mut last_fps_times = Vec::with_capacity(20);

    while window.is_open() {
        let start = Instant::now();
        for event in window.events() {
            match event {
                Event::CloseRequested => {
                    window.close();
                }
                Event::Tab => {
                    state.layout.tab_pressed(&window);
                }
                Event::Enter => {
                    state.layout.enter_pressed(&window, &mut server);
                }
                Event::Backspace => {
                    state.layout.backspace_pressed();
                }
                Event::Click { x, y } => {
                    state.layout.click((x, y), &window, &mut server);
                }
                Event::TextInput { text } => {
                    state.layout.text_input(text);
                }
                Event::MouseMove { x, y } => {
                    state.set_last_mouse_position((x, y));
                }
            }
        }
        for ev in server.incoming() {
            match ev {
                ServerToClient::LoginResult(LoginResult::Success { player_info }) => {
                    state.set_player_info(&mut server, player_info);
                }
                ServerToClient::LoginResult(LoginResult::Failed) => {
                    println!("Login failed");
                }
                ServerToClient::GalaxyList(galaxies) => state.set_galaxies(galaxies),
                ServerToClient::SolarSystemList(systems) => state.set_solar_systems(systems),
            }
        }

        window.clear(Color::black());
        state.render(&mut window, &server);

        window.finish();

        let elapsed = start.elapsed();
        last_fps_times.push(elapsed);

        if last_fps_times.len() == last_fps_times.capacity() {
            let total_frame_time = last_fps_times
                .drain(..)
                .map(|t| t.as_secs_f32())
                .sum::<f32>();
            let average_frame_time = total_frame_time / last_fps_times.capacity() as f32;
            window.set_title(format!("WildHunter {:.0} fps", 1.0 / average_frame_time));
            last_fps_times.clear();
        }
    }
}
