pub mod title_screen;

use ggez::{graphics::Canvas, Context, GameError};

use crate::states::main_menu::MainMenu;

use super::draw_system::title_screen::TitleScreen;

pub struct DrawSystem {
    title_screen: TitleScreen,
}

impl DrawSystem {
    pub fn new(game_title: &str) -> Self {
        Self {
            title_screen: TitleScreen::new(game_title),
        }
    }

    pub fn run(
        &self,
        ctx: &mut Context,
        canvas: &mut Canvas,
        main_menu: &MainMenu,
    ) -> Result<(), GameError> {
        match main_menu {
            MainMenu::TitleScreen => self.title_screen.run(ctx, canvas)?,
            MainMenu::Settings => {}
        }
        Ok(())
    }
}
