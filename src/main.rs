use macroquad::prelude::*;

use crate::screens::screen::{Screen, ScreenTransition};
use crate::screens::menu::Menu;

mod data;
mod screens;
mod systems;
mod utility;
mod subsystems;

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
    let mut game: systems::game_state::GameState = utility::file_control::load_game_json(); //try and load a previous save by default, no save will create a new one.
    let mut current_screen: Box<dyn Screen> = Box::new(Menu::new());
    let mut ticking = false;

    loop {
        let dt = get_frame_time() as f64;

        if ticking {
            game.tick_accumulator += dt;
            while game.tick_accumulator >= game.tick_rate {
                game.tick_accumulator -= game.tick_rate;
                game.tick();
            }
        }

        match current_screen.draw(&mut game) {
            ScreenTransition::Stay => {}
            ScreenTransition::Goto(next) => {
                current_screen = next;
                ticking = true; //start ticking once we leave the menu
            },
        }

        next_frame().await;
    }
}