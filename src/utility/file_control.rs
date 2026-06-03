use serde_json;
use std::path::Path;

use crate::systems;

pub fn save_game_json(current_game_state: &systems::game_state::GameState) {
    std::fs::create_dir_all("saves").expect("Could not create saves directory!"); //Make sure the saves directory is there
    let json = serde_json::to_string(current_game_state).unwrap();
    std::fs::write("saves/save.json", json).unwrap();
}

pub fn load_game_json() -> systems::game_state::GameState {
    if Path::new("saves/save.json").exists() {
        let json = std::fs::read_to_string("saves/save.json")
            .expect("Could not load the saved file!");
        serde_json::from_str(&json).expect("Could not deserialize from save!")
    } else {
        let game = systems::game_state::GameState::new();
        save_game_json(&game);
        game // just return the new state directly, no recursion needed
    }
}