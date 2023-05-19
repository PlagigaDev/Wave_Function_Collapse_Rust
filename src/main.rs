pub mod tile;

use std::collections::HashMap;
use std::fs;
use image::{DynamicImage, GenericImageView, ImageBuffer};
use image::imageops::FilterType;
use rand::Rng;
use tile::Tile;


const TILE_PATH: &str = "./tiles/demo-tracks";

const DIM: usize = 10;
const IMAGE_PIXEL_SIZE: usize = 2000;
const MAX_SIZE: usize = usize::MAX;


#[derive(Debug, Clone)]
struct GridCell{
    pub collapsed: bool,
    pub options: Vec<usize>
}

impl GridCell{
    pub fn new(max_options: usize) -> Self{
        GridCell{ 
            collapsed: false,
            options: (0..max_options).collect(),
        }
    }
}

fn load_images(directory_path: &str) -> Vec<Tile> {
    let mut images_map: Vec<Tile> = Vec::new();

    // Read the directory contents
    /*if let Ok(entries) = fs::read_dir(directory_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Some(file_name) = entry.file_name().to_str() {
                            // Load the image file
                            if let Ok(image) = image::open(entry.path()) {
                                let image_size = IMAGE_PIXEL_SIZE/DIM;
                                let image = image.resize(image_size as u32,image_size as u32, FilterType::Nearest);
                                images_map.insert(TileType::from_str(file_name.replace(".png", "").trim()).unwrap(), Tile::from(image));
                            }
                        }
                    }
                }
            }
        }
    }*/
    let image_size = IMAGE_PIXEL_SIZE/DIM;

    let blank = Tile::from(image::open(directory_path.to_owned()+"/blank.png").expect("Failed to load image").resize(image_size as u32,image_size as u32, FilterType::Nearest), vec![0,0,0,0]);
    let up = Tile::from(image::open(directory_path.to_owned()+"/up.png").expect("Failed to load image").resize(image_size as u32,image_size as u32, FilterType::Nearest), vec![1,1,0,1]);
    let left = up.get_rotated90();
    let down = up.get_rotated180();
    let right = up.get_rotated270();
    
    images_map.push(blank);
    images_map.push(up);
    images_map.push(left);
    images_map.push(down);
    images_map.push(right);
    
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

fn get_random_tile(options: &mut Vec<usize>){
    let mut rand = rand::thread_rng();
    let random_option = rand.gen_range(0..options.len());
    let mut new_vec: Vec<usize> = Vec::new();
    new_vec.insert(0, options[random_option].clone());
    *options = new_vec;
}

fn collapse_lowest_entropy(grid: &mut Vec<GridCell>){
    let lowest_entropy_value = grid.iter().map(|cell| {
        if cell.collapsed{
            return MAX_SIZE;
        }
        return cell.options.len();
    })
    .min()
    .unwrap();
    
    let mut lowest_entropy_cells: Vec<&mut GridCell> = grid.iter_mut().filter(|cell| cell.options.len() == lowest_entropy_value).collect();
    
    let mut rand = rand::thread_rng();
    let random_cell = rand.gen_range(0..lowest_entropy_cells.len());

    let current_cell: &mut GridCell = lowest_entropy_cells[random_cell];
    current_cell.collapsed = true;
    get_random_tile(&mut current_cell.options);
}

fn get_image_que(grid: &mut Vec<GridCell>, tiles: &Vec<Tile>) -> Vec<DynamicImage>{

    (0..(DIM*DIM)).for_each(|_| collapse_lowest_entropy(grid));

    grid.iter().map(|cell| tiles.get(*cell.options.get(0).unwrap()).unwrap().get_image()).collect()
}

fn main() {
    let mut grid: Vec<GridCell> = Vec::new();
    
    let mut tiles = load_images(TILE_PATH);

    for i in 0..(DIM*DIM){
        grid.insert(i, GridCell::new(tiles.len()));
    }
    
    let tiles_clone = tiles.clone();
    tiles.iter_mut().for_each(|tile| tile.analyze(&tiles_clone));
   
    
    let vec: Vec<DynamicImage> = get_image_que(&mut grid, &tiles);

    let result_image = combine_images(&vec).unwrap();

    // Save the result image to a file
    result_image.save("result_image.png").expect("Failed to save image.");
}
