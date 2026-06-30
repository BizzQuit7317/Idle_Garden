use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets, hash};

use crate::data;
use crate::utility;
use crate::systems;
use crate::screens::screen::{Screen, ScreenTransition};
use crate::screens::home::Home;
use crate::screens::store::Store;
use crate::screens::player_inventory::PlayerInventory;

pub struct Rebirth;

impl Rebirth {
    pub fn new() -> Self { Rebirth }
}

impl Screen for Rebirth {
    fn draw(&mut self, game: &mut systems::game_state::GameState) -> ScreenTransition {
        let sw = screen_width();
        let sh = screen_height();

        clear_background(data::constants::DEFAULT_BACKGROUND_COLOR);

        //Draw text elements, they will still fall behind root_ui elements
        draw_text(&format!("Cash: {:.2}", game.player.cash), sw * 0.05, sh * 0.05, 28.0, WHITE);
        draw_text(&format!("Conservation: {:.2}", game.player.conservation_points), sw * 0.3, sh * 0.05, 28.0, WHITE);
	draw_text(&format!("First Name: {}", game.player.family_name), sw * 0.7, sh * 0.05, 28.0, WHITE);
	draw_text(&format!("Family Name: {}", game.player.first_name), sw * 0.7, sh * 0.10, 28.0, WHITE);
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

	//Rebirth info
        draw_text(&format!("Some info"), sw * 0.5, sh * 0.35, 28.0, WHITE);
	draw_text(&format!("Somoe more info"), sw * 0.5, sh * 0.4, 28.0, WHITE);
        draw_text(&format!("Some info"), sw * 0.5, sh * 0.45, 28.0, WHITE);
	draw_text(&format!("Somoe more info"), sw * 0.5, sh * 0.5, 28.0, WHITE);
        draw_text(&format!("Some info"), sw * 0.5, sh * 0.55, 28.0, WHITE);
	draw_text(&format!("Somoe more info"), sw * 0.5, sh * 0.6, 28.0, WHITE);

        //Rebirth Button
	if widgets::Button::new("Rebirth")
            .position(vec2(sw * 0.5, sh * 0.65))
            .size(vec2(160.0, 40.0))
            .ui(&mut root_ui())
        {
            game.player.pick_name();
            game.player.generation += 1;
        }

        ScreenTransition::Stay
    }
}