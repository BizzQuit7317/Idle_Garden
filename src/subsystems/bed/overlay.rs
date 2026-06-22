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
        let def = crate::subsystems::get_item_definition(item_id);
        let display_name = def.map(|d| d.display_name).unwrap_or(item_id.as_str());
        let label = format!("{}: {}", display_name, amount);
        if ui.button(None, label.as_str()) {
            bed.selected_item = Some(item_id.clone());
        }
    }

    if let Some(ref selected) = bed.selected_item.clone() {
        let selected_display = crate::subsystems::get_item_definition(selected)
            .map(|d| d.display_name)
            .unwrap_or(selected.as_str());
        ui.label(None, &format!("Selected: {}", selected_display));
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

        let plant_display = spot.plant.as_deref()
            .and_then(|id| bed.plant_definitions.iter().find(|p| p.seed_id == id))
            .map(|p| p.display_name.as_str())
            .unwrap_or("Empty");

        let spot_label = format!("Spot {}: {} | Stage: {} | Watered: {} | Fertilised: {}", i, plant_display, stage_label, spot.watered, spot.fertilised);

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