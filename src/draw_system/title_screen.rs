use ggez::{
    graphics::{Canvas, Color, Drawable, Text, TextFragment},
    Context, GameError,
};

pub struct TitleScreen {
    title: TextFragment,
    start: TextFragment,
    settings: TextFragment,
}

impl TitleScreen {
    pub fn new(title: &str) -> Self {
        Self {
            title: TextFragment::new(title).color(Color::WHITE).scale(80.0),
            start: TextFragment::new("Start").color(Color::WHITE).scale(40.0),
            settings: TextFragment::new("Settings")
                .color(Color::WHITE)
                .scale(40.0),
        }
    }

    pub fn run(&self, ctx: &mut Context, canvas: &mut Canvas) -> Result<(), GameError> {
        let screen_coordinates = Canvas::screen_coordinates(canvas).unwrap();
        dbg!("{:?}", screen_coordinates);

        /* ----- TITLE ----- */
        let title = Text::new(self.title.to_owned());

        let title_dimension = title.dimensions(ctx).unwrap();
        dbg!("{:?}", title_dimension);

        let title_destination = [
            screen_coordinates.w / 2.0 - title_dimension.w / 2.0,
            screen_coordinates.h / 2.0 - title_dimension.h,
        ];
        dbg!("{:?}", title_destination);

        /* ----- START ----- */
        let start = Text::new(self.start.to_owned());

        let start_dimension = start.dimensions(ctx).unwrap();
        dbg!("{:?}", start_dimension);

        let start_destination = [
            screen_coordinates.w / 2.0 - start_dimension.w / 2.0,
            screen_coordinates.h - start_dimension.h * 12.0,
        ];
        dbg!("{:?}", start_destination);

        /* ----- SETTINGS ----- */
        let settings = Text::new(self.settings.to_owned());

        let settings_dimension = settings.dimensions(ctx).unwrap();
        dbg!("{:?}", settings_dimension);

        let settings_destination = [
            screen_coordinates.w / 2.0 - settings_dimension.w / 2.0,
            screen_coordinates.h - settings_dimension.h * 10.0,
        ];
        dbg!("{:?}", settings_destination);

        /* ----- ----- */
        // Documentation said graphics::queue_text() deprecated, use Canvas::draw() instead
        Canvas::draw(canvas, &title, title_destination);
        Canvas::draw(canvas, &start, start_destination);
        Canvas::draw(canvas, &settings, settings_destination);
        Ok(())
    }
}
