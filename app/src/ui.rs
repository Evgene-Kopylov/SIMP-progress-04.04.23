use macroquad::prelude::Vec2;

pub trait UI {
    fn visible_coords(&self) -> Vec2;
    fn visible_size(&self) -> f32;
}