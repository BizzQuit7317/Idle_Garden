use crate::systems::store_state::StoreItem;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NPCViewState { 
    Dialogue, 
    Store 
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NPC {
    pub id: String,
    pub family_name: String,
    pub first_name: String,
    pub key_dialogue: Vec<String>,
    pub key_dialogue_index: usize,
    pub ambient_dialogue: Vec<String>,
    pub generation: u8,
    pub stock: Vec<StoreItem>,
}

pub fn load_npc_data() -> Vec<NPC> {
    let json = include_str!("assets/npcs.json");
    serde_json::from_str(json).expect("Failed to parse npcs.json")
}

impl NPC {
    pub fn get_current_dialogue(&self) -> Option<&String> {
        if self.key_dialogue_index < self.key_dialogue.len() {
            Some(&self.key_dialogue[self.key_dialogue_index])
        } else {
            // They've finished key dialogue, pick from ambient pool
            self.ambient_dialogue.get(0)  // or random pick
        }
    }
    
    pub fn advance_dialogue(&mut self) {
        if self.key_dialogue_index < self.key_dialogue.len() {
            self.key_dialogue_index += 1;
        }
    }
}