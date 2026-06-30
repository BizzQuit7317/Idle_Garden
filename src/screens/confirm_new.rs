use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};

use crate::data;
use crate::systems;
use crate::utility;
use crate::screens::screen::{Screen, ScreenTransition};
use crate::screens::home::Home;
use crate::screens::menu::Menu;
use crate::utility::text_input::{TextInput, InputKind, InputEvent};

pub struct ConfirmNew {
    pub name_input: TextInput,
    pub started: bool,
}

impl ConfirmNew {
    pub fn new() -> Self { 
        ConfirmNew {
            name_input: TextInput::new(20, InputKind::Text),
            started: false,
        }
    }
}

impl Screen for ConfirmNew {
    fn draw(&mut self, game: &mut systems::game_state::GameState) -> ScreenTransition {
        let sw = screen_width();
        let sh = screen_height();

        clear_background(data::constants::DEFAULT_BACKGROUND_COLOR);

        if !self.started {
            self.name_input.focus();
            self.started = true;
        }

        //Text
        let label = "Enter a family name and click continue to start a new game, or cancel to return to the menu.";
        let font_size = 28.0;
        let dims = measure_text(label, None, font_size as u16, 1.0);
        draw_text(
            label,
            sw / 2.0 - dims.width / 2.0,   // centred horizontally
            sh / 4.0,                       // up near the top
            font_size,
            WHITE,
        );

        //Family Name Section
        let _ = self.name_input.update();

        let box_w = 320.0;
        let box_h = 50.0;
        self.name_input.draw(sw / 2.0 - box_w / 2.0, sh / 2.5, box_w, box_h);

        // --- Continue button: only valid if a name was typed ---
        let name = self.name_input.buffer.trim().to_string();
        let has_name = !name.is_empty();

        if widgets::Button::new("Continue")
            .position(vec2(sw / 2.0 - 100.0, sh / 2.0))
            .size(vec2(200.0, 80.0))
            .ui(&mut root_ui())
        {
            if has_name {
                *game = systems::game_state::GameState::new();
                // write the captured name onto whoever owns it:
                game.player.family_name = name; // adapt field
                utility::file_control::save_game_json(game);
                let remaining_dialogue =
                    game.npcs[0].key_dialogue[game.npcs[0].key_dialogue_index..].to_vec();
                game.popups.push_modal(
                    remaining_dialogue,
                    Some(game.npcs[0].first_name.clone()),
                    Some(game.npcs[0].stock.clone()),
                );
                return ScreenTransition::Goto(Box::new(Home::new()));
            }
            // else: no name yet — ignore the click (or flash a warning)
        }

        if widgets::Button::new("Cancel")
            .position(vec2(sw / 2.0 - 100.0, sh / 2.0 + 100.0))
            .size(vec2(200.0, 80.0))
            .ui(&mut root_ui())
        {
            return ScreenTransition::Goto(Box::new(Menu::new()));
        }

        ScreenTransition::Stay
    }
}