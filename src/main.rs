use tetra::ContextBuilder;

pub mod gamestate;

use gamestate::GameState;

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", gamestate::WINDOW_WIDTH as i32, gamestate::WINDOW_HEIGHT as i32)
    .quit_on_escape(true)
    .build()?
    .run(GameState::new)
}