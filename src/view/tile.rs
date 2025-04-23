
#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Empty,
    SnakeBody,
    SnakeHead,
    Goal,
}

#[derive(Copy, Clone)]
pub struct Tile {
    pub x: u8,
    pub y: u8,
    pub ttype: TileType
}


impl Tile {
    pub fn new(posx: u8, posy: u8, tile_type: TileType) -> Self {
        Self {
            x: posx,
            y: posy,
            ttype: tile_type
        }
    }
}
