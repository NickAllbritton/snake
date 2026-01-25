use sdl3::rect::Rect;
use sdl3::pixels::Color;
use sdl3::render::Canvas;
use sdl3::video::Window;

use std::error::Error;

pub struct ScoreBoard {
    pub goals_collected: u32,
    draw_pos: (f32, f32),
    pos: (f32, f32),
    wnd_width: f32,
    wnd_height: f32,
    tally_width: f32,
    tally_height: f32,
    padding_width: f32,
    tallies_per_side: u32
}

impl ScoreBoard {

    pub fn new(wnd_w: u32, wnd_h: u32, side: u32) -> Result<Self, Box<dyn Error>> {
        // Smallest side of Window
        let smallest_side: f32 = std::cmp::min(wnd_w, wnd_h) as f32;
        // Length of tallies should be less than 5% of side length.
        let tally_height: f32 = smallest_side*3f32/100f32;
        // Width of the tallies should be much smaller
        let tally_width: f32 = smallest_side*1f32/300f32;
        // Padding between edge of board and nearest tallies (not between tallies)
        let padding_edge: f32 = smallest_side*3f32/200f32;
        // Score per side (before tallies wrap to next side of board border
        let score_per_side: u32 = 29;
        // Padding between tallies (algebra comes from geometry where there are n-1 paddings for
        // ntallies per side with type chaos preventing code readibility... refactor?
        // p - padding between tallies (padding_between)
        // n - number of tallies (score_per_side)
        // s - side length (side)
        // w - width of each tally (tally_width)
        // p = (s - wn)/(n - 1)
        let total_tally_width: f32 = (score_per_side as f32)*tally_width;
        let padding_between: f32 = (side as f32 - total_tally_width)/((score_per_side-1) as f32);
        // Start drawing in bottom left corner of board plus padding
        let draw_pos = (((wnd_w-side) as f32)/2f32, ((wnd_h+side) as f32)/2f32 + padding_edge);
        Ok(Self {
            goals_collected: 0u32, // TESTING
            draw_pos: draw_pos,
            pos: (0f32, 0f32),
            wnd_width: wnd_w as f32,
            wnd_height: wnd_h as f32,
            tally_width: tally_width,
            tally_height: tally_height,
            padding_width: padding_between,
            tallies_per_side: score_per_side
        })
    }

    pub fn draw_tallies(&mut self, canvas: &mut Canvas<Window>) {
        if self.goals_collected == 0u32 { return; }
        // Record the first tallies draw position
        let draw_pos_original: (f32, f32) = self.draw_pos;
        let tally_dimensions: (f32, f32) = (self.tally_width, self.tally_height);
        for i in 0..self.goals_collected {
            canvas.set_draw_color(Color::RGB(170, 170, 170));
            // Calculate the modulus of the index of the current tally by the
            // number of tallies for the width of game screen
            let nrotations: u32 = (i as f32 / self.tallies_per_side as f32).trunc() as u32;
            // Calculate which tally from 0 to self.tallies_per_side-1 this iteration should draw
            // on the current edge of the board
            let i_this_side: i32 = (i%self.tallies_per_side) as i32;
            // Calculate what the drawing position should be if the tick were drawn on the bottom
            // of the screen where it starts (calculate displacement along the bottom)
            // 1. Calculate offset for regular padding and width of each tally
            let regular_offset: f32 = (self.tally_width+self.padding_width)*(i_this_side as f32);
            // Use the calculated offsets to prepare the drawing position for this iteration
            self.draw_pos = (self.draw_pos.0+regular_offset, self.draw_pos.1);

            // Calculate the true position from the drawing position in screen space
            self.pos = self.true_pos(self.draw_pos);
            // Calculate the center of the tally
            self.pos.0 += tally_dimensions.0/2f32;
            self.pos.1 -= tally_dimensions.1/2f32;
            // Based on the number of rotations made around the board while counting tallies, swap
            // width and height of the tallies so they are either vertical or horizontal
            if nrotations % 2 == 0 {
                self.tally_width = tally_dimensions.0;
                self.tally_height = tally_dimensions.1;
            }
            else {
                self.tally_width = tally_dimensions.1;
                self.tally_height = tally_dimensions.0;
            }
            //                          [cosx -sinx]
            // Use rotation matrix ->   [          ] 
            //                          [sinx  cosx]
            // Use the previously calculated modulus as a loop counter to 
            // determine how many rotations there should be. 
            // Then this rotation matrix should be multiplied by the position vector.
            // Except this matrix only works if the center of the game board were the 
            // position 0,0
            match (nrotations as u32) % 4u32 {
                0u32 => {} // No rotation
                1u32 => { // 90 degrees
                    // Calculate the position as the dot product of the two row vectors
                    // [0  -1]
                    // [1   0]
                    let prev_pos_x = self.pos.0; // store the x pos in temp variable
                    self.pos.0 = -self.pos.1;
                    self.pos.1 = prev_pos_x;
               }
                2u32 => { // 180 degrees
                    // Calculate the position as the dot product of the two row vectors
                    // [-1  0]
                    // [0  -1]
                    self.pos.0 = -self.pos.0;
                    self.pos.1 = -self.pos.1;
                }
                3u32 => { // 270 degrees
                    // Calculate the position as the dot product of the two row vectors
                    // [0   1]
                    // [-1  0]
                    let prev_pos_x = self.pos.0; // store the x pos in temp variable
                    self.pos.0 = self.pos.1;
                    self.pos.1 = -prev_pos_x;
                }
                _ => {} // Error handle? Impossible to reach though since modulo 4 is 0-3
            }
            // Calculate the top left corner of the tally 
            self.pos.0 -= self.tally_width/2f32;
            self.pos.1 += self.tally_height/2f32;
            // Calculate the drawing position from the true position
            self.draw_pos = self.screen_pos(self.pos);
            let tally = Rect::new(self.draw_pos.0.trunc() as i32, self.draw_pos.1.trunc() as i32, 
                                self.tally_width.ceil() as u32, self.tally_height.ceil() as u32);
            let _ = canvas.fill_rect(tally);
            // Reset the draw_pos
            self.draw_pos = draw_pos_original;
            self.tally_width = tally_dimensions.0;
            self.tally_height = tally_dimensions.1;
        }
    }

    pub fn collect_goal(&mut self) {
        self.goals_collected += 1;
    }

    fn screen_pos(&self, true_pos: (f32, f32)) -> (f32, f32) {
        (true_pos.0 + self.wnd_width/2f32, -true_pos.1 + self.wnd_height/2f32) 
    }

    fn true_pos(&self, screen_pos: (f32, f32)) -> (f32, f32) {
        (screen_pos.0 - self.wnd_width/2f32, self.wnd_height/2f32 - screen_pos.1) 
    }

}
