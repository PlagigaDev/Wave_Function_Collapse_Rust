use image::{DynamicImage, ImageBuffer, Rgba};
use image::imageops::{rotate90,rotate180,rotate270};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Edges{
    UP = 0,
    RIGHT = 1,
    DOWN = 2,
    LEFT = 3,
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
    pub edges: HashMap<Edges, Vec<usize>>
}

impl Tile{
    pub fn from(image: DynamicImage, connections: Vec<u8>) -> Self{
        Tile{tile_image: image.into_rgba8(), connections: Connections::from_vec(connections), edges: HashMap::new()}
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
        let mut connections = self.connections.clone();
        connections.rotate_left(1);
        Tile{
            tile_image: rotate90(&self.tile_image),
            connections: connections,
            edges: HashMap::new()
        }
    }
    pub fn get_rotated180(&self) -> Self{
        let mut connections = self.connections.clone();
        connections.rotate_left(2);
        Tile{
            tile_image: rotate180(&self.tile_image),
            connections: connections,
            edges: HashMap::new()
        }
    }
    pub fn get_rotated270(&self) -> Self {
        let mut connections = self.connections.clone();
        connections.rotate_left(3);
        Tile{
            tile_image: rotate270(&self.tile_image),
            connections: connections,
            edges: HashMap::new()
        }
    }
    pub fn get_image(&self) -> DynamicImage{
        DynamicImage::ImageRgba8(self.tile_image.clone())
    }
    pub fn analyze(&mut self, tiles: &Vec<Tile>){
        for index in 0..tiles.len() {
            let tile = tiles.get(index).unwrap();
            if tile.connections[2] == self.connections[0] {
                self.edges.get_mut(&Edges::UP)
                .unwrap()
                .push(index);
            }
            if tile.connections[3] == self.connections[1] {
                self.edges.get_mut(&Edges::UP)
                .unwrap()
                .push(index);
            }
            if tile.connections[0] == self.connections[2] {
                self.edges.get_mut(&Edges::UP)
                .unwrap()
                .push(index);
            }
            if tile.connections[1] == self.connections[3] {
                self.edges.get_mut(&Edges::UP)
                .unwrap()
                .push(index);
            }
        }
    }
}