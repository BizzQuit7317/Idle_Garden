use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};

use crate::data;


pub fn draw() -> data::constants::Page {
    let sw = screen_width();
    let sh = screen_height();

    clear_background(data::constants::DEFAULT_BACKGROUND_COLOR);

    //Flower Pot
    if widgets::Button::new("Flower Pot").position(vec2(sw/3.0, sh/2.0)).size(vec2(200.0, 80.0)).ui(&mut root_ui()) {
        println!("Open some flower menu here");
    }

    //Vegetable Pot
    if widgets::Button::new("Vegetable Pot").position(vec2(sw/2.0, sh/2.0)).size(vec2(200.0, 80.0)).ui(&mut root_ui()) {
        println!("Open some vegetable menu here");
    }


    data::constants::Page::Balcony //Consstantly returning itself unless another page is returned

}