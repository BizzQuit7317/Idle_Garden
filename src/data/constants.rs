use macroquad::prelude::*;
use serde::{Serialize, Deserialize};

//Default UI Colors
pub const DEFAULT_BACKGROUND_COLOR: Color = RED;

//UI Default Scale
pub const WINDOWS_DEFAULT_WIDTH: i32 = 1280;
pub const WINDOWS_DEFAULT_LENGTH: i32 = 720;

//Page enum
#[derive(Debug, Serialize, Deserialize)]
pub enum Page {
    Default,
    Menu,
    Balcony,
}

//Differnt Houses enum
#[derive(Debug, Serialize, Deserialize)]
pub enum Houses {
    Balcony,
    Terrace,
}