use glam::IVec2;
use rand::Rng;

use crate::view::tile::{Tile, TileType};
use crate::view::board::Board;


pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {

    pub fn random() -> Self {
        let mut rng = rand::rng();
        match rng.random_range(0..4) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => unreachable!()
        }
    }

    pub fn vec(&self) -> IVec2 {
        match self {
            Direction::Up => IVec2::new(0, -1),
            Direction::Down => IVec2::new(0, 1),
            Direction::Left => IVec2::new(-1, 0),
            Direction::Right => IVec2::new(1, 0),
        }
    }

}

// Do I create an enum Direction and use a speed
// or do I do what I'm doing and use glam::Vec2.
// The second way sounds more difficult with little to no return on investment

pub struct Snake {
    pub dir: Direction,
    pub velocity: f32,
    pub body: Vec<Tile>
}

impl Snake {

    pub fn new(head_pos: IVec2) -> Self {
        let rand_direction = Direction::random();
        let pos_tail = head_pos - rand_direction.vec();
        let head_tile = Tile::new(head_pos.x, head_pos.y, TileType::SnakeHead);
        let tail_tile = Tile::new(pos_tail.x, pos_tail.y, TileType::SnakeBody);
        Self {
            dir: rand_direction,
            velocity: 0.25, // Temporary value
            body: vec![head_tile, tail_tile]
        }
    }

    pub fn within_board(&self) -> bool {
        return self.body[0].x >= 0 && self.body[0].y >= 0 
            && self.body[0].x < 20 && self.body[0].y < 20
    }

    pub fn move_once(&mut self, board: &mut Board) {
        // Move the head in the direction snake is moving
        // First store the position of the current head 
        let mut pos_vacated_tile = (self.body[0].x, self.body[0].y);
        self.body[0].x += self.dir.vec().x;
        self.body[0].y += self.dir.vec().y;
       
        for i in 1..self.body.len() {
            board.vacate_tile(self.body[i].x as usize, self.body[i].y as usize);
            let tmp_vacated_tile = (self.body[i].x, self.body[i].y);
            self.body[i].x = pos_vacated_tile.0;
            self.body[i].y = pos_vacated_tile.1;
            pos_vacated_tile = tmp_vacated_tile;
        }
    }
}
