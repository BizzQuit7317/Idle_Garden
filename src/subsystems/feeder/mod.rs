use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};
use serde::{Serialize, Deserialize};
use crate::subsystems::{Subsystem, SubsystemRegistration};

#[derive(Debug, Serialize, Deserialize)]
pub struct FeederSystem;

#[typetag::serde]
impl Subsystem for FeederSystem {
    fn tick(&mut self, _dt: f64) {}
    fn name(&self) -> &str { "feeder" }
    fn draw_overlay(&mut self) {
        let sw = screen_width();
        let sh = screen_height();
        draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.5));
        draw_rectangle(sw * 0.2, sh * 0.2, sw * 0.6, sh * 0.6, BLUE);
    }
}

inventory::submit!(SubsystemRegistration {
    create: || Box::new(FeederSystem),
});