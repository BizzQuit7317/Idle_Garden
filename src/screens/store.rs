use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};

use crate::data;
use crate::utility;
use crate::systems;
use crate::screens::screen::{Screen, ScreenTransition};
use crate::screens::home::Home;
use crate::screens::player_inventory::PlayerInventory;
use crate::screens::rebirth::Rebirth;
use crate::subsystems::get_item_definition;

pub enum StoreTab {
    CashBuy,
    ConservationBuy,
    CashSell,
    ConservationSell,
}

struct SlotContext {
    items: Vec<(String, f64, u32)>,
    currency_available: f64,
}

pub struct Store {
    active_tab: StoreTab,
}

impl Store {
    pub fn new() -> Self {
        Store {
            active_tab: StoreTab::CashBuy,
        }
    }
}

impl Screen for Store {
    fn draw(&mut self, game: &mut systems::game_state::GameState) -> ScreenTransition {
        let sw = screen_width();
        let sh = screen_height();

        clear_background(data::constants::DEFAULT_BACKGROUND_COLOR);

        // ── Everything below here is UNCHANGED from original ──────────────────

        //TEST BUTTON FOR REROLLING STORE
        if widgets::Button::new("Reroll")
            .position(vec2(sw * 0.9, sh * 0.9))
            .size(vec2(200.0, 80.0))
            .ui(&mut root_ui())
        {
            game.store.build_stock();
        }

        draw_text(&format!("Cash: {:.2}", game.player.cash), sw * 0.05, sh * 0.05, 28.0, WHITE);
        draw_text(&format!("Conservation: {:.2}", game.player.conservation_points), sw * 0.3, sh * 0.05, 28.0, WHITE);

        if widgets::Button::new("Save")
            .position(vec2(sw * 0.1, sh * 0.15))
            .size(vec2(200.0, 80.0))
            .ui(&mut root_ui())
        {
            utility::file_control::save_game_json(game);
            game.popups.push_toast(String::from("Saved Game"), sw * 0.5, sh * 0.5, 1.0);
        }

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

        let (cash_cost, conservation_cost) = game.player.property.upgrade_cost();
        draw_text(&format!("Cash Cost: {}", cash_cost), sw * 0.7, sh * 0.1, 28.0, WHITE);
        draw_text(&format!("Conservation Cost: {}", conservation_cost), sw * 0.7, sh * 0.15, 28.0, WHITE);

        if widgets::Button::new("Upgrade Property")
            .position(vec2(sw * 0.535, sh * 0.05))
            .size(vec2(200.0, 80.0))
            .ui(&mut root_ui())
        {
            if !game.player.upgrade_property() {
                game.popups.push_toast(String::from("Cannot Afford that buddy"), sw * 0.5, sh * 0.5, 1.0);
            }
        }

        // ── NEW: Store tab buttons, pushed down below existing UI ─────────────
        // Original grid started at sh * 0.35, tab buttons sit just above that
        if widgets::Button::new("Buy (Cash)")
            .position(vec2(sw * 0.1, sh * 0.36))
            .size(vec2(160.0, 40.0))
            .ui(&mut root_ui())
        {
            self.active_tab = StoreTab::CashBuy;
        }

        /*
        if widgets::Button::new("Buy (Conservation)")
            .position(vec2(sw * 0.28, sh * 0.36))
            .size(vec2(190.0, 40.0))
            .ui(&mut root_ui())
        {
            self.active_tab = StoreTab::ConservationBuy;
        }
        */ //re enable once the player has reached a certain property can worry about later

        if widgets::Button::new("Sell (Cash)")
            .position(vec2(sw * 0.53, sh * 0.36))
            .size(vec2(160.0, 40.0))
            .ui(&mut root_ui())
        {
            self.active_tab = StoreTab::CashSell;
        }

        /*
        if widgets::Button::new("Sell (Conservation)")
            .position(vec2(sw * 0.71, sh * 0.36))
            .size(vec2(190.0, 40.0))
            .ui(&mut root_ui())
        {
            self.active_tab = StoreTab::ConservationSell;
        }
        */ //re enable once the player has reached a certain property can worry about later

        // ── Build SlotContext ─────────────────────────────────────────────────
        let ctx = match self.active_tab {
            StoreTab::CashBuy => SlotContext {
                items: game.store.stock.iter()
                    .map(|i| (i.item_id.clone(), i.price, i.quantity_available))
                    .collect(),
                currency_available: game.player.cash,
            },
            StoreTab::ConservationBuy => SlotContext {
                // TODO: swap i.price for i.conservation_price once added to StoreItem
                items: game.store.stock.iter()
                    .map(|i| (i.item_id.clone(), i.conservation_price, i.quantity_available))
                    .collect(),
                currency_available: game.player.conservation_points,
            },
            StoreTab::CashSell => SlotContext {
                items: game.player.inventory.items.iter()
                    .map(|(id, &qty)| {
                        let price = get_item_definition(id)
                            .map(|def| def.cash_value)
                            .unwrap_or(0.0);
                        (id.clone(), price, qty as u32)
                    })
                    .collect(),
                currency_available: f64::INFINITY,
            },
            StoreTab::ConservationSell => SlotContext {
                items: game.player.inventory.items.iter()
                    .map(|(id, &qty)| {
                        let price = get_item_definition(id)
                            .map(|def| def.conservation_value)
                            .unwrap_or(0.0);
                        (id.clone(), price, qty as u32)
                    })
                    .collect(),
                currency_available: f64::INFINITY,
            },
        };

        // ── Grid, shifted down to sh * 0.43 to sit below the tab buttons ─────
        let cols = 3usize;
        let rows = (game.store.stock_limit as usize + cols - 1) / cols;
        let grid_top = sh * 0.43;
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

            if let Some((item_id, price, quantity)) = ctx.items.get(slot) {
                let display_name = get_item_definition(item_id)
                    .map(|def| def.display_name)
                    .unwrap_or(item_id.as_str());

                let currency_symbol = match self.active_tab {
                    StoreTab::CashBuy | StoreTab::CashSell => "£",
                    StoreTab::ConservationBuy | StoreTab::ConservationSell => "",
                };

                let label = format!("{}\n{}{:.2}\nQTY: {}", display_name, currency_symbol, price, quantity);

                if widgets::Button::new(label.as_str())
                    .position(vec2(x + 5.0, y + 5.0))
                    .size(vec2(slot_width - 10.0, slot_height - 10.0))
                    .ui(&mut root_ui())
                {
                    match self.active_tab {
                        StoreTab::CashBuy => {
                            if ctx.currency_available >= *price {
                                let cost = game.store.try_buy(slot, game.player.cash, false);
                                if cost > 0.0 {
                                    game.player.cash -= cost;
                                    game.player.inventory.add(item_id, 1);
                                    game.store.stock.retain(|i| i.quantity_available > 0);
                                }
                            }
                        }
                        StoreTab::ConservationBuy => {
                            if ctx.currency_available >= *price {
                                let cost = game.store.try_buy(slot, game.player.conservation_points, true);
                                if cost > 0.0 {
                                    game.player.conservation_points -= cost;
                                    game.player.inventory.add(item_id, 1);
                                    game.store.stock.retain(|i| i.quantity_available > 0);
                                }
                            }
                        }
                        StoreTab::CashSell => {
                            if *quantity > 0 {
                                game.player.inventory.remove(item_id, 1);
                                game.player.cash += price;
                                game.player.cash_current_rebirth += price;
                            }
                        }
                        StoreTab::ConservationSell => {
                            if *quantity > 0 {
                                game.player.inventory.remove(item_id, 1);
                                game.player.conservation_points += price;
                                game.player.conservation_points_current_rebirth += price;
                            }
                        }
                    }
                }
            } else {
                draw_rectangle(x + 5.0, y + 5.0, slot_width - 10.0, slot_height - 10.0, Color::new(0.0, 0.0, 0.0, 0.4));
                draw_text("Empty", x + 15.0, y + slot_height * 0.5, 22.0, Color::new(0.6, 0.6, 0.6, 1.0));
            }
        }

        ScreenTransition::Stay
    }
}