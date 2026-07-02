use std::fmt::Debug;
use std::collections::HashMap;

use crate::systems::player::Property;
use crate::systems::npc::NPC;

pub struct  ResourceContext {
    pub cash: f64,
    pub conservation_points: f64,
    pub inventory: HashMap<String, u64>,
    pub npcs: Vec<NPC>,
}

pub struct SubsystemOutput {
    pub conservation_delta: f64,
    pub cash_delta: f64,
    pub items_produced: Vec<(String, u64)>,
    pub items_consumed: Vec<(String, u64)>,
}

impl SubsystemOutput {
    pub fn empty() -> Self {
        SubsystemOutput {
            conservation_delta: 0.0,
            cash_delta: 0.0,
            items_produced: Vec::new(),
            items_consumed: Vec::new(),
        }
    }
}

pub struct ItemDefinition {
    pub id: &'static str,
    pub display_name: &'static str,
    pub description: &'static str,
    pub cash_value: f64,
    pub conservation_value: f64,
    pub in_store: bool,
}

inventory::collect!(ItemDefinition);

pub fn available_items() -> Vec<&'static ItemDefinition> {
    inventory::iter::<ItemDefinition>
        .into_iter()
        .collect()
}

pub fn get_item_definition(id: &str) -> Option<&'static ItemDefinition> {
    inventory::iter::<ItemDefinition>
        .into_iter()
        .find(|def| def.id == id)
}

#[typetag::serde(tag = "type")]
pub trait Subsystem: Debug {
    fn tick(&mut self, ctx: &ResourceContext) -> SubsystemOutput;
    fn name(&self) -> &str;
    fn draw_overlay(&mut self, ui: &mut macroquad::ui::Ui, ctx: &ResourceContext);
}

pub struct SubsystemRegistration {
    pub create: fn() -> Box<dyn Subsystem>,
    pub min_property: Property,
}

inventory::collect!(SubsystemRegistration);

pub fn available_subsystems(player_property: &Property) -> Vec<Box<dyn Subsystem>> {
    inventory::iter::<SubsystemRegistration>
        .into_iter()
        .filter(|r| r.min_property <= *player_property)
        .map(|r| (r.create)())
        .collect()
}

pub mod bed;
pub mod feeder;