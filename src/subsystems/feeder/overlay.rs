use macroquad::prelude::*;
use macroquad::ui::Ui;
use macroquad::ui::{root_ui, widgets};
use crate::subsystems::{ResourceContext, SubsystemOutput};
use super::FeederSystem;

pub fn draw(ui: &mut Ui, feeder: &mut FeederSystem, ctx: &ResourceContext) { 
    let sw = screen_width();
    let sh = screen_height();
    draw_rectangle(sw * 0.1, sh * 0.1, sw * 0.8, sh * 0.8, BLUE);
    draw_text("Feeder System", sw * 0.15, sh * 0.2, 32.0, WHITE);
}