use macroquad::prelude::*;
use macroquad::ui::{Ui, hash, widgets};
use crate::subsystems::ResourceContext;
use super::FeederSystem;

pub fn draw(ui: &mut Ui, feeder: &mut FeederSystem, ctx: &ResourceContext) { 
    ui.label(None, "=== Feeder System ===");
    ui.separator();

    let half_w = screen_width() / 2.0;
    let full_h = screen_height();

    let top_offset = 60.0; // tweak until it clears the close button + label

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
                    feeder.selected_item = Some(item_id.clone());
                }
            }

            if let Some(ref selected) = feeder.selected_item.clone() {
                ui.label(None, &format!("Selected: {}", selected));
            }

            ui.separator();

            // Feeder spot
            ui.label(None, "-- Current Feeder --");

            let feeder_label = match &feeder.current_feeder {
                Some(f) => format!("Feeder: {}", f.display_name),
                None => "Feeder: Empty".to_string(),
            };

            if ui.button(None, feeder_label.as_str()) {
                if feeder.current_feeder.is_none() {
                    if let Some(ref item) = feeder.selected_item.clone() {
                        feeder.pending_feeder = feeder.feeder_definitions
                            .iter()
                            .find(|f| f.feeder_id == *item)
                            .cloned();
                        feeder.selected_item = None;
                    }
                }
            }

            let food_label = match &feeder.current_food {
                Some(f) => format!("Food: {}", f.display_name),
                None => String::from("Food: Empty"),
            };

            if ui.button(None, food_label.as_str()) {
                if feeder.current_food_amount == 0 && feeder.current_feeder.is_some() {
                    if let Some(ref item) = feeder.selected_item.clone() {
                        if let Some(food_def) = feeder.food_definitions.iter().find(|f| f.food_id == *item) {
                            if let Some(ref current_feeder) = feeder.current_feeder {
                                if food_def.quantity <= current_feeder.food_capacity {
                                    feeder.pending_food = Some(food_def.clone());
                                    feeder.selected_item = None;
                                } else {
                                    println!("[DBG] Food quantity {} exceeds feeder capacity {}", food_def.quantity, current_feeder.food_capacity);
                                }
                            }
                        }
                    }
                }
            }

            let food_amount_label = format!("Food Amount: {}", feeder.current_food_amount);
            ui.label(None, food_amount_label.as_str());

            ui.separator();

            // Current birds panel
            ui.label(None, "-- Current Birds --");
            if feeder.current_birds.is_empty() {
                ui.label(None, "No birds");
            } else {
                for (bird, ticker) in &feeder.current_birds {
                    let label = format!("{} ({:.0}/{:.0})", bird.display_name, ticker, bird.max_time);
                    ui.label(None, &label);
                }
            }

            ui.separator();
        });

    // RIGHT SIDE 
    widgets::Group::new(hash!("right_panel"), Vec2::new(half_w, full_h - top_offset))
        .position(Vec2::new(half_w, top_offset))
        .ui(ui, |ui| {
            ui.label(None, "Dropped Items");
            let item_label = format!("{:?}", feeder.dropped_items);
            ui.label(None, item_label.as_str());

            ui.separator();

            if ui.button(None, "Collect Items") {
                for (item, amount) in feeder.dropped_items.drain() {
                    *feeder.pending_items.entry(item).or_insert(0) += amount;
                }
            }

            ui.separator();
        });
}