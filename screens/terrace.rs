use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets, hash};

use crate::data;
use crate::utility;
use crate::systems;
use crate::screens::screen::{Screen, ScreenTransition};
use crate::screens::store::Store;
use crate::screens::player_inventory::PlayerInventory;
use crate::subsystems::{available_subsystems, get_item_definition};
use crate::screens::routing::screen_for_player;

pub struct Terrace {
    active_slot: Option<usize>,
    picking_for_slot: Option<usize>,
}

impl Terrace {
    pub fn new() -> Self {
        Terrace {
            active_slot: None,
            picking_for_slot: None,
        }
    }
}

impl Screen for Terrace {
    fn draw(&mut self, game: &mut systems::game_state::GameState) -> ScreenTransition {
        let sw = screen_width();
        let sh = screen_height();

        clear_background(data::constants::DEFAULT_BACKGROUND_COLOR);

        //Draw text elements, they will still fall behind root_ui elements
        draw_text(&format!("Cash: {:.2}", game.player.cash), sw * 0.05, sh * 0.05, 28.0, WHITE);
        draw_text(&format!("Conservation: {:.2}", game.player.conservation_points), sw * 0.3, sh * 0.05, 28.0, WHITE);

        // Save Button
        if widgets::Button::new("Save")
            .position(vec2(sw * 0.1, sh * 0.15))
            .size(vec2(200.0, 80.0))
            .ui(&mut root_ui())
        {
            utility::file_control::save_game_json(game);
            println!("[DBG]Saved Game");
        }

        //MAIN TABS
        if widgets::Button::new("Go to Property")
            .position(vec2(sw * 0.3, sh * 0.2))
            .size(vec2(200.0, 80.0))
            .ui(&mut root_ui())
        {
            return ScreenTransition::Goto(screen_for_player(&game.player));
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

            root_ui().window(
                hash!("subsystem_overlay"),
                vec2(sw * 0.1, sh * 0.1),
                vec2(sw * 0.8, sh * 0.8),
                |ui| {
                    if ui.button(None, "Close") {
                        close = true;
                    }
                }
            );

            if let Some(Some(subsystem)) = game.player.slots.get_mut(i) {
                subsystem.draw_overlay();
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
                    let all = available_subsystems();
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
                game.player.slots[slot_index] = available_subsystems()
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