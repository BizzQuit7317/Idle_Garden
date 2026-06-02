use macroquad::prelude::*;

mod data;
mod screens;

//#[macroquad::main("Idle Garden")]
fn window_conf() -> Conf {
    Conf {
        window_title: "Idle Garden".to_string(),
        window_width: data::constants::WINDOWS_DEFAULT_WIDTH,
        window_height: data::constants::WINDOWS_DEFAULT_LENGTH,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut current_page: data::constants::Page = data::constants::Page::Menu; //Keep track of the current page
    loop {

        match current_page {
            data::constants::Page::Menu => { screens::menu::draw(); },
            _ => { screens::default::draw(); } //arm for Default
        }

        next_frame().await;
    }
}