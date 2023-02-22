use ggez::{
    conf::{FullscreenType, WindowMode, WindowSetup},
    event, ContextBuilder, GameError,
};
mod game_state;
use game_state::GameState;
mod draw_system;
mod states;

// Name of the game
const GAME_TITLE: &str = env!("CARGO_PKG_NAME");
fn main() -> Result<(), GameError> {
    let assets_path = env!("CARGO_MANIFEST_DIR");

    let window_mode = WindowMode::default()
        .dimensions(1280.0, 720.0)
        .fullscreen_type(FullscreenType::Desktop)
        .maximized(true);
    let window_setup = WindowSetup::default().title(GAME_TITLE).vsync(false);

    // Make a context
    let (mut game_ctx, event_loop) = ContextBuilder::new("ggez-game-01", "CeailO")
        .window_mode(window_mode)
        .window_setup(window_setup)
        .build()
        .expect("Context creation error");

    // Make game state context
    let game_state = GameState::new(&mut game_ctx, GAME_TITLE)?;

    // Run the game
    event::run(game_ctx, event_loop, game_state)
}
