// use std::slice::range;
use macroquad::prelude::*;
mod settings;
use settings::{
    GROUND_COLOR,
    LINE_COLOR,
};
use crate::settings::{SELECTOR_COLOR, TRANSPARENT};


struct Camera {
    x: f32,
    y: f32,
    speed: f32,
    step: f32,
    thickness: f32,
}

impl Camera {
    fn start() -> Camera {
        Camera {
            x: 0.,
            y: 0.,
            speed: 300.,
            step: 100.,
            thickness: 1.,
        }
    }

    fn draw_coordination_greed(&self) {
        let mut range_x = ((screen_width() + self.x.abs()) / self.step) as i32;
        for i in -range_x..=range_x {
            let mut x = (i as f32) * self.step + self.x;
            if x > 0. && x < screen_width() {
                draw_line(
                    x, 0.,
                    x, screen_height(),
                    1.,
                    LINE_COLOR);
            }
        }

        let mut range_y = ((screen_height() + self.y.abs()) / self.step) as i32;
        for i in -range_y..=range_y {
            let mut y = (i as f32) * self.step + self.y;
            if y > 0. && y < screen_width() {
                draw_line(
                    0.,y,
                    screen_width(),y,
                    1.,
                    LINE_COLOR);
            }
        }

    }

    fn draw_hexagon(&self) {
        draw_hexagon(
            300. * self.step * 0.01 + self.x,
            300. * self.step * 0.01 + self.y,
            self.step,
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

        if is_key_down(KeyCode::Up) {
            self.y -= dt * self.speed;
        }

        if is_key_down(KeyCode::Down) {
            self.y += dt * self.speed;
        }

        let mw = mouse_wheel().1;
        if mw != 0. {
            println!("{}", mw);
            let dmw = mw * 0.01 * 0.01 * self.speed;


            let min_step = 16. * self.thickness;
            if self.step + dmw >= min_step {
                self.step += dmw;
                self.x -= (screen_width() / 2.) * (0.01 * 0.01 * self.speed) * (mw.abs()/mw) ;
            }
        }

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
