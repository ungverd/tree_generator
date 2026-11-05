use macroquad::prelude::*;
use std::cmp;
use std::iter::zip;

fn load_voxels() -> Voxels {
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
    twod_to_threed(ico.width as usize, ico.height as usize, bools)
}

struct Section {
    // start included
    start: usize,
    len: usize,
    parents: Vec<usize>,
    surface: f64,
    width: usize,
}

fn add_section(start: usize,
               len: usize,
               row_num: usize,
               rows_of_sections: &Vec<Vec<Section>>) -> Section {
    let parents;
    if row_num > 0 {
        parents = get_parents(start, len, &rows_of_sections[row_num - 1]);
    } else {
        parents = vec![];
    }
    Section{start, len, parents, surface: 0., width: 0}
}

fn get_parents(start: usize, len: usize, prev_row: &Vec<Section>) -> Vec<usize> {
    let mut parents = vec![];
    for (i, sec) in prev_row.iter().enumerate() {
        let stop1 = start + len - 1;
        let start2 = sec.start;
        let stop2 = sec.start + sec.len - 1;
        if !(start2 > stop1 || start > stop2) {
            parents.push(i);
        }
    }
    parents
}

fn verify_not_parent(rows_of_sections: &Vec<Vec<Section>>, row_num: usize, sec_num: usize) -> bool {
    if row_num == rows_of_sections.len() - 1 {
        return true;
    }
    else {
        let next_row = &rows_of_sections[row_num + 1];
        for sec in next_row {
            for num in &sec.parents {
                if *num == sec_num {
                    return false;
                }
            }
        }
    }
    true
}

fn get_sections(w: usize, h: usize, bools: Vec<bool>) -> Vec<Vec<Section>> {
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
                let sec = add_section(start, len, i, &rows_of_sections);
                row_of_sections.push(sec);
            }
            if *now_el && j == w - 1 {
                len = w - start;
                let sec = add_section(start, len, i, &rows_of_sections);
                row_of_sections.push(sec);
            }
            prev_el = *now_el;
        }
        rows_of_sections.push(row_of_sections);
    }
    rows_of_sections
}

fn get_intersection(sec1: &Section, sec2: &Section) -> usize {
    let start_intersec = cmp::max(sec1.start, sec2.start);
    let end_intersec = cmp::min(sec1.start + sec1.len, sec2.start + sec2.len);
    end_intersec - start_intersec
}

fn populate_widths(rows_of_sections: &mut Vec<Vec<Section>>) {
    for row_num in (0..rows_of_sections.len()).rev() {
        let row = &rows_of_sections[row_num];
        for sec_num in 0..row.len() {
            if verify_not_parent(rows_of_sections, row_num, sec_num) {
                let sec = &mut rows_of_sections[row_num][sec_num];
                sec.surface = sec.len as f64;
            }
            if row_num > 0 {
                let prev_row = &rows_of_sections[row_num - 1];
                let sec = &rows_of_sections[row_num][sec_num];
                let mut intersections = vec![];
                for parent_num in &sec.parents {
                    let sec2 = &prev_row[*parent_num];
                    intersections.push(get_intersection(sec, sec2));
                }
                let intersections_sum: usize = intersections.iter().sum();
                let now_surface = sec.surface;
                let parents = sec.parents.clone();
                for (parent_num, intersec) in zip(parents, intersections) {
                    let surface_to_add = now_surface * (intersec as f64) / (intersections_sum as f64);
                    let parent = &mut rows_of_sections[row_num - 1][parent_num];
                    parent.surface += surface_to_add;
                }
            }
        }
    }
    for row in rows_of_sections {
        for sec in row {
            sec.width = cmp::max((sec.surface / (sec.len as f64)).round() as usize, 1);
        }
    }
}

fn row_to_slice(row: &Vec<Section>, width: usize, max_depth: usize) -> Vec<bool> {
    let mut my_slice = vec![false; width * max_depth];
    for sec in row {
        for depth in 0..sec.width {
            for x in sec.start..sec.start + sec.len {
                my_slice[width * depth + x] = true;
            }
        }
    }
    my_slice
}

struct Voxels {
    width: usize,
    height: usize,
    depth: usize,
    cells: Vec<bool>
}

struct Point {
    x: f32,
    y: f32,
    z: f32
}

impl Voxels {
    fn get_cell(&self, x: usize, y: usize, z: usize) -> bool {
        self.cells[z * self.width * self.depth + y * self.width + x]
    }

    fn is_border(&self, x: usize, y: usize, z: usize) -> bool {
        if x == 0 ||
           y == 0 ||
           z == 0 ||
           x == self.width - 1 ||
           y == self.depth - 1 ||
           z == self.height - 1 {
            return true;
        }
        if !self.get_cell(x + 1, y, z) ||
           !self.get_cell(x - 1, y, z) ||
           !self.get_cell(x, y + 1, z) ||
           !self.get_cell(x, y - 1, z) ||
           !self.get_cell(x, y, z + 1) ||
           !self.get_cell(x, y, z - 1) {
            return true;
           }
        false
    }

    fn get_border_coords(&self) -> Vec<Point> { // for k-d tree
        let mut points = vec![];
        for x in 0..self.width {
            for y in 0..self.depth {
                for z in 0..self.height {
                    if self.get_cell(x, y, z) { // cell is black
                        if self.is_border(x, y, z) { // border is white or out of range
                            let p = Point{x: x as f32 + 0.5,
                                                 y: y as f32 + 0.5,
                                                 z: z as f32 + 0.5};
                            points.push(p);
                        }
                    }
                }
            }
        }
        points
    }
}

fn twod_to_threed(width: usize, height: usize, bools: Vec<bool>) -> Voxels {
    let mut rows_of_sections = get_sections(width, height, bools);
    populate_widths(&mut rows_of_sections);
    let mut max_width = 0;
    for row in &rows_of_sections {
        for sec in row {
            if sec.width > max_width {
                max_width = sec.width;
            }
        }
    }
    let mut threed = vec![];
    for row in &rows_of_sections {
        let mut sli = row_to_slice(row, width, max_width);
        threed.append(&mut sli);
    }
    Voxels{width, height, depth: max_width, cells: threed}
}

#[macroquad::main("MyGame")]
async fn main() {
    let ve = load_voxels();
    println!("width {:?}", ve.width);
    println!("height {:?}", ve.height);
    println!("depth {:?}", ve.depth);
    println!("{:?}", ve.cells);
    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}