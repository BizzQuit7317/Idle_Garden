mod system;
mod overlay;

use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};
use serde::{Serialize, Deserialize};
use crate::subsystems::{Subsystem, SubsystemRegistration, ResourceContext, SubsystemOutput, ItemDefinition};

#[derive(Debug, Serialize, Deserialize)]
pub struct FeederSystem {
    pub feed: u8,
}

impl FeederSystem {
    pub fn new() -> FeederSystem {
        FeederSystem {
            feed: 0,
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

macro_rules! register_item {
    ($id:expr, $display:expr, $desc:expr) => {
        inventory::submit!(ItemDefinition {
            id: $id,
            display_name: $display,
            description: $desc,
        });
    }
}

register_item!("bird_feed", "Bird Feed", "Seeds and scraps left by visiting birds.");
register_item!("feather", "Feather", "A small feather left behind by a visiting bird.");