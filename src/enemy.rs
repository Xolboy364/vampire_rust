use macroquad::prelude::*;

pub struct Enemy {
    pub pos: Vec2,
    pub speed: f32,
    pub radius: f32,
}

impl Enemy {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            speed: 120.0,
            radius: 10.0,
        }
    }

    pub fn update(&mut self, player_pos: Vec2, dt: f32) {
        let to_player = player_pos - self.pos;
        if to_player.length_squared() > 0.0 {
            let enemy_dir = to_player.normalize();
            self.pos += enemy_dir * self.speed * dt;
        }
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, RED);
    }
}
