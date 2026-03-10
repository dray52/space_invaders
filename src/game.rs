use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::player::Player;
pub async fn run() -> String {

    let background = StillImage::new(
        "assets/background.png",
        800.0,  // width
        1200.0,  // height
        0.0,  // x position
        0.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
let enemy = StillImage::new(
        "assets/enemy.png",
        50.0,  // width
        50.0,  // height
        0.0,  // x position
        0.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
 let mut ship = Player::new(
        "assets/ship.png",
        50.0,  // width
        50.0,  // height
        200.0,  // x position
        500.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;



loop{
clear_background(WHITE);
background.draw();
ship.moveing();
if ship.move_check_collision_x(&enemy){

}









enemy.draw();
    ship.draw();
     next_frame().await;
}
    
}