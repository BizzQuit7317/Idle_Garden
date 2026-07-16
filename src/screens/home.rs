use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets, hash};

use crate::data;
use crate::utility;
use crate::systems;
use crate::screens::screen::{Screen, ScreenTransition};
use crate::screens::store::Store;
use crate::screens::player_inventory::PlayerInventory;
use crate::screens::rebirth::Rebirth;
use crate::subsystems::{available_subsystems, get_item_definition};

pub struct Home {
    active_slot: Option<usize>,
    picking_for_slot: Option<usize>,
}

impl Home {
    pub fn new() -> Self {
        Home {
            active_slot: None,
            picking_for_slot: None,
        }
    }
}

impl Screen for Home {
    fn draw(&mut self, game: &mut systems::game_state::GameState) -> ScreenTransition {
        let sw = screen_width();
        let sh = screen_height();

        clear_background(data::constants::DEFAULT_BACKGROUND_COLOR);

        //Interact with the tutorial_npc
        if widgets::Button::new("Tutorial NPC")
            .position(vec2(sw * 0.8, sh * 0.4))
            .size(vec2(200.0, 80.0))
            .ui(&mut root_ui())
        {
            let remaining_dialogue = game.npcs[0].key_dialogue[game.npcs[0].key_dialogue_index..].to_vec();
            game.popups.push_modal(remaining_dialogue, Some(game.npcs[0].first_name.clone()), );
        }

        //Draw text elements, they will still fall behind root_ui elements
        draw_text(&format!("Cash: {:.2}", game.player.cash), sw * 0.05, sh * 0.05, 28.0, WHITE);
        draw_text(&format!("Conservation: {:.2}", game.player.conservation_points), sw * 0.3, sh * 0.05, 28.0, WHITE);
	    draw_text(&format!("First Name: {}", game.player.first_name), sw * 0.7, sh * 0.05, 28.0, WHITE);
	    draw_text(&format!("Family Name: {}", game.player.family_name), sw * 0.7, sh * 0.10, 28.0, WHITE);
	    draw_text(&format!("Generation: {}", game.player.generation), sw * 0.7, sh * 0.15, 28.0, WHITE);

        // Save Button
        if widgets::Button::new("Save")
            .position(vec2(sw * 0.1, sh * 0.15))
            .size(vec2(200.0, 80.0))
            .ui(&mut root_ui())
        {
            utility::file_control::save_game_json(game);
            game.popups.push_toast(String::from("Saved Game"), sw * 0.5, sh * 0.5, 1.0);
        }

        //MAIN TABS
        if widgets::Button::new("Go to Property")
            .position(vec2(sw * 0.3, sh * 0.2))
            .size(vec2(200.0, 80.0))
            .ui(&mut root_ui())
        {
            return ScreenTransition::Goto(Box::new(Home::new()));
        }

        if widgets::Button::new("Go to Inventory")
            .position(vec2(sw * 0.5, sh * 0.2))
            .size(vec2(200.0, 80.0))
            .ui(&mut root_ui())
        {
            return ScreenTransition::Goto(Box::new(PlayerInventory::new()));
        }

        if widgets::Button::new("Go to Store")
            .position(vec2(sw * 0.7, sh * 0.2))
            .size(vec2(200.0, 80.0))
            .ui(&mut root_ui())
        {
            return ScreenTransition::Goto(Box::new(Store::new()));
        }

	//yes I know its out of line but its just temporary
	if widgets::Button::new("Go to Rebirth")
            .position(vec2(sw * 0.1, sh * 0.6))
            .size(vec2(200.0, 80.0))
            .ui(&mut root_ui())
        {
            return ScreenTransition::Goto(Box::new(Rebirth::new()));
        }

        // Slot buttons — only drawn when no overlay is open
        if self.active_slot.is_none() && self.picking_for_slot.is_none() {
            for (i, slot) in game.player.slots.iter().enumerate() {
                let label = match slot {
                    Some(s) => s.name().to_string(),
                    None => "Empty".to_string(),
                };
                if widgets::Button::new(label.as_str())
                    .position(vec2(sw / 3.0 + i as f32 * 220.0, sh / 2.0))
                    .size(vec2(200.0, 80.0))
                    .ui(&mut root_ui())
                {
                    match slot {
                        Some(_) => self.active_slot = Some(i),
                        None => self.picking_for_slot = Some(i),
                    }
                }
            }
        }

        // Subsystem overlay
        if let Some(i) = self.active_slot {
            let mut close = false;

            let ctx = crate::subsystems::ResourceContext {
                cash: game.player.cash,
                conservation_points: game.player.conservation_points,
                inventory: game.player.inventory.items.clone(),
                npcs: game.npcs.clone(),
            };

            if let Some(Some(subsystem)) = game.player.slots.get_mut(i) {
                root_ui().window(
                    hash!("subsystem_overlay"),
                    vec2(sw * 0.1, sh * 0.1),
                    vec2(sw * 0.8, sh * 0.8),
                    |ui| {
                        if ui.button(None, "Close") {
                            close = true;
                        }
                        subsystem.draw_overlay(ui, &ctx);
                    }
                );
            }

            if close {
                self.active_slot = None;
            }
        }

        // Picker overlay
        if let Some(slot_index) = self.picking_for_slot {
            draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.5));

            let mut chosen: Option<String> = None;
            let mut cancelled = false;

            root_ui().window(
                hash!("picker_overlay"),
                vec2(sw * 0.2, sh * 0.2),
                vec2(sw * 0.6, sh * 0.6),
                |ui| {
                    let all = available_subsystems(&game.player.property);
                    for subsystem in all.iter() {
                        if ui.button(None, subsystem.name()) {
                            chosen = Some(subsystem.name().to_string());
                        }
                    }
                    if ui.button(None, "Cancel") {
                        cancelled = true;
                    }
                }
            );

            if let Some(name) = chosen {
                game.player.slots[slot_index] = available_subsystems(&game.player.property)
                    .into_iter()
                    .find(|s| s.name() == name);
                self.picking_for_slot = None;
            }
            if cancelled {
                self.picking_for_slot = None;
            }
        }

        ScreenTransition::Stay
    }
}