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
                if let Some(spot) = bed.growing_spots.get_mut(spot_index) {
                    spot.plant = Some(item_id.clone());
                    spot.stage = PlantStage::Seed;
                    output.items_consumed.push((item_id, 1));
                }
            }
        }
    }

    // Handle pending harvest
    if let Some(spot_index) = bed.pending_harvest.take() {
        if let Some(spot) = bed.growing_spots.get_mut(spot_index) {
            if let Some(item_id) = spot.harvest() {
                output.items_produced.push((item_id, 1));
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