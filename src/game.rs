use sdl2::render::Canvas;
use sdl2::video::Window;
use glam::IVec2;

use crate::board::Board;
use crate::snake::{Snake, Direction};
use crate::goal::Goal;
use crate::time::FrameTimer;

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
        // If 50 milliseconds has passed since last update, update
        if !self.ft.mark(100) {
            return;
        }
        
        if self.snake.alive() {
            if self.snake.within_board() && !self.snake.eating_tail() {
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
            // Check for arrow keys and handle them
            // If the arrow key that is pressed is opposite of the
            // current direction then do nothing (otherwise it would kill the snake)
            sdl2::keyboard::Keycode::Up => {
                if self.snake.dir != Direction::Down {
                    self.snake.dir = Direction::Up;
                }
            },
            sdl2::keyboard::Keycode::Down => {
                if self.snake.dir != Direction::Up {
                    self.snake.dir = Direction::Down;
                }
            },
            sdl2::keyboard::Keycode::Left => {
                if self.snake.dir != Direction::Right {
                    self.snake.dir = Direction::Left;
                }
            },
            sdl2::keyboard::Keycode::Right => {
                if self.snake.dir != Direction::Left {
                    self.snake.dir = Direction::Right;
                }
            },
            _ => {/*Do nothing*/}
        }
    }
}
