use macroquad::prelude::*;

fn load_voxels() {
    let icon = Image::from_file_with_format(
    include_bytes!("../images/2.png"),
    Some(ImageFormat::Png),
    );
    let ico = icon.unwrap();
    let bytes = ico.bytes;
    let mut bools = vec![];
    for i in 0..bytes.len() / 4 {
        bools.push(bytes[i * 4] == 0);
    }
}

struct Section {
    // start included
    start: usize,
    len: usize,
    children: Vec<usize>
}

fn get_sections(w: usize, h: usize, bools: Vec<bool>) {
    let mut rows_of_sections = vec![];
    for i in 0..h {
        let mut row_of_sections = vec![];
        let now_row = &bools[(h - i - 1) * w .. (h - i) * w];
        let mut prev_el = false;
        let mut start = 0;
        let mut len;
        for (j, now_el) in now_row.iter().enumerate() {
            if !prev_el && *now_el {
                start = j;
            }
            if prev_el &&!*now_el {
                len = j - start;
                let sec = Section{start, len, children: vec![]};
                row_of_sections.push(sec);
            }
            if *now_el && j == w - 1 {
                len = w - start;    
                let sec = Section{start, len, children: vec![]};
                row_of_sections.push(sec);
            }
            prev_el = *now_el;
        }
        rows_of_sections.push(row_of_sections);
    }
}

fn twod_to_threed(width: usize, height: usize, bools: Vec<bool>) {

}

#[macroquad::main("MyGame")]
async fn main() {
    //let ve = load_voxels();
    //println!("{:?}", ve);
    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}