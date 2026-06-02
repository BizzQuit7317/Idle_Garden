use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};

use crate::data;

pub enum MenuChoice {
    NewGame,
    Continue,
    Exit
}

pub fn draw() -> data::constants::Page {
    let sw = screen_width();
    let sh = screen_height();

    clear_background(data::constants::DEFAULT_BACKGROUND_COLOR);

    //New Game Button
    if widgets::Button::new("New Game").position(vec2(sw/2.0, sh/2.0)).size(vec2(200.0, 80.0)).ui(&mut root_ui()) {
        return data::constants::Page::Default
    }

    //Continue Button
    if widgets::Button::new("Continue").position(vec2(sw/2.0, sh/3.0)).size(vec2(200.0, 80.0)).ui(&mut root_ui()) {
        return data::constants::Page::Default
    }


    data::constants::Page::Menu //Consstantly returning itself unless another page is returned

}