use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};

use crate::data;
use crate::systems;
use crate::utility;
use crate::screens::screen::{Screen, ScreenTransition};
use crate::screens::home::Home;

pub struct Menu;

impl Menu {
    pub fn new() -> Self { Menu }
}

impl Screen for Menu {
    fn draw(&mut self, game: &mut systems::game_state::GameState) -> ScreenTransition {
        let sw = screen_width();
        let sh = screen_height();

        clear_background(data::constants::DEFAULT_BACKGROUND_COLOR);

        //New Game Button
        if widgets::Button::new("New Game").position(vec2(sw/2.0, sh/2.0)).size(vec2(200.0, 80.0)).ui(&mut root_ui()) {
            *game = systems::game_state::GameState::new();
            utility::file_control::save_game_json(game);
            return ScreenTransition::Goto(Box::new(Home::new()));
        }

        //Continue Button
        if widgets::Button::new("Continue").position(vec2(sw/2.0, sh/3.0)).size(vec2(200.0, 80.0)).ui(&mut root_ui()) {
            game.process_offline_progress();
            return ScreenTransition::Goto(Box::new(Home::new()));
        }

        ScreenTransition::Stay
    }
}