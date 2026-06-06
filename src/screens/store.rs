use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets, hash};

use crate::data;
use crate::utility;
use crate::systems;
use crate::screens::screen::{Screen, ScreenTransition};
use crate::screens::routing::screen_for_player;
use crate::screens::player_inventory::PlayerInventory;
use crate::systems::store_state::StoreItem;
use crate::subsystems::{available_subsystems, get_item_definition};

pub struct Store;

impl Store {
    pub fn new() -> Self { Store }
}

impl Screen for Store {
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

        //Display the stock
        let cols = 3usize;
        let rows = (game.store.stock_limit as usize + cols - 1) / cols;
        let grid_top = sh * 0.35;
        let grid_left = sw * 0.1;
        let grid_width = sw * 0.8;
        let grid_height = sh * 0.55;
        let slot_width = grid_width / cols as f32;
        let slot_height = grid_height / rows as f32;

        for slot in 0..game.store.stock_limit as usize {
            let col = slot % cols;
            let row = slot / cols;
            let x = grid_left + col as f32 * slot_width;
            let y = grid_top + row as f32 * slot_height;

            draw_rectangle(x + 5.0, y + 5.0, slot_width - 10.0, slot_height - 10.0, Color::new(0.0, 0.0, 0.0, 0.4));

            if let Some(item) = game.store.stock.get(slot) {
                let display_name = get_item_definition(&item.item_id)
                    .map(|def| def.display_name)
                    .unwrap_or(&item.item_id);

                draw_text(&format!("{}", display_name), x + 15.0, y + slot_height * 0.35, 22.0, WHITE);
                draw_text(&format!("£{:.2}", item.price), x + 15.0, y + slot_height * 0.6, 20.0, WHITE);
                draw_text(&format!("Qty: {}", item.quantity_available), x + 15.0, y + slot_height * 0.8, 18.0, Color::new(0.7, 0.7, 0.7, 1.0));
            } else {
                draw_text("Empty", x + 15.0, y + slot_height * 0.5, 22.0, Color::new(0.6, 0.6, 0.6, 1.0));
            }
        }

                ScreenTransition::Stay
            }
        }