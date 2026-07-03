use macroquad::prelude::*;
use macroquad::ui::{Ui, hash, widgets};
use crate::subsystems::bed::system::{GrowingSpot, PlantStage};
use crate::subsystems::ResourceContext;
use crate::systems::popup::Modal;
use crate::systems::npc::NPCViewState;

pub fn draw(ui: &mut Ui, bed: &mut crate::subsystems::bed::BedSystem, ctx: &ResourceContext) {
    ui.label(None, "=== Bed System ===");

    //Tempporary location of npc interation button
    //Get npc index
    let mut index_counter = 0;
    let mut bed_npc = &ctx.npcs[0]; //default to 0 index npc incase of failure
    for npc in &ctx.npcs {
        if npc.id == bed.npc_id {
            //bed_npc_index = index_counter;
            bed_npc = npc;
        }
        index_counter += 1;
    }

    if ui.button(None, "Bed NPC") {
        //println!("Bed NPC: {:?}", bed_npc.key_dialogue[bed_npc.key_dialogue_index]);
        bed.pending_modals.push(Modal {
            message: bed_npc.key_dialogue.clone(), 
            dismissed: false, 
            npc_flag: true,
            npc_name: Some(bed_npc.family_name.clone()),
            npc_state: Some(NPCViewState::Dialogue), // start in Dialogue if NPC
            npc_stock: Some(bed_npc.stock.clone()),
            current_line: 0,
        });
    }

    //Button for system upgrade
    let upgrade_label = format!("Upgrade Flower Bed: {:.0} Cash", bed.upgrade_price);
    if ui.button(None, upgrade_label.as_str()) {
        if ctx.cash > bed.upgrade_price {
            bed.pending_upgrade = true;
        }
    }

    ui.separator();

    let half_w = screen_width() / 2.0;
    let full_h = screen_height();

    let top_offset = 90.0; // tweak until it clears the close button + label

    // LEFT SIDE
    widgets::Group::new(hash!("left_panel"), Vec2::new(half_w, full_h - top_offset))
        .position(Vec2::new(0.0, top_offset))
        .ui(ui, |ui| {
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
                            let remaining = ctx.inventory
                                .iter()
                                .find(|(id, _)| *id == item)
                                .map(|(_, amt)| *amt)
                                .unwrap_or(0);

                            if remaining <= 1 {
                                bed.selected_item = None;
                            }
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
                            spot.fertilised = true;
                            let remaining = ctx.inventory
                                .iter()
                                .find(|(id, _)| *id == item)
                                .map(|(_, amt)| *amt)
                                .unwrap_or(0);

                            if remaining <= 1 {
                                bed.selected_item = None;
                            }
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
        });
    
    // RIGHT SIDE 
    widgets::Group::new(hash!("right_panel"), Vec2::new(half_w, full_h - top_offset))
        .position(Vec2::new(half_w, top_offset))
        .ui(ui, |ui| {
            ui.label(None, "Dropped Items");
            let item_label = format!("{:?}", bed.auto_harvested_items);
            ui.label(None, item_label.as_str());

            ui.separator();
        });
}