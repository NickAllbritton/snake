use sdl2::render::Canvas;
use sdl2::video::Window;
use glam::IVec2;

use crate::view::board::Board;
use crate::model::snake::{Snake, Direction};
use crate::model::goal::Goal;
use crate::model::time::FrameTimer;

pub struct Game {
    //wnd_width: u32,
    //wnd_height: u32,
    pub board: Board,
    pub goal: Goal,
    pub snake: Snake,
    pub ft: FrameTimer
}

impl Game {
    
    pub fn new(window_width: u32, window_height: u32) -> Self {
        Self {
            //wnd_width: window_width,
            //wnd_height: window_height,
            board: Board::new((window_width/20 - 5).try_into().unwrap(),
                                (window_height/20 - 5).try_into().unwrap(),
                                window_width*9/10, window_height*9/10),
            goal: Goal::new(),
            snake: Snake::new(IVec2 {x: 10, y: 10}),
            ft: FrameTimer::new()
        }
    }

    pub fn draw_wnd(&mut self, canvas: &mut Canvas<Window>) {
        self.board.render(&self.goal, &self.snake, canvas);
        canvas.present();
    }

    pub fn clear_wnd(&self, canvas: &mut Canvas<Window>) {
        
        // Black background color
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear(); // Paint the background color
    }

    pub fn update(&mut self) {
        // TODO: Update according to game logic
        //
        // If 50 milliseconds has passed since last update, update
        if !self.ft.mark(100) {
            return;
        }
        
        if self.snake.alive() {
            if self.snake.within_board() {
                self.snake.try_eat_goal(&mut self.goal);
                self.snake.move_once(&mut self.board);
            }
            else {
                self.snake.die();
            }
        }
        else {
            // TODO: Show final score or something
        }
    }

    pub fn handle_key_press(&mut self, key: sdl2::keyboard::Keycode)
    {
        match key {
            sdl2::keyboard::Keycode::Up => {
                self.snake.dir = Direction::Up;
            },
            sdl2::keyboard::Keycode::Down => {
                self.snake.dir = Direction::Down;
            },
            sdl2::keyboard::Keycode::Left => {
                self.snake.dir = Direction::Left;
            },
            sdl2::keyboard::Keycode::Right => {
                self.snake.dir = Direction::Right;
            },
            _ => {/*Do nothing*/}
        }
    }
}
