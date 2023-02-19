use macroquad::prelude::*;
mod settings;
use settings::{
    GROUND_COLOR,
    LINE_COLOR,
};
use crate::settings::{SELECTOR_COLOR, TRANSPARENT};


struct Line {
    x: f32,
    speed: f32,
    step: f32,
    thickness: f32,
}

impl Line {
    fn new() -> Line {
        Line {
            x: 0.,
            speed: 300.,
            step: 100.,
            thickness: 1.,
        }
    }

    fn draw(&self) {
        let width = screen_width();
        for i in 0..(width/ self.step) as i32 {
            let mut x = self.x + i as f32 * self.step;
            if x > width {
                x -= width;
            } else if x < 0. {
                x += width;
            } else {
                x = x;
            }
            draw_line(
                x, 0.,
                x, screen_height(),
                1.,
                LINE_COLOR);
        }

        draw_hexagon(
            300. * self.step * 0.01 + self.x, 200.,
            33. * self.step * 0.01,
            1.,
            true,
            DARKGRAY,
            TRANSPARENT
        )

    }

    fn update(&mut self, dt: f32) {
        if is_key_down(KeyCode::Left) {
            self.x -= dt * self.speed;
        }

        if is_key_down(KeyCode::Right) {
            self.x += dt * self.speed;
        }

        let mw = mouse_wheel().1;
        if mw != 0. {
            println!("{}", mw);
            self.step += mw * 0.01 * 0.01 * self.speed ;
            let min_step = 16. * self.thickness;
            if self.step <= min_step {
                self.step = min_step;
            }
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
