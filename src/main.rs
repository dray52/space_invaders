/*
By: Draydon Levesque
Date: 2026-03-09
Program Details: Space invaders game
*/
mod modules;

mod start;
mod game;

use macroquad::prelude::*;
use crate::modules::preload_image::TextureManager;
    use crate::modules::preload_image::LoadingScreenOptions;
fn window_conf() -> Conf {
    Conf {
        window_title: "Space Invaders".to_owned(),
        window_width: 800,
        window_height: 1200,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut current_screen = "screen1".to_string();
    let mut last_switch = get_time() - 0.02;
    let all_assets = vec![
        "assets/background.png",
        "assets/title.png",
        "assets/ship.png",
        "assets/wall.png",
        "assets/heart.png",
        "assets/enemy.png",
        "assets/bullet.png",
        "assets/bullet1.png",
    ];
let tm = TextureManager::new(); 
let loading_options = LoadingScreenOptions {
       title: Some("MY GAME".to_string()),
       background_color: DARKBLUE,
       bar_fill_color: GOLD,
       // Use default values for other options
       ..Default::default()
            };
            tm.preload_with_loading_screen(&all_assets, Some(loading_options)).await;
    loop {
       
        
        if get_time() - last_switch > 0.01 {
            current_screen = match current_screen.as_str() {
                "screen1" => start::run(&tm).await,
                "screen2" => game::run(&tm).await,
                _ => break,
            };
            last_switch = get_time();
        }
        next_frame().await;
    }
}
