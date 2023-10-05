fn main() {
    greenify::set_event_handler(move |key| match key {
        greenify::Key::Left => greenify::clear_screen_to_color(1.0, 0.0, 0.0, 1.0),
        greenify::Key::Right => greenify::clear_screen_to_color(0.0, 1.0, 0.0, 1.0),
        greenify::Key::Up => greenify::clear_screen_to_color(0.0, 0.0, 1.0, 1.0),
        greenify::Key::Down => greenify::clear_screen_to_color(0.0, 1.0, 1.0, 1.0),
        greenify::Key::Space => greenify::clear_screen_to_color(1.0, 1.0, 0.0, 1.0),
    });
}