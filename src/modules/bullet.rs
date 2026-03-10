//pub mod bullet;

//use crate::modules::bullet::Bullet;

use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::collision::check_collision;
pub struct Bullet {
    view: StillImage,
    move_speed: f32,
    movement: Vec2,
    


}



impl Bullet{
 pub async fn new(
        asset_path: &str, 
        width: f32, 
        height: f32, 
        x: f32, 
        y: f32,
        stretch_enabled: bool,
        zoom_level: f32) -> Bullet{

        Bullet {
            view: StillImage::new(asset_path, width, height, x, y, stretch_enabled, zoom_level).await,
            move_speed: 400.0, // Default speed
            movement: Vec2::ZERO,
            
            
        }
        }
    }