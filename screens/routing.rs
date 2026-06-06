use crate::systems::player::{Player, Property};
use crate::screens::screen::Screen;
use crate::screens::balcony::Balcony;
use crate::screens::terrace::Terrace;

pub fn screen_for_player(player: &Player) -> Box<dyn Screen> {
    match player.property {
        Property::Balcony => Box::new(Balcony::new()),
        Property::Terrace => Box::new(Terrace::new()),
    }
}