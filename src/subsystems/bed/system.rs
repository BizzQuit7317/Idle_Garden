use super::BedSystem;
use crate::subsystems::{ResourceContext, SubsystemOutput};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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
}

pub fn tick(bed: &mut BedSystem, ctx: &ResourceContext) -> SubsystemOutput {
    let mut output = SubsystemOutput::empty();

    //loop over growing spots
    for spot in &mut bed.growing_spots {
        
    }

    output
}