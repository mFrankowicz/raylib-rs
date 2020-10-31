extern crate raylib;
use raylib::prelude::*;

fn main() {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screen_width = 800;
    let screen_height = 450;

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("raylib-rs [textures] example - background scrolling")
        .build();

    // NOTE: Be careful, background width must be equal or bigger than screen width
    // if not, texture should be draw more than two times for scrolling effect
    let background = rl
        .load_texture(&thread, "resources/cyberpunk_street_background.png")
        .expect("could not load cyberpunk_street_background image");
    let midground = rl
        .load_texture(&thread, "resources/cyberpunk_street_midground.png")
        .expect("could not load cyberpunk_street_midground image");
    let foreground = rl
        .load_texture(&thread, "resources/cyberpunk_street_foreground.png")
        .expect("could not load cyberpunk_street_foreground image");

    let mut scrolling_back = 0.0;
    let mut scrooling_mid = 0.0;
    let mut scrooling_fore = 0.0;

    // Set our game to run at 60 frames-per-second
    rl.set_target_fps(60);
    //--------------------------------------------------------------------------------------

    // Main game loop
    // Detect window close button or ESC key
    while !rl.window_should_close() {
        // Update
        //----------------------------------------------------------------------------------
        scrolling_back -= 0.1;
        scrooling_mid -= 0.5;
        scrooling_fore -= 1.0;

        // NOTE: Texture is scaled twice its size, so it sould be considered on scrolling
        if scrolling_back <= (-&background.width() * 2) as f32 {
            scrolling_back = 0.0;
        }
        if scrooling_mid <= (-&midground.width() * 2) as f32 {
            scrooling_mid = 0.0;
        }
        if scrooling_fore <= (-&foreground.width() * 2) as f32 {
            scrooling_fore = 0.0;
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::get_color(0x052c46ff));

        // Draw background image twice
        // NOTE: Texture is scaled twice its size
        d.draw_texture_ex(
            &background,
            Vector2::new(scrolling_back, 20.0),
            0.0,
            2.0,
            Color::WHITE,
        );

        d.draw_texture_ex(
            &background,
            Vector2::new(&(background.width() as f32) * 2.0 + scrolling_back, 20.0),
            0.0,
            2.0,
            Color::WHITE,
        );

        // Draw midground image twice
        d.draw_texture_ex(
            &midground,
            Vector2::new(scrooling_mid, 20.0),
            0.0,
            2.0,
            Color::WHITE,
        );

        d.draw_texture_ex(
            &midground,
            Vector2::new(&(midground.width() as f32) * 2.0 + scrooling_mid, 20.0),
            0.0,
            2.0,
            Color::WHITE,
        );

        // Draw foreground image twice
        d.draw_texture_ex(
            &foreground,
            Vector2::new(scrooling_fore, 70.0),
            0.0,
            2.0,
            Color::WHITE,
        );

        d.draw_texture_ex(
            &foreground,
            Vector2::new(&(foreground.width() as f32) * 2.0 + scrooling_fore, 70.0),
            0.0,
            2.0,
            Color::WHITE,
        );

        d.draw_text("BACKGROUND SCROLLING & PARALLAX", 10, 10, 20, Color::RED);
        d.draw_text(
            "(c) Cyberpunk Street Environment by Luis Zuno (@ansimuz)",
            screen_width - 330,
            screen_height - 20,
            10,
            Color::RAYWHITE,
        );
    }
}
