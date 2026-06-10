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
            PlantStage::Empty   => "Empty",
            PlantStage::Seed    => "Seed",
            PlantStage::Sprout  => "Sprout",
            PlantStage::Grown   => "Grown",
            PlantStage::Harvest => "Ready to Harvest!",
            PlantStage::Dead    => "Dead",
        };

        let plant_label = spot.plant.as_deref().unwrap_or("none");
        let spot_label = format!("Spot {}: {} | Stage: {} | Watered: {} | Fertilised: {}", i, plant_label, stage_label, spot.watered, spot.fertilised);

        if ui.button(None, spot_label.as_str()) {
            if spot.stage == PlantStage::Empty {
                if let Some(ref item) = bed.selected_item.clone() {
                    bed.pending_plant = Some((i, item.clone()));
                    bed.selected_item = None;
                }
            }
        }

        let water_label = format!("Water##{}", i);
        if ui.button(None, water_label.as_str()) {
            spot.watered = true;
        }

        let fertilise_label = format!("Fertilise##{}", i);
        if ui.button(None, fertilise_label.as_str()) {
            if let Some(ref item) = bed.selected_item.clone() {
                // only apply if the selected item is a fertiliser
                if bed.fertiliser_definitions.iter().any(|f| f.fertiliser_id == *item) {
                    bed.pending_fertilise = Some((i, item.clone()));
                    bed.selected_item = None;
                    spot.fertilised = true;
                }
            }
        }

        if matches!(spot.stage, PlantStage::Harvest) {
            let harvest_label = format!("Harvest##{}", i);
            if ui.button(None, harvest_label.as_str()) {
                bed.pending_harvest = Some(i);
            }
        }

        if matches!(spot.stage, PlantStage::Dead) {
            let dead_label = format!("Dead##{}", i);
            if ui.button(None, dead_label.as_str()) {
                bed.pending_waste = Some(i);
            }
        }

        ui.separator();
    }
}