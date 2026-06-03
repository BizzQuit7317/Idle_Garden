use macroquad::prelude::*;

mod data;
mod screens;
mod systems;
mod utility;

//#[macroquad::main("Idle Garden")]
fn window_conf() -> Conf {
    Conf {
        window_title: "Idle Garden".to_string(),
        window_width: data::constants::WINDOWS_DEFAULT_WIDTH,
        window_height: data::constants::WINDOWS_DEFAULT_LENGTH,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut current_page: data::constants::Page = data::constants::Page::Menu; //Keep track of the current page
    //let mut game: systems::game_state::GameState = systems::game_state::GameState::new();
    let mut game: systems::game_state::GameState = utility::file_control::load_game_json(); //try and load a previous save by default, no save will create a new one.

    loop {

        match current_page {
            data::constants::Page::Menu => { current_page = screens::menu::draw(&mut game); },
            data::constants::Page::Balcony => { current_page = screens::balcony::draw(); },
            _ => { current_page = screens::default::draw(); } //arm for Default
        }

        next_frame().await;
    }
}