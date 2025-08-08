use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::tile::{Tile, TileType};
use crate::snake::Snake;
use crate::goal::Goal;

pub const WIDTH: usize = 30;

pub struct Board {
    pub board_area: Rect,
    tiles: [Tile; WIDTH*WIDTH]
}

impl Board {
    pub fn new(posx: i32, posy: i32, width: u32, height: u32) -> Self {
        Self {
            board_area: Rect::new(posx, posy, width, height),
            tiles: [Tile::new(0, 0, TileType::Empty); WIDTH*WIDTH] 
        }
    }

    pub fn tile_at(&mut self, posx: usize, posy: usize) -> &mut Tile {
        return &mut self.tiles[posx + posy * WIDTH];
    }

    pub fn vacate_tile(&mut self, posx: usize, posy: usize) {
        self.tile_at(posx, posy).ttype = TileType::Empty;
    }

    /*fn generate_board() -> [Tile; 400] {  
        let temp_tiles: 
    }*/

    pub fn render(&mut self, goal: &Goal, snake: &Snake, canvas: &mut Canvas<Window>) {
        self.draw_edges(canvas);
        self.draw_goal(goal);
        self.draw_snake(snake);
        self.draw_tiles(snake, goal, canvas);
    }

    fn draw_edges(&self, canvas: &mut Canvas<Window>) {
        // Set color to be white
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        // Draw lines around the play area of the game
        let _ = canvas.draw_rect(self.board_area);
    }

    fn draw_goal(&mut self, goal: &Goal) {
        self.tile_at(goal.tile.x as usize, goal.tile.y as usize).ttype = TileType::Goal;
    }

    fn draw_snake(&mut self, snake: &Snake) {
        for tile in snake.body.clone() {
            // Copy the tile of each snake tile into its location on the board
            *self.tile_at(tile.x as usize, tile.y as usize) = tile.clone(); 
        }
    }

    fn draw_tiles(&self, snake: &Snake, goal: &Goal, canvas: &mut Canvas<Window>) {
        for y in 0..WIDTH { 
            for x in 0..WIDTH {
                let cur_tile_type = self.tiles[x + WIDTH * y].ttype.clone();
                match cur_tile_type {
                    TileType::Empty => {
                        continue;
                    }
                    TileType::Goal => {
                        canvas.set_draw_color(goal.color);
                        self.draw_tile(x, y, canvas);
                    }
                    TileType::SnakeHead => {
                        if snake.alive() {
                            canvas.set_draw_color(Color::RGB(110, 110, 110));
                        }
                        else {
                            canvas.set_draw_color(Color::RGB(200, 20, 20));
                        }
                        self.draw_tile(x, y, canvas);
                    } 
                    TileType::SnakeBody => {
                        if snake.alive() {
                            canvas.set_draw_color(Color::RGB(120, 90, 110));
                        }
                        else {
                            canvas.set_draw_color(Color::RGB(200, 20, 20));
                        }
                        self.draw_tile(x, y, canvas);
                    }
                }
            }
        }
    }

    fn draw_tile(&self, posx: usize, posy: usize, canvas: &mut Canvas<Window>) {
        // Calculate the window coordinates from the board position
        let x: i32 = self.board_area.x() + i32::try_from(posx).unwrap() * (self.board_area.width() as i32)/(WIDTH as i32);
        let y: i32 = self.board_area.y() + i32::try_from(posy).unwrap() * (self.board_area.height() as i32)/(WIDTH as i32);
        let pad: i32 = 2;
        let tile_rect: Rect = Rect::new(x + pad, y + pad, 
            self.board_area.width()/(WIDTH as u32) - (2*pad as u32), self.board_area.height()/(WIDTH as u32) - (2*pad as u32));
        // Assume the correct draw color is set in the canvas
        let _ = canvas.fill_rect(tile_rect);
    }
}
