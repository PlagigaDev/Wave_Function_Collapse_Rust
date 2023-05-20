pub mod tile;
pub mod images;

use image::DynamicImage;
use rand::Rng;
use tile::Tile;
use images::{combine_images, load_images, DIM};

const TILE_PATH: &str = "./tiles/demo-tracks";

const MAX_SIZE: usize = usize::MAX;


#[derive(Debug, PartialEq, Clone)]
struct GridCell{
    pub collapsed: bool,
    pub id: usize,
    pub options: Vec<usize>
}

impl GridCell{
    pub fn new(id: usize, max_options: usize) -> Self{
        GridCell{ 
            collapsed: false,
            id: id,
            options: (0..max_options).collect(),
        }
    }
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

    let mut lowest_entropy_cells: Vec<&mut GridCell> = grid.iter_mut().filter(|cell| !cell.collapsed && cell.options.len() == lowest_entropy_value).collect();

    let mut rand = rand::thread_rng();
    let random_cell = rand.gen_range(0..lowest_entropy_cells.len());

    let mut current_cell: &mut GridCell = &mut lowest_entropy_cells[random_cell];
    current_cell.collapsed = true;
    
    get_random_tile(&mut current_cell.options);
}

fn update_grid(grid: &mut Vec<GridCell>, tiles: &Vec<Tile>){
    for index in 0..(DIM*DIM) {
        if grid[index].collapsed { continue; }

        let y_index = index/DIM;
        let x_index = index%DIM;

        let mut options: Vec<usize> = (0..tiles.len()).collect();
        
        //Look up
        if y_index > 0 {
            let up = &grid[x_index + (y_index-1) * DIM];
            let mut valid: Vec<usize> = Vec::new();
            up.options.iter().for_each(|option| valid.append(&mut tiles[*option].edges.down.clone()));
            options.retain(|tile_value| valid.iter().find(|valid_tile| *tile_value == **valid_tile).is_some());
        }
        
        //Look right
        if x_index < DIM-1 {
            let right = &grid[index + 1];
            let mut valid: Vec<usize> = Vec::new();
            right.options.iter().for_each(|option| valid.append(&mut tiles[*option].edges.left.clone()));
            options.retain(|tile_value| valid.iter().find(|valid_tile| *tile_value == **valid_tile).is_some());
        }

        //Look down
        if y_index < DIM-1 {
            let down = &grid[x_index + (y_index+1) * DIM];
            let mut valid: Vec<usize> = Vec::new();
            down.options.iter().for_each(|option| valid.append(&mut tiles[*option].edges.up.clone()));
            options.retain(|tile_value| valid.iter().find(|valid_tile| *tile_value == **valid_tile).is_some());
        }
        //Look left
        if x_index > 0 {
            let lef = &grid[index - 1];
            let mut valid: Vec<usize> = Vec::new();
            lef.options.iter().for_each(|option| valid.append(&mut tiles[*option].edges.right.clone()));
            options.retain(|tile_value| valid.iter().find(|valid_tile| *tile_value == **valid_tile).is_some());
        }

        grid[index].options = options;

    }
    
}

fn get_image_que(grid: &mut Vec<GridCell>, tiles: &Vec<Tile>) -> Vec<DynamicImage>{

    for _ in 0..(DIM*DIM) {
        collapse_lowest_entropy(grid);
        update_grid( grid, tiles);
    }
    grid.iter().map(|cell| tiles.get(*cell.options.get(0).clone().unwrap()).unwrap().get_image()).collect()
}

fn main() {
    let mut grid: Vec<GridCell> = Vec::new();
    
    let mut tiles = load_images(TILE_PATH);
    
    for i in 0..(DIM*DIM){
        grid.insert(i, GridCell::new(i, tiles.len()));
    }
    
    let tiles_clone: Vec<Tile> = tiles.clone();
    tiles.iter_mut().for_each(|tile| tile.analyze(&tiles_clone));

    let vec: Vec<DynamicImage> = get_image_que(&mut grid, &tiles);

    let result_image = combine_images(&vec).unwrap();
    
    // Save the result image to a file
    result_image.save("result_image.png").expect("Failed to save image.");
}
