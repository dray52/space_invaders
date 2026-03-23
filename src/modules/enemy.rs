//pub mod enemy;

//use crate::modules::enemy::Enemy;

use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::collision::check_collision;
use crate::modules::bullet::{self, Bullet};
use crate::start;
pub struct Enemy {
    view: StillImage,
    move_speed: f32,
    movement: Vec2,
    
    


}



impl Enemy{
 pub async fn new(
        asset_path: &str, 
        width: f32, 
        height: f32, 
        x: f32, 
        y: f32,
        stretch_enabled: bool,
        zoom_level: f32) -> Enemy{

        Enemy {
            view: StillImage::new(asset_path, width, height, x, y, stretch_enabled, zoom_level).await,
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

   
    

  pub fn enemy_move(&mut self, move_speed: f32, start_x: f32, start_pos: Vec2) {
        self.set_x(self.get_x()+move_speed * get_frame_time());
        if self.get_x() > start_pos.x+350.0 || self.get_x() < start_pos.x-50.0 {
            println!("Enemy changed direction");
            self.move_speed *= -1.0; // Reverse direction
            self.set_y(self.get_y() + 40.0);
        }

    }
    
    // Getter for visibility
    #[allow(unused)]
  
    pub fn view_player(&self) -> &StillImage {
        &self.view
    }
    // Setter for position
    #[allow(unused)]
    pub fn set_position(&mut self, x: f32, y: f32) -> &mut Self {
        self.view.set_x(x);
        self.view.set_y(y);
        self
    }
     pub fn get_x(&self) -> f32 {
        self.view.get_x()
    }

    #[allow(unused)]
    pub fn set_x(&mut self, x: f32) {
        self.view.set_x(x);
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
    
    
    
    
    
    
    }