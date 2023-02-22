use std::time::Duration;

use ggez::{
    event::EventHandler,
    graphics::{self, Color},
    Context, GameError,
};

use super::draw_system::DrawSystem;
use super::states::main_menu::MainMenu;

// Game context object
pub struct GameState {
    p_x: f32,
    p_y: f32,
    dt: Duration,
    main_menu: MainMenu,
    draw_system: DrawSystem,
}

impl GameState {
    //
    pub fn new(ctx: &mut Context, game_name: &str) -> Result<Self, GameError> {
        // Return success
        // ctx.gfx.add_font(
        //     "ToThePointRegular",
        //     FontData::from_path(ctx, "/src/assets/ToThePointRegular-n9y4.ttf").unwrap(),
        // );
        Ok(Self {
            p_x: 0.,
            p_y: 0.,
            dt: Duration::new(0, 0),
            main_menu: MainMenu::default(),
            draw_system: DrawSystem::new(game_name),
        })
    }

    pub fn setup() -> Result<(), GameError> {
        Ok(())
    }
}

/*
 * Event handler trait from GGEZ library
 * See the EventHandler trait definition for more information
 */
impl EventHandler for GameState {
    // Game logic context
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        self.p_x = self.p_x % 800.0 + 1.0;
        self.p_y = self.p_y % 800.0 + 1.0;
        self.dt = ctx.time.delta();
        Ok(())
    }

    // Draw the game
    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        /* ----- BACKGROUND CANVAS ----- */
        // canvas need to be mutable to be borrowed from here
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(36, 36, 36));

        //debugging canvas dimensions
        // dbg!("{:?}", canvas.screen_coordinates());
        // let _canvas_coordinates = canvas.screen_coordinates().unwrap();

        /* ----- TEXT ----- */
        // Text draw from top-left corner
        // let offset = self.frames as f32 / 10.0;
        // let dest_point = vec2(offset, offset);
        // let text = Text::new("Game")
        //     .set_font("ToThePointRegular")
        //     .set_scale(48.);

        /* ----- TITLE TEXT ----- */
        // let game_name = Text::new(
        //     TextFragment::new(self.game_name.to_string())
        //         .color(Color::from_rgb(255, 255, 255))
        //         .scale(PxScale::from(72.0)),
        // );

        // let dimension = game_name.dimensions(ctx).unwrap();

        // /* ----- CANVAS DRAW FUNCTION ----- */
        // canvas.draw(
        //     &game_name,
        //     DrawParam::new().dest([
        //         canvas.screen_coordinates().unwrap().w / 2.0 - dimension.w / 2.0,
        //         canvas.screen_coordinates().unwrap().h / 2.0 - dimension.h / 2.0,
        //     ]),
        // );

        self.draw_system.run(ctx, &mut canvas, &self.main_menu).unwrap();
        canvas.finish(ctx)?;
        Ok(())
    }
}
