fn main() {
    let mut x_position: f32 = 20.0;
    let mut y_position: f32 = 30.0;

    greenify::set_event_handler(move |key | {
        let move_amount: f32 = 20.0;

        match key {
            greenify::Key::Left => {
                if x_position - move_amount/2.0 >= 0.0 {
                    x_position -= move_amount/2.0;
                }
            },
            greenify::Key::Right => {
                if !( x_position + move_amount/2.0 < 0.0) {
                    x_position += move_amount/2.0;
                }
            },
            greenify::Key::Up => {
                if !(y_position + move_amount/2.0 <= 0.0){
                    y_position += move_amount/2.0;
                }
            },
            greenify::Key::Down => {
                if y_position - move_amount/2.0 >= 0.0 {
                    y_position -= move_amount/2.0;
                }
            },
            greenify::Key::Space => {}
        }

        greenify::clear_screen_to_color(0.0, 0.0, 0.3, 1.0);
        greenify::draw_rectangle(x_position, y_position, 10., 10.);
    })  
        
}

