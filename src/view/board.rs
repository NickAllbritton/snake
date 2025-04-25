use sdl2::rect::{Rect,Point};
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::view::tile::{Tile, TileType};
use crate::model::snake::Snake;
use crate::model::goal::Goal;

pub struct Board {
    pub board_area: Rect,
    pub goal_color: Color,
    // Array of tiles 17*17
    tiles: [Tile; 400]
}

impl Board {
    pub fn new(posx: i32, posy: i32, width: u32, height: u32) -> Self {
        Self {
            board_area: Rect::new(posx, posy, width, height),
            goal_color: Color::RGB(150, 140, 200),
            tiles: [Tile::new(0, 0, TileType::Empty); 400]
        }
    }

    pub fn tile_at(&mut self, posx: usize, posy: usize) -> &mut Tile {
        return &mut self.tiles[posx + posy * 20];
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
        self.draw_tiles(snake, canvas);
    }

    fn draw_edges(&self, canvas: &mut Canvas<Window>) {
        // Set color to be white
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        // Draw lines around the play area of the game
        // Left side borderline
        let _ = canvas.draw_line(
            Point::new(self.board_area.x, self.board_area.y),
            Point::new(self.board_area.x, self.board_area.y + self.board_area.h)
        );
        // Right side borderline
        let _ = canvas.draw_line(
            Point::new(self.board_area.x + self.board_area.w, self.board_area.y),
            Point::new(self.board_area.x + self.board_area.w, self.board_area.y + self.board_area.h)
        );
        // Top side borderline
        let _ = canvas.draw_line(
            Point::new(self.board_area.x, self.board_area.y),
            Point::new(self.board_area.x + self.board_area.w, self.board_area.y)
        );
        // Bottom side borderline
        let _ = canvas.draw_line(
            Point::new(self.board_area.x, self.board_area.y + self.board_area.h),
            Point::new(self.board_area.x + self.board_area.w, self.board_area.y + self.board_area.h)
        );
    }

    fn draw_goal(&mut self, goal: &Goal) {
        self.tile_at(goal.tile.x as usize, goal.tile.y as usize).ttype = TileType::Goal;
    }

    fn draw_snake(&mut self, snake: &Snake) {
        for tile in snake.body.clone() {
            *self.tile_at(tile.x as usize, tile.y as usize) = tile.clone(); 
        }
    }

    fn draw_tiles(&self, snake: &Snake, canvas: &mut Canvas<Window>) {
        for y in 0..20 {
            for x in 0..20 {
                let cur_tile_type = self.tiles[x + 20 * y].ttype.clone();
                match cur_tile_type {
                    TileType::Empty => {
                        continue;
                    }
                    TileType::Goal => {
                        canvas.set_draw_color(self.goal_color);
                        self.draw_tile(x, y, canvas);
                    }
                    _ => {
                        // Snake
                        if snake.alive() {
                            canvas.set_draw_color(Color::RGB(200, 200, 200));
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
        let x: i32 = self.board_area.x() + i32::try_from(posx).unwrap() * (self.board_area.width() as i32)/20;
        let y: i32 = self.board_area.y() + i32::try_from(posy).unwrap() * (self.board_area.height() as i32)/20;
        let tile_rect: Rect = Rect::new(x + 4, y + 4, self.board_area.width()/20 - 4, self.board_area.height()/20 - 4);
        // Assume the correct draw color is set in the canvas
        let _ = canvas.fill_rect(tile_rect);
    }

}
