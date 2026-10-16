use macroquad::prelude::*;

pub struct Player {
    pub pos: Vec2,
    pub speed: f32,
    pub radius: f32,
    pub hp: i32,
    pub max_hp: i32,
    pub xp: i32,
    pub level: i32,
    pub next_level_xp: i32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: vec2(x, y),
            speed: 200.0,
            radius: 14.0,
            hp: 100,
            max_hp: 100,
            xp: 0,
            level: 1,
            next_level_xp: 50,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let mut dir = Vec2::ZERO;
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) { dir.y -= 1.0; }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) { dir.y += 1.0; }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) { dir.x -= 1.0; }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) { dir.x += 1.0; }

        if dir.length_squared() > 0.0 {
            dir = dir.normalize();
            self.pos += dir * self.speed * dt;
        }

        self.pos.x = self.pos.x.clamp(self.radius, screen_width() - self.radius);
        self.pos.y = self.pos.y.clamp(self.radius, screen_height() - self.radius);
    }

    pub fn add_xp(&mut self, amount: i32) -> bool {
        self.xp += amount;
        if self.xp >= self.next_level_xp {
            self.xp -= self.next_level_xp;
            self.level += 1;
            self.next_level_xp += 30;
            true // Level up sodir bo'ldi!
        } else {
            false
        }
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, YELLOW);
    }
}
