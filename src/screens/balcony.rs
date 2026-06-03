use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};

use crate::data;
use crate::utility;
use crate::systems;
use crate::screens::screen::{Screen, ScreenTransition};

pub struct Balcony;

impl Balcony {
    pub fn new() -> Self { Balcony }
}

impl Screen for Balcony {
    fn draw(&mut self, game: &mut systems::game_state::GameState) -> ScreenTransition {
        let sw = screen_width();
        let sh = screen_height();

        clear_background(data::constants::DEFAULT_BACKGROUND_COLOR);

        //Save Button
        if widgets::Button::new("Save").position(vec2(sw*0.25, sh*0.25)).size(vec2(200.0, 80.0)).ui(&mut root_ui()) {
            utility::file_control::save_game_json(game);
            println!("[DBG]Saved Game");
        }

        //Flower Pot
        if widgets::Button::new("Flower Pot").position(vec2(sw/3.0, sh/2.0)).size(vec2(200.0, 80.0)).ui(&mut root_ui()) {
            println!("[DBG]Open some flower menu here");
        }

        //Vegetable Pot
        if widgets::Button::new("Vegetable Pot").position(vec2(sw/2.0, sh/2.0)).size(vec2(200.0, 80.0)).ui(&mut root_ui()) {
            println!("[DBG]Open some vegetable menu here");
        }


        ScreenTransition::Stay

    }
}