use super::BedSystem;
use crate::subsystems::{ResourceContext, SubsystemOutput};

pub fn tick(bed: &mut BedSystem, ctx: &ResourceContext) -> SubsystemOutput {
    SubsystemOutput {
        conservation_delta: 0.0,  
        cash_delta: 1.0,
    }
}