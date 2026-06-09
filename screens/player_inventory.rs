use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets, hash};

use crate::data;
use crate::utility;
use crate::systems;
use crate::screens::screen::{Screen, ScreenTransition};
use crate::screens::home::Home;
use crate::screens::store::Store;
use crate::subsystems::{available_subsystems, get_item_definition};

pub struct PlayerInventory;

impl PlayerInventory {
    pub fn new() -> Self { PlayerInventory }
}

impl Screen for PlayerInventory {
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

        // Inventory display — 3 slots along the bottom Need to uppdate too adapt to mutliple slots
        let inventory_items: Vec<(&String, &u64)> = game.player.inventory.items.iter().collect();
        let cols = 3;
        let rows = 3;
        let grid_top = sh * 0.35;
        let grid_left = sw * 0.1;
        let grid_width = sw * 0.8;
        let grid_height = sh * 0.55; // fills space between grid_top and ~90% screen height
        let slot_width = grid_width / cols as f32;
        let slot_height = grid_height / rows as f32;

        for slot in 0..(cols * rows) {
            let col = slot % cols;
            let row = slot / cols;
            let x = grid_left + col as f32 * slot_width;
            let y = grid_top + row as f32 * slot_height;

            draw_rectangle(x + 5.0, y + 5.0, slot_width - 10.0, slot_height - 10.0, Color::new(0.0, 0.0, 0.0, 0.4));

            if let Some((id, quantity)) = inventory_items.get(slot) {
                let display_name = get_item_definition(id)
                    .map(|def| def.display_name)
                    .unwrap_or(id.as_str());

                draw_text(
                    &format!("{}: {}", display_name, quantity),
                    x + 15.0,
                    y + slot_height * 0.5,
                    22.0,
                    WHITE,
                );
            } else {
                draw_text(
                    "Empty",
                    x + 15.0,
                    y + slot_height * 0.5,
                    22.0,
                    Color::new(0.6, 0.6, 0.6, 1.0),
                );
            }
        }

        ScreenTransition::Stay
    }
}