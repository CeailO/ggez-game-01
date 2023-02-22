pub enum MainMenu {
    TitleScreen,
    Settings
}

impl Default for MainMenu {
    fn default() -> Self {
        Self::TitleScreen
        //Self::Settings
    }
}
