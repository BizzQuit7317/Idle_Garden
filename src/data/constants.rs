use macroquad::prelude::*;
//Default UI Colors
pub const DEFAULT_BACKGROUND_COLOR: Color = RED;

//UI Default Scale
pub const WINDOWS_DEFAULT_WIDTH: i32 = 1280;
pub const WINDOWS_DEFAULT_LENGTH: i32 = 720;

//Page enum
pub enum Page {
    Default,
    Menu,
    Balcony,
}