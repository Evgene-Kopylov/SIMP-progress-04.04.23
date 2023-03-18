use macroquad::prelude::*;
mod settings;
mod camera;

use crate::settings::{
    GROUND_COLOR
};
use crate::camera::Camera;


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
