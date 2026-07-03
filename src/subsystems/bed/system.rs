use super::BedSystem;
use crate::subsystems::{ResourceContext, SubsystemOutput};
use macroquad::rand::gen_range;
use serde::{Serialize, Deserialize};
use crate::systems::popup::Modal;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PlantStage {
    Seed,
    Sprout,
    Grown,
    Harvest,
    Dead,
    Empty,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlantDefinition {
    pub seed_id: String,
    pub display_name: String,
    pub produces: Vec<(String, f64)>,
    pub dead_drops: Vec<(String, f64)>,
    pub ticks_per_stage: Vec<f64>,
    pub water_till_death: f64
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FertiliserDefinition {
    pub fertiliser_id: String,
    pub display_name: String,
    pub stage_modifiers: Vec<f64>,
}

pub fn load_plant_definitions() -> Vec<PlantDefinition> {
    let json = include_str!("assets/plants.json");
    serde_json::from_str(json).expect("Failed to parse plants.json")
}

pub fn load_fertiliser_definitions() -> Vec<FertiliserDefinition> {
    let json = include_str!("assets/fertiliser.json");
    serde_json::from_str(json).expect("Failed to parse fertiliser.json")
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GrowingSpot {
    pub plant: Option<String>,
    pub stage: PlantStage,
    pub watered: bool,
    pub fertilised: bool, 
    pub ticks_passed: f64,
    pub water_retention_ticks: f64,
    pub ticks_since_watered: f64,
    pub drought_counter: f64,
    pub stage_modifiers: Option<Vec<f64>>,
}

impl GrowingSpot {
    pub fn new() -> GrowingSpot {
        GrowingSpot {
           plant: None,
           stage: PlantStage::Empty,
           watered: false,
           fertilised: false,
           ticks_passed: 0.0,
           water_retention_ticks: 50.0,
           ticks_since_watered: 0.0,
           drought_counter: 0.0,
           stage_modifiers: None,
        }
    }

    pub fn can_grow(&self) -> bool {
        self.plant.is_some() && self.watered
    }

    pub fn advance(&mut self) {
        match self.stage {
            PlantStage::Seed    => { self.stage = PlantStage::Sprout;  self.ticks_passed = 0.0; },
            PlantStage::Sprout  => { self.stage = PlantStage::Grown;   self.ticks_passed = 0.0; },
            PlantStage::Grown   => { self.stage = PlantStage::Harvest; self.ticks_passed = 0.0; },
            PlantStage::Harvest => {},
            PlantStage::Dead    => {},
            PlantStage::Empty   => {},
        }
    }

    pub fn harvest(&mut self) -> Option<String> {
        let item = self.plant.clone();
        self.plant = None;
        self.stage = PlantStage::Empty;
        self.ticks_passed = 0.0;
        self.ticks_since_watered = 0.0;
        self.drought_counter = 0.0;
        self.watered = false;
        self.fertilised = false;
        self.stage_modifiers = None;
        item
    }

    pub fn clear_dead(&mut self) -> Option<String> {
        let item = self.plant.clone();
        self.plant = None;
        self.stage = PlantStage::Empty;
        self.ticks_passed = 0.0;
        self.ticks_since_watered = 0.0;
        self.drought_counter = 0.0;
        self.watered = false;
        self.fertilised = false;
        self.stage_modifiers = None;
        item
    }
}

pub fn tick(bed: &mut BedSystem, ctx: &ResourceContext) -> SubsystemOutput {
    let mut output = SubsystemOutput::empty();

    // Handle pending fertilise
    if let Some((spot_index, item_id)) = bed.pending_fertilise.take() {
        if let Some(amount) = ctx.inventory.get(&item_id) {
            if *amount > 0 {
                if let Some(def) = bed.fertiliser_definitions.iter().find(|f| f.fertiliser_id == item_id) {
                    if let Some(spot) = bed.growing_spots.get_mut(spot_index) {
                        spot.stage_modifiers = Some(def.stage_modifiers.clone());
                        output.items_consumed.push((item_id, 1));
                    }
                }
            }
        }
    }

    // Handle pending plant
    if let Some((spot_index, item_id)) = bed.pending_plant.take() {
        if let Some(amount) = ctx.inventory.get(&item_id) {
            if *amount > 0 {
                if bed.plant_definitions.iter().any(|p| p.seed_id == item_id) {
                    if let Some(spot) = bed.growing_spots.get_mut(spot_index) {
                        spot.plant = Some(item_id.clone());
                        spot.stage = PlantStage::Seed;
                        output.items_consumed.push((item_id, 1));
                    }
                } else {
                    println!("[DBG] No plant definition found for item: {}", item_id);
                }
            }
        }
    }

    // Handle pending harvest
    if let Some(spot_index) = bed.pending_harvest.take() {
        if let Some(spot) = bed.growing_spots.get_mut(spot_index) {
            if let Some(seed_id) = spot.harvest() {
                if let Some(def) = bed.plant_definitions.iter().find(|p| p.seed_id == seed_id) {
                    for (item, chance) in &def.produces {
                        let roll: f64 = gen_range(0.0, 1.0);
                        if roll <= *chance {
                            output.items_produced.push((item.clone(), 1));
                        }
                    }
                }
            }
        }
    }

    // Handle pending waste
    if let Some(spot_index) = bed.pending_waste.take() {
        if let Some(spot) = bed.growing_spots.get_mut(spot_index) {
            if let Some(seed_id) = spot.clear_dead() {
                if let Some(def) = bed.plant_definitions.iter().find(|p| p.seed_id == seed_id) {
                    for (item, chance) in &def.dead_drops {
                        let roll: f64 = gen_range(0.0, 1.0);
                        if roll <= *chance {
                            output.items_produced.push((item.clone(), 1));
                        }
                    }
                }
            }
        }
    }

    //Handle pending modals
    for modal in bed.pending_modals.drain(..) {
        output.modals.push(modal);
    }

    // Grow loop
    for spot in &mut bed.growing_spots {
        if let Some(ref seed_id) = spot.plant {
            if let Some(def) = bed.plant_definitions.iter().find(|p| p.seed_id == *seed_id) {

                let stage_index = match spot.stage {
                    PlantStage::Seed   => 0,
                    PlantStage::Sprout => 1,
                    PlantStage::Grown  => 2,
                    _ => continue,
                };

                if let Some(&base_ticks) = def.ticks_per_stage.get(stage_index) {
                    let modifier = spot.stage_modifiers
                        .as_ref()
                        .and_then(|m| m.get(stage_index))
                        .copied()
                        .unwrap_or(1.0);

                    if spot.can_grow() && spot.ticks_passed >= base_ticks * modifier {
                        spot.advance();
                    }
                }

                // Watering state
                if spot.watered {
                    spot.ticks_since_watered = 0.0;
                    spot.drought_counter = 0.0;
                } else {
                    spot.ticks_since_watered += 1.0;
                    spot.drought_counter += 1.0;
                }

                if spot.ticks_since_watered >= spot.water_retention_ticks {
                    spot.watered = false;
                }

                if spot.drought_counter >= def.water_till_death {
                    spot.stage = PlantStage::Dead;
                }
            }
        }

        spot.ticks_passed += 1.0;
    }

    output
}