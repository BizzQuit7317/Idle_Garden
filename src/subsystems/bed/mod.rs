mod system;
mod overlay;

use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};
use serde::{Serialize, Deserialize};
use crate::subsystems::{Subsystem, SubsystemRegistration, ResourceContext, SubsystemOutput, ItemDefinition};

#[derive(Debug, Serialize, Deserialize)]
pub struct BedSystem {
    pub soil_quality: u8,
}

impl BedSystem {
    pub fn new() -> BedSystem {
        BedSystem {
            soil_quality: 0,
        }
    }
}

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
    create: || Box::new(BedSystem::new()),
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

register_item!("blue_flower_seed", "Blue Flower Seed", "A small seed for a blue flower.");
register_item!("flower_flower", "Blue Flower", "A blue flower.");
register_item!("grass_seeds", "Grass Seeds", "Grass seeds, what else?.");
register_item!("grass", "Grass", "Regular degular grass buddy.");