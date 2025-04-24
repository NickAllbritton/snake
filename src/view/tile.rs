
#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Empty,
    SnakeBody,
    SnakeHead,
    Goal,
}

#[derive(Copy, Clone)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub ttype: TileType
}


impl Tile {
    pub fn new(posx: i32, posy: i32, tile_type: TileType) -> Self {
        Self {
            x: posx,
            y: posy,
            ttype: tile_type
        }
    }
}
