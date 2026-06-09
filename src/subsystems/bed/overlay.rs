use macroquad::prelude::*;
use macroquad::ui::Ui;
use crate::subsystems::bed::system::{GrowingSpot, PlantStage};
use crate::subsystems::ResourceContext;

pub fn draw(ui: &mut Ui, bed: &mut crate::subsystems::bed::BedSystem, ctx: &ResourceContext) {
    ui.label(None, "=== Bed System ===");
    ui.separator();

    // Inventory panel
    ui.label(None, "-- Inventory --");
    for (item_id, amount) in &ctx.inventory {
        let label = format!("{}: {}", item_id, amount);
        if ui.button(None, label.as_str()) {
            bed.selected_item = Some(item_id.clone());
        }
    }

    if let Some(ref selected) = bed.selected_item.clone() {
        ui.label(None, &format!("Selected: {}", selected));
    }

    ui.separator();

    // Growing spots
    ui.label(None, "-- Growing Spots --");
    for (i, spot) in bed.growing_spots.iter_mut().enumerate() {
        let stage_label = match spot.stage {
            PlantStage::Empty    => "Empty",
            PlantStage::Seed     => "Seed",
            PlantStage::Sprout   => "Sprout",
            PlantStage::Grown    => "Grown",
            PlantStage::Harvest  => "Ready to Harvest!",
            PlantStage::Dead     => "Dead",
        };

        let plant_label = spot.plant.as_deref().unwrap_or("none");
        let label = format!("Spot {}: {} | Stage: {} | Watered: {}", i + 1, plant_label, stage_label, spot.watered);
        if ui.button(None, label.as_str()) {
            if let Some(ref item) = bed.selected_item.clone() {
                spot.plant = Some(item.clone());
                bed.selected_item = None; // deselect after planting
            }
        }

        let label = format!("Water##{}", i);
        if ui.button(None, label.as_str()) {
            spot.watered = true;
        }

        if matches!(spot.stage, PlantStage::Harvest) {
            let harvest_label = format!("Harvest##{}", i);
            if ui.button(None, harvest_label.as_str()) {
                spot.plant = None;
                spot.stage = PlantStage::Empty;
                spot.ticks_passed = 0.0;
                spot.watered = false;
            }
        }

        ui.separator();
    }
}