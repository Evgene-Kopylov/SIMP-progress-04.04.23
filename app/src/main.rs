use macroquad::prelude::*;
mod settings;
mod camera;
mod unit;

use crate::settings::{GROUND_COLOR, UNIT_COLOR, UNIT_SIZE, UNIT_SPEED};
use crate::camera::Camera;
use crate::unit::Unit;


#[macroquad::main("breakout")]
async fn main() {
    let mut camera = Camera::start();
    let mut unit = Unit::new();


    loop {
        clear_background(GROUND_COLOR);

        let (d, zoom) = camera.update(get_frame_time());
        camera.draw_coordination_greed();
        camera.draw_hexagon();

        unit.update(get_frame_time(), d, zoom);
        unit.draw();

        next_frame().await
    }
}
