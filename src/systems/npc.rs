use crate::systems::store_state::StoreItem;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum NPCViewState { 
    Dialogue, 
    Store 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NPC {
    pub id: String,
    pub family_name: String,
    pub first_name: String,
    pub key_dialogue: Vec<String>,
    pub ambient_dialogue: Vec<String>,
    pub generation: u8,
    pub stock: Vec<StoreItem>,
}

pub fn load_npc_data() -> Vec<NPC> {
    let json = include_str!("assets/npcs.json");
    serde_json::from_str(json).expect("Failed to parse npcs.json")
}