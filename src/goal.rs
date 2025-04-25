use rand::Rng;

use crate::tile::{Tile, TileType};
use crate::snake::Snake;


pub struct Goal {
    pub tile: Tile
}

impl Goal {

    pub fn new() -> Self {
        let mut rng = rand::rng();
        let x: i32;
        let y: i32;
        if rng.random_bool(0.5) {
            x = rng.random_range(0..10);
        }
        else {
            x = rng.random_range(11..20);
        }
        if rng.random_bool(0.5) {
            y = rng.random_range(0..10);
        }
        else {
            y = rng.random_range(11..20);
        }

        Self {
            tile: Tile::new(x, y, TileType::Goal)
        }
    }

    pub fn respawn(&mut self, snake: &Snake) {
        let mut rng = rand::rng();
        let mut newx = rng.random_range(0..20);
        let mut newy = rng.random_range(0..20);
        let mut i: usize = 0;
        while i < snake.body.len() {
            // If a conflict occurs regenerate the position and try again
            if snake.body[i].x == newx && snake.body[i].y == newy {
                newx = rng.random_range(0..20);
                newy = rng.random_range(0..20);
                i = 0;
            }
            i += 1;
        }
        // If this point is reached then there is no conflict with the snake pos
        self.tile.x = newx;
        self.tile.y = newy;
    }

}
