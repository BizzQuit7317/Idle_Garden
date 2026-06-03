use std::fmt::Debug;

#[typetag::serde(tag = "type")]
pub trait Subsystem: Debug {
    fn tick(&mut self, dt: f64);
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