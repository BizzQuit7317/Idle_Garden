use super::BedSystem;
use crate::subsystems::{ResourceContext, SubsystemOutput};

use serde::{Serialize, Deserialize};

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
    pub produces_id: String,
    pub ticks_per_stage: f64,
    pub water_till_death: u8
}

pub fn load_plant_definitions() -> Vec<PlantDefinition> {
    let json = include_str!("assets/plants.json");
    serde_json::from_str(json).expect("Failed to parse plants.json")
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GrowingSpot {
    pub plant: Option<String>,
    pub stage: PlantStage,
    pub watered: bool,
    pub ticks_passed: f64,
}

impl GrowingSpot {
    pub fn new() -> GrowingSpot {
        GrowingSpot {
           plant: None,
           stage: PlantStage::Empty,
           watered: false,
           ticks_passed: 0.0,
        }
    }

    pub fn can_grow(&self) -> bool {
        if self.plant != None && self.watered{
            return true
        }
        false
    }

    pub fn advance(&mut self) {
        match self.stage {
            PlantStage::Seed => { self.stage = PlantStage::Sprout},
            PlantStage::Sprout => { self.stage = PlantStage::Grown},
            PlantStage::Grown => { self.stage = PlantStage::Harvest},
            PlantStage::Harvest => {},
            PlantStage::Dead => {},
            PlantStage::Empty => {},
        }
    }

    pub fn harvest(&mut self) -> Option<String> {
        let item = self.plant.clone(); //Need too change this t give grown plant noot just seeds
        self.plant = None;
        self.stage = PlantStage::Empty;
        self.ticks_passed = 0.0;
        self.watered = false;
        item
    }
}

pub fn tick(bed: &mut BedSystem, ctx: &ResourceContext) -> SubsystemOutput {
    let mut output = SubsystemOutput::empty();

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
                    output.items_produced.push((def.produces_id.clone(), 1));
                }
            }
        }
    }

    // Grow loop
    for spot in &mut bed.growing_spots {
        if spot.can_grow() {
            spot.advance();
        }
    }

    output
}