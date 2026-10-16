mod player;
mod enemy;
mod bullet;
mod xp;

use macroquad::prelude::*;
use player::Player;
use enemy::Enemy;
use bullet::Bullet;
use xp::XpCrystal;

#[derive(PartialEq)]
enum GameState {
    Playing,
    LevelUp,
    GameOver,
}

#[macroquad::main("Vampire Survivor")]
async fn main() {
    let mut player = Player::new(screen_width() / 2.0, screen_height() / 2.0);
    let mut enemies: Vec<Enemy> = vec![];
    let mut bullets: Vec<Bullet> = vec![];
    let mut crystals: Vec<XpCrystal> = vec![];
    
    let mut spawn_timer = 0.0;
    let mut shoot_timer = 0.0;
    let mut score = 0;
    let mut hit_cooldown = 0.0;
    let mut game_state = GameState::Playing;

    loop {
        let dt = get_frame_time();

        // --- 1. LEVEL UP MENYUSI HOLATI ---
        if game_state == GameState::LevelUp {
            clear_background(BLACK);

            draw_text("DARAJA OSHDI! BONUS TANLANG:", screen_width() / 2.0 - 180.0, screen_height() / 2.0 - 80.0, 22.0, YELLOW);
            draw_text("[1] Harakat tezligini oshirish (+Speed)", screen_width() / 2.0 - 180.0, screen_height() / 2.0 - 30.0, 18.0, WHITE);
            draw_text("[2] Jonni to'liq tiklash (+HP)", screen_width() / 2.0 - 180.0, screen_height() / 2.0 + 10.0, 18.0, WHITE);
            draw_text("[3] Katta XP bonusi (+50 XP)", screen_width() / 2.0 - 180.0, screen_height() / 2.0 + 50.0, 18.0, WHITE);

            if is_key_pressed(KeyCode::Key1) || is_key_pressed(KeyCode::Kp1) {
                player.speed += 35.0;
                game_state = GameState::Playing;
            } else if is_key_pressed(KeyCode::Key2) || is_key_pressed(KeyCode::Kp2) {
                player.hp = player.max_hp;
                game_state = GameState::Playing;
            } else if is_key_pressed(KeyCode::Key3) || is_key_pressed(KeyCode::Kp3) {
                player.add_xp(50);
                game_state = GameState::Playing;
            }

            next_frame().await;
            continue;
        }

        // --- 2. O'YIN TUGAGAN HOLATI ---
        if game_state == GameState::GameOver {
            clear_background(BLACK);
            draw_text("O'YIN TUGADI!", screen_width() / 2.0 - 80.0, screen_height() / 2.0 - 20.0, 24.0, RED);
            draw_text("Qayta boshlash uchun ENTER bosing", screen_width() / 2.0 - 130.0, screen_height() / 2.0 + 20.0, 16.0, LIGHTGRAY);

            if is_key_pressed(KeyCode::Enter) {
                player = Player::new(screen_width() / 2.0, screen_height() / 2.0);
                enemies.clear();
                bullets.clear();
                crystals.clear();
                score = 0;
                game_state = GameState::Playing;
            }

            next_frame().await;
            continue;
        }

        // --- 3. O'YIN JARAYONI (PLAYING) ---
        if hit_cooldown > 0.0 {
            hit_cooldown -= dt;
        }

        player.update(dt);

        // Dushmanlarni spawn qilish
        spawn_timer += dt;
        if spawn_timer > 0.7 {
            spawn_timer = 0.0;
            let angle = rand::gen_range(0.0, std::f32::consts::TAU);
            let spawn_dist = 450.0;
            let enemy_pos = player.pos + vec2(angle.cos(), angle.sin()) * spawn_dist;
            enemies.push(Enemy::new(enemy_pos));
        }

        // Avtomatik o'q otish
        shoot_timer += dt;
        if shoot_timer > 0.5 && !enemies.is_empty() {
            shoot_timer = 0.0;
            let mut closest_pos = enemies[0].pos;
            let mut min_dist_sq = (closest_pos - player.pos).length_squared();

            for enemy in &enemies {
                let dist_sq = (enemy.pos - player.pos).length_squared();
                if dist_sq < min_dist_sq {
                    min_dist_sq = dist_sq;
                    closest_pos = enemy.pos;
                }
            }

            let bullet_dir = (closest_pos - player.pos).normalize_or_zero();
            if bullet_dir != Vec2::ZERO {
                bullets.push(Bullet {
                    pos: player.pos,
                    dir: bullet_dir,
                    speed: 400.0,
                    radius: 5.0,
                    lifetime: 2.0,
                });
            }
        }

        for enemy in &mut enemies {
            enemy.update(player.pos, dt);
        }

        for crystal in &mut crystals {
            crystal.update(player.pos, dt);
        }

        for bullet in &mut bullets {
            bullet.update(dt);
        }
        bullets.retain(|b| b.lifetime > 0.0);

        // To'qnashuvlar (Collision)
        let mut bullets_to_remove = vec![];
        let mut enemies_to_remove = vec![];
        let mut crystals_to_remove = vec![];

        for (b_idx, bullet) in bullets.iter().enumerate() {
            for (e_idx, enemy) in enemies.iter().enumerate() {
                if (bullet.pos - enemy.pos).length_squared() < (bullet.radius + enemy.radius).powi(2) {
                    bullets_to_remove.push(b_idx);
                    enemies_to_remove.push(e_idx);
                    score += 10;
                }
            }
        }

        for &e_idx in &enemies_to_remove {
            if e_idx < enemies.len() {
                crystals.push(XpCrystal::new(enemies[e_idx].pos));
            }
        }

        for (c_idx, crystal) in crystals.iter().enumerate() {
            if (player.pos - crystal.pos).length_squared() < (player.radius + crystal.radius).powi(2) {
                crystals_to_remove.push(c_idx);
                let leveled_up = player.add_xp(crystal.value);
                if leveled_up {
                    game_state = GameState::LevelUp;
                }
            }
        }

        for enemy in &enemies {
            if (player.pos - enemy.pos).length_squared() < (player.radius + enemy.radius).powi(2) {
                if player.hp > 0 && hit_cooldown <= 0.0 {
                    player.hp -= 10;
                    hit_cooldown = 0.5;
                    if player.hp <= 0 {
                        game_state = GameState::GameOver;
                    }
                }
            }
        }

        // Tozalash
        bullets_to_remove.sort_unstable();
        bullets_to_remove.dedup();
        for &idx in bullets_to_remove.iter().rev() {
            if idx < bullets.len() { bullets.remove(idx); }
        }

        enemies_to_remove.sort_unstable();
        enemies_to_remove.dedup();
        for &idx in enemies_to_remove.iter().rev() {
            if idx < enemies.len() { enemies.remove(idx); }
        }

        crystals_to_remove.sort_unstable();
        crystals_to_remove.dedup();
        for &idx in crystals_to_remove.iter().rev() {
            if idx < crystals.len() { crystals.remove(idx); }
        }

        // --- 4. RENDER ---
        let bg_color = if hit_cooldown > 0.0 { MAROON } else { DARKGRAY };
        clear_background(bg_color);

        for crystal in &crystals { crystal.draw(); }
        for enemy in &enemies { enemy.draw(); }
        for bullet in &bullets { bullet.draw(); }
        player.draw();

        draw_text(
            &format!("HP: {} | Level: {} | XP: {}/{} | Ball: {}", player.hp, player.level, player.xp, player.next_level_xp, score),
            20.0, 30.0, 18.0, WHITE,
        );
        draw_text("Vampire Survivor - Level Up Menyu Tayyor!", 20.0, 60.0, 16.0, LIGHTGRAY);

        next_frame().await;
    }
}
