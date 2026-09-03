use bevy::{
    app::{App, Plugin},
    color::Color,
    input_focus::InputFocus,
};

pub struct InterfacePlugin;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

// Plugin root
impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app // `InputFocus` must be set for accessibility to recognize the button.
            .init_resource::<InputFocus>();
    }
}
