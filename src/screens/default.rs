use macroquad::prelude::*;

use crate::data;

pub fn draw() -> data::constants::Page  {
    clear_background(data::constants::DEFAULT_BACKGROUND_COLOR);

    data::constants::Page::Default //Consstantly returning itself unless another page is returned
}