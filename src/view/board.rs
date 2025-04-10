use sdl2::rect::{Rect,Point};
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Board {
    pub board_area: Rect,
    pub goal_color: Color,
    // Array of tiles
}

impl Board {
    pub fn new(posx: i32, posy: i32, width: u32, height: u32) -> Self {
        Self {
            board_area: Rect::new(posx, posy, width, height),
            goal_color: Color::RGB(100, 100, 100)
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        self.draw_edges(canvas);
    }

    pub fn draw_edges(&self, canvas: &mut Canvas<Window>) {
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
}
