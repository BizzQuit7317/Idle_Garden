use std::fmt::Debug;

pub struct  ResourceContext {
    pub cash: f64,
    pub conservation_points: f64,
}

pub struct SubsystemOutput {
    pub conservation_delta: f64,
    pub cash_delta: f64,
}

impl SubsystemOutput {
    pub fn empty() -> Self {
        SubsystemOutput {
            conservation_delta: 0.0,
            cash_delta: 0.0,
        }
    }
}

#[typetag::serde(tag = "type")]
pub trait Subsystem: Debug {
    fn tick(&mut self, ctx: &ResourceContext) -> SubsystemOutput;
    fn name(&self) -> &str;
    fn draw_overlay(&mut self);
}

pub struct SubsystemRegistration {
    pub create: fn() -> Box<dyn Subsystem>,
}

inventory::collect!(SubsystemRegistration);

pub fn available_subsystems() -> Vec<Box<dyn Subsystem>> {
    inventory::iter::<SubsystemRegistration>
        .into_iter()
        .map(|r| (r.create)())
        .collect()
}

pub mod bed;
pub mod feeder;