use macroquad::prelude::{Color, Vec2};

pub trait UI {
    fn pos(&self) -> Vec2;
    fn zoom(&self) -> f32;
    fn d(&self) -> Vec2;
    fn size(&self) -> f32;
    fn unit_color(&self) -> Color;

    fn visible_coords(&self) -> Vec2 {
        Vec2::new(
            self.pos().x * self.zoom() + self.d().x,
            self.pos().y * self.zoom() + self.d().y,
        )
    }

    fn visible_size(&self) -> f32 {
        self.size() * self.zoom()
    }
}