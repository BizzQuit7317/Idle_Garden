use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};

pub fn draw() { 
    let sw = screen_width();
    let sh = screen_height();
    draw_rectangle(sw * 0.1, sh * 0.1, sw * 0.8, sh * 0.8, DARKGRAY);
    draw_text("Bed System", sw * 0.15, sh * 0.2, 32.0, WHITE);
}