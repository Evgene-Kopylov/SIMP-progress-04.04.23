use macroquad::prelude::*;

use crate::{UNIT_COLOR, UNIT_SIZE, UNIT_SPEED};

pub struct DarkGrayRectangle {
    rect: Rect,
    d: Vec2,
    zoom: f32,
}

impl DarkGrayRectangle {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5 - UNIT_SIZE.x * 0.5,
                screen_height() * 0.5 - UNIT_SIZE.y * 0.5,
                UNIT_SIZE.x,
                UNIT_SIZE.y,
            ),
            d: Vec2::new(0., 0.),
            zoom: 1.,
        }
    }

    pub fn update(&mut self, dt: f32, d: Vec2, zoom: f32) -> &str {
        self.d = d;
        self.zoom = zoom;
        let mut keycode: &str = "";
        let mut x_move = 0f32;
        if is_key_down(KeyCode::Left) {
            x_move -= 1f32;
            keycode = "Left"
        }
        if is_key_down(KeyCode::Right) {
            x_move += 1f32;
            keycode = "Right"
        }

        let mut y_move = 0f32;
        if is_key_down(KeyCode::Up) {
            y_move -= 1f32;
            keycode = "Up"
        }
        if is_key_down(KeyCode::Down) {
            y_move += 1f32;
            keycode = "Down"
        }

        if self.rect.x < 1f32 {
            x_move = 1f32;
        }

        if self.rect.y < 1f32 {
            y_move = 1f32;
        }

        self.rect.x += x_move * dt * UNIT_SPEED;
        self.rect.y += y_move * dt * UNIT_SPEED;
        keycode
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.rect.x * self.zoom + self.d.x,
            self.rect.y * self.zoom + self.d.y,
            self.rect.w * self.zoom,
            self.rect.h * self.zoom,
            UNIT_COLOR
        );
    }
}