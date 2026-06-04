use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};
use serde::{Serialize, Deserialize};
use crate::subsystems::{Subsystem, SubsystemRegistration, ResourceContext, SubsystemOutput};

#[derive(Debug, Serialize, Deserialize)]
pub struct BedSystem;

#[typetag::serde]
impl Subsystem for BedSystem {
    fn tick(&mut self, _ctx: &ResourceContext) -> SubsystemOutput {
        SubsystemOutput {
            conservation_delta: 0.0, 
            cash_delta: 1.0, //Bed will tick up cash
        }

    }

    fn name(&self) -> &str { "bed" }

    fn draw_overlay(&mut self) {
        let sw = screen_width();
        let sh = screen_height();
        draw_rectangle(sw * 0.1, sh * 0.1, sw * 0.8, sh * 0.8, DARKGRAY);
        draw_text("Bed System", sw * 0.15, sh * 0.2, 32.0, WHITE);
    }
}

inventory::submit!(SubsystemRegistration {
    create: || Box::new(BedSystem),
});