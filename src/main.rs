use std::collections::HashMap;
use std::fs;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};

const TILE_PATH: &str = "./tiles/demo-tracks";

const DIM: usize = 2;



enum Tiles{
    BLANK = 0,
    UP = 1,
    RIGHT = 2,
    DOWN = 3,
    LEFT = 4,
}

impl Tiles{
    pub fn from_str(str: &str) -> Option<Self> {
        match str.to_uppercase().as_str() {
            "BLANK" => Some(Tiles::BLANK),
            "UP" => Some(Tiles::UP),
            "Right" => Some(Tiles::RIGHT),
            "DOWN" => Some(Tiles::DOWN),
            "LEFT" => Some(Tiles::LEFT),
            _ => None
        }
    }
}
 


struct GridCell{
    pub collapsed: bool,
    pub options: Vec<Tiles>
}

impl GridCell{
    pub fn new() -> Self{
        GridCell{ 
            collapsed: false,
            options: Vec::from([Tiles::BLANK,Tiles::UP,Tiles::RIGHT,Tiles::DOWN,Tiles::LEFT])
        }
    }
}

fn load_images(directory_path: &str) -> HashMap<String, image::DynamicImage> {
    let mut images_map: HashMap<String, image::DynamicImage> = HashMap::new();

    // Read the directory contents
    if let Ok(entries) = fs::read_dir(directory_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Some(file_name) = entry.file_name().to_str() {
                            // Load the image file
                            if let Ok(image) = image::open(entry.path()) {
                                images_map.insert(file_name.replace(".png", "").to_owned(), image);
                            }
                        }
                    }
                }
            }
        }
    }

    images_map
}

fn combine_images(images: &Vec<DynamicImage>) -> Option<DynamicImage> {
    let mut widths: Vec<usize> = Vec::new();
    let mut heights: Vec<usize> = Vec::new();

    // Calculate the total width and height of the combined image
    for img in images {
        let (width, height) = img.dimensions();
        widths.push(width as usize);
        heights.push(height as usize);
    }

    let max_width = *widths.iter().max().unwrap_or(&0);
    let max_height = *heights.iter().max().unwrap_or(&0);
    let column_count = DIM;
    let row_count = (images.len() as f64 / column_count as f64).ceil() as usize;
    let combined_width = max_width * column_count;
    let combined_height = max_height * row_count;
    let mut combined_image = ImageBuffer::new(combined_width as u32, combined_height as u32);

    for (i, img) in images.iter().enumerate() {
        let (width, height) = img.dimensions();
        let col = i % column_count;
        let row = i / column_count;
        let x_offset = col * max_width;
        let y_offset = row * max_height;

        for (x, y, pixel) in img.pixels() {
            combined_image.put_pixel(x + x_offset as u32, y + y_offset as u32, pixel);
        }
    }

    Some(DynamicImage::ImageRgba8(combined_image))
}

fn get_image_que(grid: &Vec<GridCell>) -> Vec<DynamicImage>{
    let mut image_que: Vec<DynamicImage> = Vec::new();
    for x in 0..DIM{
        for y in 0..DIM{
            let cell = match grid.get(y + x*DIM){
                Some(grid_cell) => grid_cell,
                None => continue,
            };
            if cell.collapsed{
                let index = cell.options.get(0).unwrap();
                //image_que.
            }
        }
    }
    image_que
}

fn main() {
    let mut grid: Vec<GridCell> = Vec::new();

    for i in 0..(DIM*DIM){
        grid.insert(i, GridCell::new());
    }

    let images = load_images(TILE_PATH);

    let vec: Vec<DynamicImage> = images.values().cloned().collect();

    let result_image = combine_images(&vec).unwrap();

    // Save the result image to a file
    result_image.save("result_image.png").expect("Failed to save image.");
}
