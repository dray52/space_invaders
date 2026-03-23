use crate::modules::bullet::Bullet;
use crate::modules::enemy::Enemy;
use crate::modules::player::Player;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use macroquad::prelude::*;

pub async fn run() -> String {
    let background = StillImage::new(
        "assets/background.png",
        800.0,  // width
        1200.0, // height
        0.0,    // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    )
    .await;
    let enemy = Enemy::new(
        "assets/enemy.png",
        50.0, // width
        50.0, // height
        0.0,  // x position
        0.0,  // y position
        true, // Enable stretching
        1.0,  // Normal zoom (100%)
    )
    .await;
    let mut ship = Player::new(
        "assets/ship.png",
        50.0,  // width
        50.0,  // height
        200.0, // x position
        500.0, // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;

     let wall1 = StillImage::new(
        "assets/wall.png",
        500.0,  // width
        2000.0, // height
        -230.0,    // x position
        -300.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    )
    .await;
let wall2 = StillImage::new(
        "assets/wall.png",
        500.0,  // width
        2000.0, // height
        530.0,  // x position
        -300.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    )
    .await;
    let btn_exit = TextButton::new(200.0, 750.0, 400.0, 130.0, "EXIT", RED, YELLOW, 90);
    let mut bullets: Vec<Bullet> = vec![];
    let mut timer = get_time();
    let mut enemy_x = 200.0;
    let mut enemy_y = 50.0;
    let mut enemies: Vec<Enemy> = vec![];
    for i in 0..25 {
        let enemy = Enemy::new(
            "assets/enemy.png",
            40.0,    // width
            40.0,    // height
            enemy_x, // x position
            enemy_y, // y position
            true,    // Enable stretching
            1.0,     // Normal zoom (100%)
        )
        .await;
        enemy_x += 90.0; // Move the next enemy to the right
        if enemy_x > 600.0 {
            // If we reach the edge of the screen
            enemy_x = 200.0; // Reset x position
            enemy_y += 60.0; // Move the next row down
        }
        enemies.push(enemy);
    }

    loop {
        clear_background(WHITE);
        background.draw();

        if btn_exit.click() {
            return "screen1".to_string();
        }
        if is_key_down(KeyCode::Space) && get_time() - timer > 0.5 {
            bullets = ship.spawn_bullet(bullets).await;
            timer = get_time();
        }
        ship.moveing();

        let mut delete_bullets: Vec<usize> = Vec::new();
        for bullet in 0..bullets.len() {
            bullets[bullet].moving(-5.0);
            bullets[bullet].draw();
            if bullets[bullet].get_y() < 1.0 {

                delete_bullets.push(bullet);
            }
        }
        for i in 0..delete_bullets.len() {
            bullets.remove(delete_bullets[i]);
        }

        ship.draw();
        for enemy in 0..enemies.len() {
            enemies[enemy].draw();
        }
        wall1.draw();
        wall2.draw();
        next_frame().await;
    }
}
