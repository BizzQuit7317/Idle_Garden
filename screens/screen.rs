use crate::systems::game_state::GameState;

pub enum ScreenTransition {
    Stay,
    Goto(Box<dyn Screen>),
}

pub trait Screen {
    fn draw(&mut self, game: &mut GameState) -> ScreenTransition;
}