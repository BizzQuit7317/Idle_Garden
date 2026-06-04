mod system;
mod overlay;

use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};
use serde::{Serialize, Deserialize};
use crate::subsystems::{Subsystem, SubsystemRegistration, ResourceContext, SubsystemOutput};

#[derive(Debug, Serialize, Deserialize)]
pub struct FeederSystem {
    pub feed: u8,
}

impl FeederSystem {
    pub fn new() -> FeederSystem {
        FeederSystem {
            feed: 0;
        }
    }
}

#[typetag::serde]
impl Subsystem for FeederSystem {
    fn tick(&mut self, ctx: &ResourceContext) -> SubsystemOutput {
        system::tick(self, ctx)
    }

    fn name(&self) -> &str { "feeder" }

    fn draw_overlay(&mut self) {
        overlay::draw();
    }
}

inventory::submit!(SubsystemRegistration {
    create: || Box::new(FeederSystem::new()),
});