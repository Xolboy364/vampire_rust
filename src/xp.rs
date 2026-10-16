use macroquad::prelude::*;

pub struct XpCrystal {
    pub pos: Vec2,
    pub value: i32,
    pub radius: f32,
}

impl XpCrystal {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            value: 10,
            radius: 5.0,
        }
    }

    pub fn update(&mut self, player_pos: Vec2, dt: f32) {
        // Magnit effekti: o'yinchi yaqinlashsa (masalan, 160 px radiusda) kristal o'yinchi tomon uchadi
        let dist_sq = (player_pos - self.pos).length_squared();
        if dist_sq < 160.0 * 160.0 {
            let dir = (player_pos - self.pos).normalize_or_zero();
            self.pos += dir * 300.0 * dt;
        }
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, GREEN);
    }
}
