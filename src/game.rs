use crate::modules::bullet::{self, Bullet};
use crate::modules::collision::check_collision;
use crate::modules::enemy::Enemy;
use crate::modules::player::Player;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use crate::modules::label::Label;
use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;
use miniquad::date;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::preload_image::TextureManager;
    use crate::modules::preload_image::LoadingScreenOptions;

pub async fn run(tm: &TextureManager) -> String {
    let mut background = StillImage::new("", 800.0, 1200.0, 0.0, 0.0, true, 1.0).await;
    background.set_preload(tm.get_preload("assets/background.png").unwrap());

    let mut ship = Player::new("", 50.0, 50.0, 200.0, 1100.0).await;
    ship.set_preload(tm.get_preload("assets/ship.png").unwrap());

    let wall1 = StillImage::new("assets/wall.png", 500.0, 2000.0, -230.0, -300.0, true, 1.0).await;
    let wall2 = StillImage::new("assets/wall.png", 500.0, 2000.0, 530.0, -300.0, true, 1.0).await;
    let btn_exit = TextButton::new(760.0, 10.0, 50.0, 32.5, "X", RED, YELLOW, 20);
let heart1 = StillImage::new("assets/heart.png", 50.0, 50.0, 10.0, 10.0, true, 1.0).await;
let heart2 = StillImage::new("assets/heart.png", 50.0, 50.0, 70.0, 10.0, true, 1.0).await;
let heart3 = StillImage::new("assets/heart.png", 50.0, 50.0, 130.0, 10.0, true, 1.0).await;
let mut lbl_score = Label::new("0", 350.0, 20.0, 30);
lbl_score.with_colors(WHITE, None);

let mut score = 0;
let mut lives = 3;  
    let mut e_bullets: Vec<Bullet> = vec![];
    let mut p_bullets: Vec<Bullet> = vec![];
    let mut cooldown = get_time();
    let mut bullet_cooldown = get_time();
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

    rand::srand(date::now() as u64);
    loop {
         use_virtual_resolution(800.0, 1200.0);
        clear_background(WHITE);
        background.draw();
        for enemy in 0..enemies.len() {
            enemies[enemy].enemy_move(&wall1, &wall2);
        }
        if get_time() - bullet_cooldown > 1.0 {
            let spawn = enemies.choose().unwrap();
            let bullet_x = spawn.get_x();
            let bullet_y = spawn.get_y();
            let bullet = Bullet::new(
                "assets/bullet1.png",
                50.0, // width
                50.0, // height
                bullet_x,
                bullet_y,
                true, // Enable stretching
                1.0,  // Normal zoom (100%)
            );
            e_bullets.push(bullet.await);
            bullet_cooldown = get_time();
        }

        if is_key_down(KeyCode::Space) && get_time() - cooldown > 0.5 {
            p_bullets = ship.spawn_bullet(p_bullets).await;
            cooldown = get_time();
        }
        

        
        for bullet in 0..p_bullets.len() {
            p_bullets[bullet].moving(-2.0);
            p_bullets[bullet].draw();
            if p_bullets[bullet].get_y() < 30.0 {
                
                 p_bullets.remove(bullet);
                 break;
            }
        }


        
        for enemy in 0..enemies.len() {
            enemies[enemy].draw();
        }
        for bullet in 0..e_bullets.len() {
            e_bullets[bullet].moving(2.0);
            e_bullets[bullet].draw();
            if e_bullets[bullet].get_y() > 1200.0 {
               e_bullets.remove(bullet);
                 break;
            }
        }



let mut matt = 0;
for bullet in 0..p_bullets.len() {
            for enemy in 0..enemies.len() {
                if check_collision(p_bullets[bullet].view_player(), enemies[enemy].view_player(), 1) {
                    p_bullets.remove(bullet);
                    enemies.remove(enemy);
                    score += 100;
                    lbl_score.set_text(format!("{}", score));
                    matt = 1;
                    break;
                }
            }
            if matt == 1 {
                break;
            }
        }

for bullet in 0..e_bullets.len() {
                if check_collision(e_bullets[bullet].view_player(), ship.view_player(), 1) {
                    lives -= 1;
                    e_bullets.remove(bullet);
                    break;
                }
            }
            if lives == 0 {
                return "screen1".to_string();
            }




ship.moveing(&wall1, &wall2);
        ship.draw();
        wall1.draw();
        wall2.draw();
        lbl_score.draw();
        if lives == 3 {
            heart1.draw();
            heart2.draw();
            heart3.draw();
        }
        if lives == 2 {
            heart1.draw();
            heart2.draw();
        }
        if lives == 1 {
            heart1.draw();
        }
        if btn_exit.click() {
            return "screen1".to_string();
        }
        next_frame().await;
    }
}
