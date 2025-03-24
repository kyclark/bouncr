use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    let w = screen_width();
    let diameter = 10.0;
    let mut x = 30.0;
    let mut x_add = 1.0;

    loop {
        clear_background(BLACK);

        draw_circle(x, 40.0, diameter, RED);

        if x + diameter / 2.0 == w {
            x_add = -1.0;
        } else if x - diameter / 2.0 == 0.0 {
            x_add = 1.0;
        }

        x += x_add;

        next_frame().await
    }
}
