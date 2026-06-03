use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};

use crate::data;
use crate::utility;
use crate::systems;
use crate::screens::screen::{Screen, ScreenTransition};
use crate::subsystems::available_subsystems;

pub struct Balcony {
    active_slot: Option<usize>,
    picking_for_slot: Option<usize>,
}

impl Balcony {
    pub fn new() -> Self {
        Balcony {
            active_slot: None,
            picking_for_slot: None,
        }
    }
}

impl Screen for Balcony {
    fn draw(&mut self, game: &mut systems::game_state::GameState) -> ScreenTransition {
        let sw = screen_width();
        let sh = screen_height();

        clear_background(data::constants::DEFAULT_BACKGROUND_COLOR);

        // Save Button
        if widgets::Button::new("Save")
            .position(vec2(sw * 0.25, sh * 0.25))
            .size(vec2(200.0, 80.0))
            .ui(&mut root_ui())
        {
            utility::file_control::save_game_json(game);
            println!("[DBG]Saved Game");
        }

        // Slot buttons — one per slot the player has
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

        // Subsystem overlay — drawn on top when a filled slot is open
        if let Some(i) = self.active_slot {
            if let Some(Some(subsystem)) = game.player.slots.get_mut(i) {
                subsystem.draw_overlay();
            }
            if widgets::Button::new("Close")
                .position(vec2(sw * 0.7, sh * 0.25))
                .size(vec2(100.0, 40.0))
                .ui(&mut root_ui())
            {
                self.active_slot = None;
            }
        }

        // Picker overlay — drawn on top when an empty slot is clicked
        if let Some(slot_index) = self.picking_for_slot {
            draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.5));
            draw_rectangle(sw * 0.2, sh * 0.2, sw * 0.6, sh * 0.6, DARKGRAY);

            let all = available_subsystems();
            for (i, subsystem) in all.iter().enumerate() {
                if widgets::Button::new(subsystem.name())
                    .position(vec2(sw * 0.3, sh * 0.3 + i as f32 * 60.0))
                    .size(vec2(200.0, 50.0))
                    .ui(&mut root_ui())
                {
                    let chosen_name = subsystem.name().to_string();
                    game.player.slots[slot_index] = available_subsystems()
                        .into_iter()
                        .find(|s| s.name() == chosen_name);
                    self.picking_for_slot = None;
                }
            }

            if widgets::Button::new("Cancel")
                .position(vec2(sw * 0.7, sh * 0.25))
                .size(vec2(100.0, 40.0))
                .ui(&mut root_ui())
            {
                self.picking_for_slot = None;
            }
        }

        ScreenTransition::Stay
    }
}