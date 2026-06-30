use macroquad::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum InputKind {
    Text,
    Numeric,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum InputEvent {
    Editing,
    Submitted(String),
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextInput {
    pub buffer: String,
    pub max_len: usize,
    pub kind: InputKind,
    pub focused:  bool,
}

impl TextInput {
    pub fn new(max_len: usize, kind: InputKind)  -> Self {
        Self {
            buffer: String::new(),
            max_len,
            kind,
            focused: false
        }
    }

    pub fn focus(&mut self) {
        self.focused = true;
        while  get_char_pressed().is_some() {}
    }

    pub fn update(&mut self) -> InputEvent {
        if  !self.focused {
            return InputEvent::Editing;
        }

        if is_key_pressed(KeyCode::Enter) {
            let trimmed = self.buffer.trim().to_string();
            if !trimmed.is_empty() {
                return InputEvent::Submitted(trimmed);
            }

        }

        if is_key_pressed(KeyCode::Backspace) {
            self.buffer.pop();
        }

        while let Some(c) = get_char_pressed() {
            if (c as u32) < 0x20 || c == '\u{7f}' {
                continue;
            }
            if self.kind == InputKind::Numeric && !c.is_ascii_digit() {
                continue;
            }
            if self.buffer.chars().count() < self.max_len {
                self.buffer.push(c);
            }
        }

        InputEvent::Editing
    }

    pub fn draw(&self, x: f32, y: f32, w: f32, h: f32) {
        // --- your "art frame" would go here; placeholder box: ---
        draw_rectangle(x, y, w, h, Color::from_rgba(30, 30, 30, 255));
        draw_rectangle_lines(
            x, y, w, h, 2.0,
            if self.focused { GREEN } else { GRAY },
        );

        let font_size = 28.0;
        let text_x = x + 8.0;
        let text_y = y + h / 2.0 + font_size / 3.0;
        draw_text(&self.buffer, text_x, text_y, font_size, WHITE);

        // blinking caret: pulse on real elapsed time, not frame count
        if self.focused {
            let blink_on = (get_time() * 2.0) as i64 % 2 == 0;
            if blink_on {
                let text_w = measure_text(&self.buffer, None, font_size as u16, 1.0).width;
                draw_rectangle(text_x + text_w + 2.0, y + 8.0, 2.0, h - 16.0, WHITE);
            }
        }
    }
}