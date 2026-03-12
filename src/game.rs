use core::time;

use crate::modules::bullet::Bullet;
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
    let enemy = StillImage::new(
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
    let btn_exit = TextButton::new(200.0, 750.0, 400.0, 130.0, "EXIT", RED, YELLOW, 90);
    let mut bullets: Vec<Bullet> = vec![];
    let mut bullet_remove=0;
    let mut timer = get_time();
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

        if ship.move_check_collision_x(&enemy) {}
for bullet in 0..bullets.len() {
    
    bullets[bullet].moving(-5.0);
    bullets[bullet].draw();
    if bullets[bullet].get_y() < 1.0 {
        println!("index: {}, bullet: {}", bullet, bullet_remove);
        
        bullets.remove(bullet);
        bullet_remove += 1;
    }
    
}
        enemy.draw();
        ship.draw();
        next_frame().await;
    }
}
