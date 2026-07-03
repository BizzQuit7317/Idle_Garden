use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets, hash};
use serde::{Serialize, Deserialize};
use crate::systems::store_state::StoreItem;
use crate::systems::npc::NPCViewState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Toast {
    pub message: String,
    pub x: f32,
    pub y: f32,
    pub lifetime: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Modal {
    pub message: Vec<String>,
    pub dismissed: bool,
    pub npc_flag: bool, //true if for npc, false if regular message
    pub npc_name: Option<String>, //if npc need to take string otherwise useless
    pub npc_state: Option<NPCViewState>, //true if npc dialogue, false if npc store
    pub npc_stock: Option<Vec<StoreItem>>, //a refernce to the npc stock to display
    pub current_line: usize,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PopupQueue {
    pub toasts: Vec<Toast>,
    pub modals: Vec<Modal>,
}

impl PopupQueue {
    pub fn new() -> PopupQueue {
        PopupQueue {
            toasts: vec![],
            modals: vec![],
        }
    }

    pub fn push_toast(&mut self, message: String, x: f32, y: f32, lifetime: f64) {
        self.toasts.push(Toast { message, x, y, lifetime });
    }

    pub fn push_modal(&mut self, message: Vec<String>, npc_name: Option<String>, npc_stock: Option<Vec<StoreItem>>) {
        self.modals.push(Modal { 
            message, 
            dismissed: false, 
            npc_flag: npc_name.is_some(),
            npc_name: npc_name.clone(),
            npc_state: npc_name.as_ref().map(|_| NPCViewState::Dialogue), // start in Dialogue if NPC
            npc_stock,
            current_line: 0,
        });
    }

    pub fn tick(&mut self, dt: f64) {
        for toast in &mut self.toasts {
            toast.lifetime -= dt;
        }
        self.toasts.retain(|t| t.lifetime > 0.0);
    }

    pub fn draw(&mut self) -> bool {
        let sw = screen_width();
        let sh = screen_height();

        // Draw toasts
        for toast in &self.toasts {
            draw_text(&toast.message, toast.x, toast.y, 28.0, WHITE);
        }

        // Draw modals — extract message first to avoid borrow conflict inside closure
        for i in 0..self.modals.len() {
            if self.modals[i].npc_flag {
                match self.modals[i].npc_state {
                    Some(NPCViewState::Dialogue) => {
                        let current_line = self.modals[i].message[self.modals[i].current_line].clone();
                        let npc_name = self.modals[i].npc_name.clone().unwrap_or("Unknown".to_string());
                        let mut close = false;
                        let mut switch_to_store = false;

                        draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.5));

                        // NPC character box (right side) - drawn with root_ui to sit above buttons
                        root_ui().window(
                            hash!("npc_character", i),
                            vec2(sw * 0.65, sh * 0.3),
                            vec2(sw * 0.15, sh * 0.45),
                            |ui| {
                                ui.label(None, "[Character]");
                            }
                        );

                        // NPC dialogue window
                        root_ui().window(
                            hash!("npc_dialogue", i),
                            vec2(0.0, sh * 0.75),
                            vec2(sw, sh * 0.25),
                            |ui| {
                                if ui.button(None, "Next") {
                                    if self.modals[i].current_line < self.modals[i].message.len() - 1 {
                                        self.modals[i].current_line += 1;
                                    }
                                }
                                if ui.button(None, "View Upgrades") {
                                    switch_to_store = true;
                                }
                                if ui.button(None, "Close") {
                                    close = true;
                                }
                                ui.label(None, &current_line);  // Display ONLY current line, not all
                            }
                        );

                        // NPC name box
                        draw_rectangle(sw * 0.02, sh * 0.72, sw * 0.15, sh * 0.05, Color::new(0.2, 0.2, 0.2, 1.0));
                        draw_text(
                            &npc_name,
                            sw * 0.04,
                            sh * 0.745,
                            20.0,
                            WHITE
                        );

                        if close {
                            self.modals[i].dismissed = true;
                        } else if switch_to_store {
                            self.modals[i].npc_state = Some(NPCViewState::Store);
                        }
                    },
                    Some(NPCViewState::Store) => {
                        let npc_name = self.modals[i].npc_name.clone().unwrap_or("Unknown".to_string());
                        let stock = self.modals[i].npc_stock.clone().unwrap_or_default();
                        let mut close = false;
                        let mut switch_to_dialogue = false;

                        draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.5));

                        // NPC character box
                        root_ui().window(
                            hash!("npc_character", i),
                            vec2(sw * 0.65, sh * 0.3),
                            vec2(sw * 0.15, sh * 0.45),
                            |ui| {
                                ui.label(None, "[Character]");
                            }
                        );

                        // NPC store window
                        root_ui().window(
                            hash!("npc_store", i),
                            vec2(0.0, sh * 0.75),
                            vec2(sw, sh * 0.25),
                            |ui| {
                                if ui.button(None, "Back") {
                                    switch_to_dialogue = true;
                                }
                                if ui.button(None, "Close") {
                                    close = true;
                                }
                                ui.label(None, "Upgrades:");

                                for (item_index, item) in stock.iter().enumerate() {
                                    // Look up the display name; fall back to the raw id if not found
                                    let display_name = crate::subsystems::get_item_definition(&item.item_id)
                                        .map(|d| d.display_name)
                                        .unwrap_or(item.item_id.as_str());

                                    let label = format!("{} - {} cash##{}", display_name, item.price, item_index);

                                    if ui.button(None, label.as_str()) {
                                        println!("[DBG] Clicked store item: {} ({})", display_name, item.item_id); //replace withh tor buying logic at some point
                                    }
                                }
                            }
                        );

                        // NPC name box
                        draw_rectangle(sw * 0.02, sh * 0.72, sw * 0.15, sh * 0.05, Color::new(0.2, 0.2, 0.2, 1.0));
                        draw_text(
                            &npc_name,
                            sw * 0.04,
                            sh * 0.745,
                            20.0,
                            WHITE
                        );

                        if close {
                            self.modals[i].dismissed = true;
                        } else if switch_to_dialogue {
                            self.modals[i].npc_state = Some(NPCViewState::Dialogue);
                        }
                    },
                    None => {}
                }
            } else {
                let message = self.modals[i].message.clone();
                let mut close = false;

                draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.5));

                root_ui().window(
                    hash!("modal_message", i),
                    vec2(sw * 0.2, sh * 0.2),
                    vec2(sw * 0.6, sh * 0.6),
                    |ui| {
                        if ui.button(None, "Close") {
                            close = true;
                        }
                        for line in &message {
                            ui.label(None, line);
                        }
                    }
                );

                if close {
                    self.modals[i].dismissed = true;
                }
            }
        }

        self.modals.retain(|m| !m.dismissed);

        !self.modals.is_empty()
    }
}