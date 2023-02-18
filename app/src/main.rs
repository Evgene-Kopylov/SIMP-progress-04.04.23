use macroquad::prelude::*;
mod settings;
use settings::{
    GROUND_COLOR
};
use crate::settings::SELECTOR_COLOR;


struct Line {
    x: f32,
    speed: f32,
}

impl Line {
    fn new() -> Line {
        Line {
            x: 0.,
            speed: 100.,
        }
    }

    fn draw(&self) {
        draw_line(
            self.x, 0.,
            self.x, screen_height(),
            1.,
            SELECTOR_COLOR);
    }

    fn update(&mut self, dt: f32) {
        if is_key_down(KeyCode::Left) {
            self.x -= dt * self.speed;
        }

        if is_key_down(KeyCode::Right) {
            self.x += dt * self.speed;
        }
    }


}



#[macroquad::main("breakout")]
async fn main() {
    let mut line = Line::new();

    loop {
        clear_background(GROUND_COLOR);


        line.update(get_frame_time());
        line.draw();


        next_frame().await
    }
}
