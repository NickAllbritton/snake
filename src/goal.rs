use rand::Rng;
use sdl2::pixels::Color;

use crate::board;
use crate::tile::{Tile, TileType};
use crate::snake::Snake;


pub struct Goal {
    pub tile: Tile,
    pub color: Color
}

impl Goal {

    pub fn new() -> Self {
        let mut rng = rand::rng();
        let x: usize;
        let y: usize;
        if rng.random_bool(0.5) {
            x = rng.random_range(0..(board::WIDTH/2));
        }
        else {
            x = rng.random_range((board::WIDTH/2 + 1)..board::WIDTH);
        }
        if rng.random_bool(0.5) {
            y = rng.random_range(0..(board::WIDTH/2));
        }
        else {
            y = rng.random_range((board::WIDTH/2 + 1)..board::WIDTH);
        }

        Self {
            tile: Tile::new(x as i32, y as i32, TileType::Goal),
            color: Color::RGB(255, 255, 255) // initialize with white
        }
    }

    pub fn respawn(&mut self, snake: &Snake) {
        let mut rng = rand::rng();
        let mut newx = rng.random_range(0..board::WIDTH);
        let mut newy = rng.random_range(0..board::WIDTH);
        let mut i: usize = 0;
        while i < snake.body.len() {
            // If a conflict occurs regenerate the position and try again
            if snake.body[i].x == newx as i32 && snake.body[i].y == newy as i32 {
                newx = rng.random_range(0..board::WIDTH);
                newy = rng.random_range(0..board::WIDTH);
                i = 0;
            }
            i += 1;
        }
        // If this point is reached then there is no conflict with the snake pos
        self.tile.x = newx as i32;
        self.tile.y = newy as i32;
    }

}
