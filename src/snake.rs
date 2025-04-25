use glam::IVec2;
use rand::Rng;

use crate::tile::{Tile, TileType};
use crate::board::{self, Board};
use crate::goal::Goal;

#[derive(PartialEq)]
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

pub struct Snake {
    pub dir: Direction,
    pub body: Vec<Tile>,
    alive: bool,
    grow: bool
}

impl Snake {

    pub fn new(head_pos: IVec2) -> Self {
        let rand_direction = Direction::random();
        let pos_tail = head_pos - rand_direction.vec();
        let head_tile = Tile::new(head_pos.x, head_pos.y, TileType::SnakeHead);
        let tail_tile = Tile::new(pos_tail.x, pos_tail.y, TileType::SnakeBody);
        Self {
            dir: rand_direction,
            body: vec![head_tile, tail_tile],
            alive: true,
            grow: false
        }
    }

    pub fn alive(&self) -> bool {
        return self.alive;
    }

    pub fn die(&mut self) {
        self.alive = false;
    }

    pub fn within_board(&self) -> bool {
        let nextx_head = self.body[0].x + self.dir.vec().x;
        let nexty_head = self.body[0].y + self.dir.vec().y;

        return nextx_head >= 0 && nextx_head < board::WIDTH as i32
            && nexty_head >= 0 && nexty_head < board::WIDTH as i32;
    }

    pub fn eating_tail(&self) -> bool {
        let new_head_pos = (self.body[0].x + self.dir.vec().x, 
            self.body[0].y + self.dir.vec().y);
        for tile in self.body.clone() {
            if new_head_pos.0 == tile.x && new_head_pos.1 == tile.y {
                return true; // next update will eat tail
            }
        }
        return false; // next update will not eat tail
    }

    pub fn try_eat_goal(&mut self, goal: &mut Goal)
    {
        let nextx_head = self.body[0].x + self.dir.vec().x;
        let nexty_head = self.body[0].y + self.dir.vec().y;

        if goal.tile.x == nextx_head && goal.tile.y == nexty_head {
            self.grow = true;
            goal.respawn(&self);
        }
    }

    pub fn move_once(&mut self, board: &mut Board) {
        // Move the head in the direction snake is moving
        // First store the position of the current head 
        let mut pos_vacated_tile = (self.body[0].x, self.body[0].y);
        self.body[0].x += self.dir.vec().x;
        self.body[0].y += self.dir.vec().y;
       
        for i in 1..self.body.len() {
            if self.grow && i == self.body.len()-1 {
                // If grow is set to true, do not vacate the last tile
                self.body.push(self.body[i]);
                self.grow = false;
            }
            else {
                board.vacate_tile(self.body[i].x as usize, self.body[i].y as usize);
            }
            let tmp_vacated_tile = (self.body[i].x, self.body[i].y);
            self.body[i].x = pos_vacated_tile.0;
            self.body[i].y = pos_vacated_tile.1;
            pos_vacated_tile = tmp_vacated_tile;
        }
    }
}
