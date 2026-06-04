use super::FeederSystem;
use crate::subsystems::{ResourceContext, SubsystemOutput};

pub fn tick(bed: &mut FeederSystem, ctx: &ResourceContext) -> SubsystemOutput {
    SubsystemOutput {
        conservation_delta: 1.0, //feed will tick up conervation
        cash_delta: 0.0,
    }
}