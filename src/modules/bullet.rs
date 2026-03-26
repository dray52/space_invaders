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
 pub async fn new(image_path: (Texture2D, Option<Vec<u8>>, String), width: f32, height: f32, x: f32, y: f32,stretch_enabled: bool,zoom_level: f32) -> Bullet{
 let mut view = StillImage::new("", width, height, x, y, stretch_enabled, zoom_level).await;
view.set_preload(image_path);
        Bullet {
            view,
            move_speed: 400.0, // Default speed
            movement: Vec2::ZERO,
            
            
        }
        }

pub fn draw(&self) {
        // Only draw if the label is visible
       self.view.draw();
        }
    
#[allow(unused)]
 //change speed
    pub fn set_speed(&mut self, move_speed: f32) -> &mut Self {
        self.move_speed = move_speed;
        self
    }
#[allow(unused)]
 pub fn view_player(&self) -> &StillImage {
        &self.view
    }
#[allow(unused)]
     pub fn set_position(&mut self, x: f32, y: f32) -> &mut Self {
        self.view.set_x(x);
        self.view.set_y(y);
        self
    }

    // Get and set y position
    #[allow(unused)]
    pub fn get_y(&self) -> f32 {
        self.view.get_y()
    }

    #[allow(unused)]
    pub fn set_y(&mut self, y: f32) {
        self.view.set_y(y);
    }
    #[allow(unused)]
     pub fn pos(&self) -> Vec2 {
        vec2(self.view.get_x(), self.view.get_y())
    }

    #[allow(unused)]
    pub fn moving(&mut self, move_speed: f32)   {
        // Move the bullet upwards
        self.movement = self.pos() * move_speed * get_frame_time();
        // Apply movement based on frame time
        self.view.set_y(self.view.get_y() + self.movement.y);

    }
}