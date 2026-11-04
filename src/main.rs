use macroquad::prelude::*;

fn load_voxels() -> Vec<u8> {
    let icon = Image::from_file_with_format(
    include_bytes!("../images/2.png"),
    Some(ImageFormat::Png),
    );
    
    icon.unwrap().bytes
}

#[macroquad::main("MyGame")]
async fn main() {
    let ve = load_voxels();
    println!("{:?}", ve);
    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}