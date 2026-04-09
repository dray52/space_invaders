use macroquad::prelude::*;

use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::preload_image::TextureManager;
    use crate::modules::preload_image::LoadingScreenOptions;
    use crate::modules::label::Label;

pub async fn run(tm: &TextureManager) -> String {
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
    let mut lbl_instructions = Label::new("Use arrow to move or A and D\n Use space to shoot", 40.0, 600.0, 60);
    let btn_start = TextButton::new(200.0, 750.0, 400.0, 130.0, "START", RED, YELLOW, 90);
    let btn_exit = TextButton::new(200.0, 900.0, 400.0, 130.0, "EXIT", RED, YELLOW, 90);

    lbl_instructions.with_colors(YELLOW, None);
    loop {
         use_virtual_resolution(800.0, 1200.0);
        clear_background(WHITE);
background.draw();
       
        if btn_start.click() {
            return "screen2".to_string();
        }
        if btn_exit.click() {
            return "exit".to_string();
        }
         
         title.draw();
         lbl_instructions.draw();
         next_frame().await;
    }
}
