// Encapsulation imgui functionality
impl ImGuiWrapper {
    // Take ggez context
    pub fn new(ctx: &mut Context) -> Self {}

    // Calling every render iteration
    pub fn render(&mut self, ctx: &mut Context) {}

    // Updating the mouse position
    pub fn update_mouse_position(&mut self, x: i32, y: i32) {}

    // Three different state for mouse button press for left, right and middle
    pub fn mouse_pressed(&mut self, pressed: (bool, bool, bool)) {}

    // Two different state for mouse button press for up and down
    pub fn middle_mouse_scroll(&mut self, pressed: (bool, bool)) {}
}
