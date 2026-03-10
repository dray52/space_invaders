use macroquad::prelude::*;

use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
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
    let title = StillImage::new(
        "assets/title.png",
        550.0, // width
        400.0, // height
        130.0, // x position
        100.0, // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;
    let btn_start = TextButton::new(200.0, 600.0, 400.0, 130.0, "START", RED, YELLOW, 90);
    let btn_exit = TextButton::new(200.0, 750.0, 400.0, 130.0, "EXIT", RED, YELLOW, 90);
    loop {
        clear_background(WHITE);
background.draw();
       
        if btn_start.click() {
            return "screen2".to_string();
        }
        if btn_exit.click() {
            return "exit".to_string();
        }
         
         title.draw();
         next_frame().await;
    }
}
