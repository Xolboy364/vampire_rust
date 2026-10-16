use macroquad::prelude::*;

const PLAYER_SPEED: f32 = 230.0;
const PLAYER_RADIUS: f32 = 16.0;
const BULLET_SPEED: f32 = 550.0;
const BULLET_DAMAGE: f32 = 14.0;

struct Player {
    pos: Vec2,
    radius: f32,
    hp: f32,
    max_hp: f32,
    level: i32,
    xp: i32,
    xp_needed: i32,
    tokens: i32,
}

struct Enemy {
    pos: Vec2,
    hp: f32,
    speed: f32,
    wing_anim: f32,
}

struct Bullet {
    pos: Vec2,
    vel: Vec2,
    life: f32,
    damage: f32,
}

struct Orb {
    pos: Vec2,
}

#[macroquad::main("Vampire Survivor Rust")]
async fn main() {
    let mut player = Player {
        pos: Vec2::ZERO,
        radius: PLAYER_RADIUS,
        hp: 100.0,
        max_hp: 100.0,
        level: 1,
        xp: 0,
        xp_needed: 5,
        tokens: 0,
    };

    let mut enemies: Vec<Enemy> = Vec::new();
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut orbs: Vec<Orb> = Vec::new();
    
    let mut environment: Vec<Vec2> = Vec::new();
    for _ in 0..100 {
        environment.push(Vec2::new(
            rand::gen_range(-1500.0, 1500.0),
            rand::gen_range(-1500.0, 1500.0),
        ));
    }

    let mut kills = 0;
    let mut elapsed = 0.0;
    let mut fire_timer = 0.0;
    let mut spawn_timer = 0.0;

    loop {
        let dt = get_frame_time();
        elapsed += dt;

        let mut dir = Vec2::ZERO;
        let touches = touches();
        if !touches.is_empty() {
            let t = touches[0];
            let screen_center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
            dir = t.position - screen_center;
            if dir.length() > 10.0 {
                dir = dir.normalize();
            } else {
                dir = Vec2::ZERO;
            }
        } else {
            if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) { dir.y -= 1.0; }
            if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) { dir.y += 1.0; }
            if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) { dir.x -= 1.0; }
            if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) { dir.x += 1.0; }
            if dir.length() > 0.0 { dir = dir.normalize(); }
        }
        player.pos += dir * PLAYER_SPEED * dt;

        spawn_timer += dt;
        if spawn_timer > 0.6 {
            spawn_timer = 0.0;
            let angle = rand::gen_range(0.0, std::f32::consts::TAU);
            let dist = 600.0;
            enemies.push(Enemy {
                pos: player.pos + Vec2::new(angle.cos(), angle.sin()) * dist,
                hp: 25.0 + elapsed * 0.8,
                speed: 80.0 + rand::gen_range(0.0, 40.0),
                wing_anim: rand::gen_range(0.0, 10.0),
            });
        }

        fire_timer += dt;
        if fire_timer > 0.4 && !enemies.is_empty() {
            fire_timer = 0.0;
            if let Some(nearest) = enemies.iter().min_by(|a, b| {
                let da = a.pos.distance_squared(player.pos);
                let db = b.pos.distance_squared(player.pos);
                da.partial_cmp(&db).unwrap()
            }) {
                let diff = nearest.pos - player.pos;
                let ang = diff.y.atan2(diff.x);
                bullets.push(Bullet {
                    pos: player.pos,
                    vel: Vec2::new(ang.cos(), ang.sin()) * BULLET_SPEED,
                    life: 1.5,
                    damage: BULLET_DAMAGE + (player.level as f32 - 1.0) * 5.0,
                });
            }
        }

        for b in &mut bullets {
            b.pos += b.vel * dt;
            b.life -= dt;
        }
        bullets.retain(|b| b.life > 0.0);

        for e in &mut enemies {
            e.wing_anim += dt * 15.0;
            let to_player = player.pos - e.pos;
            if to_player.length() > 0.0 {
                e.pos += to_player.normalize() * e.speed * dt;
            }
        }

        let mut dead_enemies = vec![false; enemies.len()];
        let mut dead_bullets = vec![false; bullets.len()];

        for (bi, b) in bullets.iter().enumerate() {
            for (ei, e) in enemies.iter_mut().enumerate() {
                if !dead_enemies[ei] && b.pos.distance(e.pos) < 18.0 {
                    e.hp -= b.damage;
                    dead_bullets[bi] = true;
                    if e.hp <= 0.0 {
                        dead_enemies[ei] = true;
                        orbs.push(Orb { pos: e.pos });
                        kills += 1;
                    }
                    break;
                }
            }
        }

        let mut i = 0;
        enemies.retain(|_| {
            let retain = !dead_enemies[i];
            i += 1;
            retain
        });
        let mut j = 0;
        bullets.retain(|_| {
            let retain = !dead_bullets[j];
            j += 1;
            retain
        });

        for o in &mut orbs {
            let dist = player.pos.distance(o.pos);
            if dist < 120.0 {
                o.pos += (player.pos - o.pos).normalize() * 450.0 * dt;
            }
            if dist < player.radius + 8.0 {
                player.xp += 1;
                player.tokens += 1;
                if player.xp >= player.xp_needed {
                    player.xp -= player.xp_needed;
                    player.level += 1;
                    player.xp_needed = (player.xp_needed as f32 * 1.35) as i32 + 3;
                    player.hp = (player.hp + 25.0).min(player.max_hp);
                }
            }
        }
        orbs.retain(|o| player.pos.distance(o.pos) > player.radius + 8.0);

        for e in &enemies {
            if player.pos.distance(e.pos) < player.radius + 14.0 {
                player.hp -= 10.0 * dt;
                if player.hp <= 0.0 {
                    player.hp = 100.0;
                    kills = 0;
                    elapsed = 0.0;
                    player.level = 1;
                    player.xp = 0;
                    enemies.clear();
                    bullets.clear();
                    orbs.clear();
                }
            }
        }

        clear_background(Color::new(0.06, 0.03, 0.09, 1.0));

        let cam_offset = player.pos - Vec2::new(screen_width() / 2.0, screen_height() / 2.0);

        let grid_size = 100.0;
        let start_x = -cam_offset.x % grid_size;
        let start_y = -cam_offset.y % grid_size;
        let mut x = start_x;
        while x < screen_width() {
            draw_line(x, 0.0, x, screen_height(), 1.0, Color::new(0.11, 0.07, 0.18, 1.0));
            x += grid_size;
        }
        let mut y = start_y;
        while y < screen_height() {
            draw_line(0.0, y, screen_width(), y, 1.0, Color::new(0.11, 0.07, 0.18, 1.0));
            y += grid_size;
        }

        for pos in &environment {
            let screen_pos = *pos - cam_offset;
            if screen_pos.x > -50.0 && screen_pos.x < screen_width() + 50.0 &&
               screen_pos.y > -50.0 && screen_pos.y < screen_height() + 50.0 {
                draw_rectangle(screen_pos.x - 8.0, screen_pos.y - 15.0, 16.0, 25.0, Color::new(0.15, 0.1, 0.23, 1.0));
            }
        }

        for o in &orbs {
            let sp = o.pos - cam_offset;
            draw_circle(sp.x, sp.y, 6.0, RED);
            draw_circle(sp.x - 2.0, sp.y - 2.0, 2.0, PINK);
        }

        for e in &enemies {
            let sp = e.pos - cam_offset;
            draw_ellipse(sp.x, sp.y, 12.0, 8.0, 0.0, Color::new(0.48, 0.13, 0.22, 1.0));
        }

        for b in &bullets {
            let sp = b.pos - cam_offset;
            draw_circle(sp.x, sp.y, 5.0, PINK);
        }

        let px = screen_width() / 2.0;
        let py = screen_height() / 2.0;
        draw_circle(px, py, PLAYER_RADIUS + 4.0, Color::new(0.11, 0.04, 0.18, 1.0));
        draw_circle(px, py, PLAYER_RADIUS, MAROON);
        draw_circle(px, py - 2.0, 8.0, BEIGE);
        draw_rectangle(px - 4.0, py - 5.0, 2.0, 2.0, RED);
        draw_rectangle(px + 2.0, py - 5.0, 2.0, 2.0, RED);

        let ui_text = format!(
            "🦇 Daraja: {} | 🩸 XP: {}/{} | 🪙 Token: {} | 💀 Kills: {} | ❤️ Jon: {:.0}",
            player.level, player.xp, player.xp_needed, player.tokens, kills, player.hp
        );
        draw_text(&ui_text, 20.0, 30.0, 24.0, WHITE);

        next_frame().await;
    }
}
