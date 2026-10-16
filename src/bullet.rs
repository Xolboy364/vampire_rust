use macroquad::prelude::*;

pub struct Bullet {
    pub pos: Vec2,
    pub dir: Vec2,
    pub speed: f32,
    pub radius: f32,
    pub lifetime: f32,
}

impl Bullet {
    pub fn update(&mut self, dt: f32) {
        self.pos += self.dir * self.speed * dt;
        self.lifetime -= dt;
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, SKYBLUE);
    }
}
