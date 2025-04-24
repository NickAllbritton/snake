use rand::Rng;

use crate::view::tile::{Tile, TileType};
use crate::model::snake::Snake;


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
        let newx = rng.random_range(0..20);
        let newy = rng.random_range(0..20);
        for tile in snake.body.clone() {
            if tile.x == newx || tile.y == newy {
                self.respawn(snake);
                break;
            }
        }
        // If this point is reached then there is no conflict with the snake pos
        self.tile.x = newx;
        self.tile.y = newy;
    }

}
