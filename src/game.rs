use sdl3::render::Canvas;
use sdl3::video::Window;
use glam::IVec2;

use std::error::Error;

use crate::board::Board;
use crate::score::ScoreBoard;
use crate::snake::{Snake, Direction};
use crate::goal::Goal;
use crate::time::FrameTimer;

pub struct Game {
    //wnd_width: u32,
    //wnd_height: u32,
    pub board: Board,
    pub score: ScoreBoard,
    pub goal: Goal,
    pub snake: Snake,
    pub ft: FrameTimer,
    pub delay: bool,
    pub pause: bool,
    pub millis_per_move: u128
}

impl Game {
    
    pub fn new(wnd_w: u32, wnd_h: u32) -> Result<Self, Box<dyn Error>> {
        // TODO: use smaller of the two values as the board length and width
        // and center both
        Ok(Self {
            //wnd_width: window_width,
            //wnd_height: window_height,
            board: Board::new((wnd_w/20 - 5).try_into()?,
                                (wnd_h/20 - 5).try_into()?,
                                wnd_w*9/10, wnd_h*9/10),
            score: ScoreBoard::new(wnd_w, wnd_h),
            goal: Goal::new(),
            snake: Snake::new(IVec2 {x: 10, y: 10}),
            ft: FrameTimer::new(),
            delay: true,
            pause: false,
            millis_per_move: 90u128
        })
    }

    pub fn resize_wnd(&mut self, wnd: &Window) -> Result<(), Box<dyn Error>> {
        // Todo: get the size of the window and calculate the board size from there.

        Ok(())
    }

    pub fn draw_wnd(&mut self, canvas: &mut Canvas<Window>) -> Result<(), Box<dyn Error>> {
        self.board.render(&self.goal, &self.snake, canvas)?;
        self.score.draw_tallies(canvas);
        canvas.present();

        Ok(())
    }

    pub fn clear_wnd(&self, canvas: &mut Canvas<Window>) {
        
        // Black background color
        canvas.set_draw_color(sdl3::pixels::Color::RGB(0, 0, 0));
        canvas.clear(); // Paint the background color
    }

    pub fn toggle_pause(&mut self) {
        self.pause = !self.pause;
    }

    pub fn update(&mut self) {
        // If the game is paused, do not update the game
        if self.pause {
            return;
        }
        // If the game delay variable is set, delay the game-play 
        if self.delay {
            self.delay(1500); // Delay for 1.5 seconds
            self.delay = false;
        }
        // If 100 milliseconds has passed since last update, update
        if !self.ft.mark(self.millis_per_move) {
            return;
        }
        // Otherwise, game logic:
        if self.snake.alive() {
            // Move the snake if possible
            if self.snake.within_board() && !self.snake.eating_tail() {
                self.snake.try_eat_goal(&mut self.score, &mut self.goal, &mut self.millis_per_move);
                self.snake.move_once(&mut self.board);
            }
            else {
                self.snake.die();
            }
            // Change the goal color
            self.goal.new_color();
        }
        else {
            // TODO: Show final score or something
        }
    }

    pub fn handle_key_press(&mut self, key: sdl3::keyboard::Keycode)
    {
        // Do not handle key presses for game logic if the game is paused
        if self.pause {
            return;
        }
        // Otherwise, process arrow-key presses -->
        // Calculate the current direction. Using a calculation 
        // based on the first two tiles of the snake fixes several
        // bugs where you can accidentally go backwards and eat yourself
        let head_tile = self.snake.body[0].clone();
        let first_body_tile = self.snake.body[1].clone();
        let current_direction = IVec2::new(head_tile.x - first_body_tile.x, head_tile.y - first_body_tile.y);
        match key {
            // If the arrow key that is pressed is opposite of the
            // current direction then do nothing (otherwise it would kill the snake)
            sdl3::keyboard::Keycode::Up => {
                if current_direction != Direction::Down.vec() {
                    self.snake.dir = Direction::Up;
                }
            }
            sdl3::keyboard::Keycode::Down => {
                if current_direction != Direction::Up.vec() {
                    self.snake.dir = Direction::Down;
                }
            }
            sdl3::keyboard::Keycode::Left => {
                if current_direction != Direction::Right.vec() {
                    self.snake.dir = Direction::Left;
                }
            }
            sdl3::keyboard::Keycode::Right => {
                if current_direction != Direction::Left.vec() {
                    self.snake.dir = Direction::Right;
                }
            }
            _ => {/*Do nothing*/}
        }
    }

    fn delay(&mut self, milliseconds: u128) {
        while !self.ft.mark(milliseconds) {
            // TODO: Display a countdown? Or keep it like this and do nothing
        }
    }
}
