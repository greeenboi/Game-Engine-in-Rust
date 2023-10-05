fn main() {
    let mut blue_amount = 0.0;

    greenify::set_event_handler(move || {
        blue_amount += 0.1;
        greenify::clear_screen_to_color(0.0, 0.0, blue_amount, 1.0);
    });
}