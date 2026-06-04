mod system;
mod overlay;

use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};
use serde::{Serialize, Deserialize};
use crate::subsystems::{Subsystem, SubsystemRegistration, ResourceContext, SubsystemOutput};

#[derive(Debug, Serialize, Deserialize)]
pub struct BedSystem;

#[typetag::serde]
impl Subsystem for BedSystem {
    fn tick(&mut self, ctx: &ResourceContext) -> SubsystemOutput {
        system::tick(self, ctx)
    }

    fn name(&self) -> &str { "bed" }

    fn draw_overlay(&mut self) {
        overlay::draw();
    }
}

inventory::submit!(SubsystemRegistration {
    create: || Box::new(BedSystem),
});