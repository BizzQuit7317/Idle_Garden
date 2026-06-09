use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets, hash};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Toast {
    pub message: String,
    pub x: f32,
    pub y: f32,
    pub lifetime: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Modal {
    pub message: Vec<String>,
    pub dismissed: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
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

    pub fn push_modal(&mut self, message: Vec<String>) {
        self.modals.push(Modal { message, dismissed: false });
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

        self.modals.retain(|m| !m.dismissed);

        !self.modals.is_empty()
    }
}