use macroquad::prelude::*;
mod settings;
use settings::{
    GROUND_COLOR,
    LINE_COLOR,
};
use crate::settings::{SELECTOR_COLOR, TRANSPARENT};


struct Camera {
    x: f32,
    speed: f32,
    step: f32,
    thickness: f32,
}

impl Camera {
    fn start() -> Camera {
        Camera {
            x: 0.,
            speed: 300.,
            step: 100.,
            thickness: 1.,
        }
    }

    fn draw_coordination_greed(&self) {
        let range = screen_width() / self.step;
        for i in 0..=range as i32 {
            let mut x = self.x + i as f32 * self.step;
            while x < 0. {
                x += screen_width();
            }
            while x > screen_width() {
                x -= screen_width();
            }

            draw_line(
                x, 0.,
                x, screen_height(),
                1.,
                LINE_COLOR);
        }
    }

    fn draw_hexagon(&self) {
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

        // if is_key_down(KeyCode::Space) {
        //
        //     println!("{}", mouse_position().0);
        // }

    }


}



#[macroquad::main("breakout")]
async fn main() {
    let mut line = Camera::start();

    loop {
        clear_background(GROUND_COLOR);


        line.update(get_frame_time());
        line.draw_coordination_greed();
        line.draw_hexagon();

        next_frame().await
    }
}
