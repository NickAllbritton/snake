use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::view::board::Board;

pub struct Game {
    //wnd_width: u32,
    //wnd_height: u32,
    pub board: Board
}

impl Game {
    
    pub fn new(window_width: u32, window_height: u32) -> Self {
        Self {
            //wnd_width: window_width,
            //wnd_height: window_height,
            board: Board::new((window_width/20 - 5).try_into().unwrap(),
                                (window_height/20 - 5).try_into().unwrap(),
                                window_width*9/10, window_height*9/10)
        }
    }

    pub fn draw_wnd(&self, canvas: &mut Canvas<Window>) {
        self.board.render(canvas);
        canvas.present();
    }

    pub fn clear_wnd(&self, canvas: &mut Canvas<Window>) {
        
        // Black background color
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear(); // Paint the background color
    }

    pub fn update(&self) {
        // TODO: Update according to game logic
    }
}
