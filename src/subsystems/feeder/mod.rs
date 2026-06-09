mod system;
mod overlay;

use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};
use serde::{Serialize, Deserialize};
use crate::subsystems::{Subsystem, SubsystemRegistration, ResourceContext, SubsystemOutput, ItemDefinition};
use crate::systems::player::Property;

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

    fn draw_overlay(&mut self, ui: &mut macroquad::ui::Ui, ctx: &ResourceContext) {
        overlay::draw(ui, self, ctx);
    }
}

inventory::submit!(SubsystemRegistration {
    create: || Box::new(FeederSystem::new()),
    min_property: Property::Balcony,
});

macro_rules! register_item {
    ($id:expr, $display:expr, $desc:expr, $cash:expr, $conservation:expr) => {
        inventory::submit!(ItemDefinition {
            id: $id,
            display_name: $display,
            description: $desc,
            cash_value: $cash,
            conservation_value: $conservation,
        });
    }
}

register_item!("bird_feed", "Bird Feed", "Seeds and scraps left by visiting birds.", 1.0, 1.0);
register_item!("feather", "Feather", "A small feather left behind by a visiting bird.", 10.0, 1.0);