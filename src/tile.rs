use image::{DynamicImage, ImageBuffer, Rgba};
use image::imageops::{rotate90,rotate180,rotate270};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Edges{
    pub up: Vec<usize>,
    pub right: Vec<usize>,
    pub down: Vec<usize>,
    pub left: Vec<usize>,
}

impl Edges {
    pub fn new() -> Self{
        Edges{up: Vec::new(), right: Vec::new(), down: Vec::new(), left: Vec::new()}
    }
    pub fn get_from_index(&self, index: usize) -> Option<&Vec<usize>>{
        match index{
            0 => Some(&self.up),
            1 => Some(&self.left),
            2 => Some(&self.down),
            3 => Some(&self.right),
            _ => None
        }
    }
    pub fn get_mut_from_index(&mut self, index: usize) -> Option<&mut Vec<usize>>{
        match index{
            0 => Some(&mut self.up),
            1 => Some(&mut self.left),
            2 => Some(&mut self.down),
            3 => Some(&mut self.right),
            _ => None
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Connections{
    BLANK = 0,
    PATH = 1,
}

impl Connections{
    pub fn from_vec(vec: Vec<u8>) -> Vec<Self>{
        vec.iter().map(|&value| match value {
            0 => Connections::BLANK,
            1 => Connections::PATH,
            _ => panic!("Invalid value encountered"),
        })
        .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tile{
    pub tile_image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub connections: Vec<Connections>,
    pub edges: Edges
}

impl Tile{
    pub fn from(image: DynamicImage, connections: Vec<u8>) -> Self{
        Tile{tile_image: image.into_rgba8(), connections: Connections::from_vec(connections), edges: Edges::new()}
    }
    pub fn rotate90(&mut self){
        self.tile_image = rotate90(&self.tile_image);
        self.connections.rotate_left(1);
    }
    pub fn rotate180(&mut self){
        self.tile_image = rotate180(&self.tile_image);
        self.connections.rotate_left(2);
    }
    pub fn rotate270(&mut self){
        self.tile_image = rotate270(&self.tile_image);
        self.connections.rotate_left(3);
    }
    pub fn get_rotated90(&self) -> Self{
        let mut clone = self.clone();
        clone.rotate90();
        clone
    }
    pub fn get_rotated180(&self) -> Self{
        let mut clone = self.clone();
        clone.rotate180();
        clone
    }
    pub fn get_rotated270(&self) -> Self {
        let mut clone = self.clone();
        clone.rotate270();
        clone
    }
    pub fn get_image(&self) -> DynamicImage{
        DynamicImage::ImageRgba8(self.tile_image.clone())
    }
    pub fn analyze(&mut self, tiles: &Vec<Tile>){
        for tile_index in 0..tiles.len() {
            let tile = tiles.get(tile_index).unwrap();
            let connections_len = tile.connections.len();

            (0..connections_len).for_each(|connection_index| {
                if tile.connections[(connection_index+2)%connections_len] == self.connections[connection_index] {
                    self.edges.get_mut_from_index(connection_index).unwrap().push(tile_index);
                }
            });
        }
    }
}