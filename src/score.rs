use sdl3::rect::Rect;
use sdl3::pixels::Color;
use sdl3::render::Canvas;
use sdl3::video::Window;

pub struct ScoreBoard {
    pub goals_collected: u32,
    draw_pos: (i32, i32),
    pos: (i32, i32),
    wnd_width: i32,
    wnd_height: i32,
    tally_width: i32,
    tally_height: i32,
    padding_width: i32,
}

impl ScoreBoard {

    pub fn new(wnd_width: u32, wnd_height: u32) -> Self {
        // borders padding TEST
        let padding: i32 = 10i32;
        let draw_pos = ((wnd_width as i32 - padding)/20 - 6, (wnd_height as i32 - padding)*772/800);
        Self {
            goals_collected: 0u32,
            draw_pos: draw_pos,
            pos: (0i32, 0i32),
            wnd_width: (wnd_width as i32 - padding),
            wnd_height: (wnd_height as i32 - padding),
            tally_width: 4i32,
            tally_height: 26i32,
            padding_width: 14i32,
        }
    }

    pub fn draw_tallies(&mut self, canvas: &mut Canvas<Window>) {
        if self.goals_collected == 0u32 { return; }
        // Record the first tallies draw position
        let draw_pos_original: (i32, i32) = self.draw_pos;
        let tally_dimensions: (i32, i32) = (self.tally_width, self.tally_height);
        for i in 0..self.goals_collected {
            canvas.set_draw_color(Color::RGB(170, 170, 170));
            // Calculate the modulus of the index of the current tally by the
            // number of tallies for the width of game screen
            let ntallies_per_side: u32 = 65u32; // small test value
            let nrotations: u32 = (i as f32 / ntallies_per_side as f32).floor() as u32;
            // Calculate what the drawing position should be if the tick were drawn on the bottom
            // of the screen where it starts (calculate displacement along the bottom
            self.draw_pos = (self.draw_pos.0+self.padding_width*((i%ntallies_per_side) as i32), self.draw_pos.1);
            // Calculate the true position from the drawing position in screen space
            self.pos = self.true_pos(self.draw_pos);
            // Calculate the center of the tally
            self.pos.0 += tally_dimensions.0/2i32;
            self.pos.1 -= tally_dimensions.1/2i32;
            // Calculate the correct padding to add between tiles
            if nrotations % 2 == 0 {
                self.tally_width = 4i32;
                self.tally_height = 26i32;
            }
            else {
                self.tally_width = 26i32;
                self.tally_height = 4i32;
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
            self.pos.0 -= self.tally_width/2i32;
            self.pos.1 += self.tally_height/2i32;
            // Calculate the drawing position from the true position
            self.draw_pos = self.screen_pos(self.pos);
            let tally = Rect::new(self.draw_pos.0, self.draw_pos.1, self.tally_width as u32, self.tally_height as u32);
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

    fn screen_pos(&self, true_pos: (i32, i32)) -> (i32, i32) {
        (true_pos.0 + self.wnd_width/2, -true_pos.1 + self.wnd_height/2) 
    }

    fn true_pos(&self, screen_pos: (i32, i32)) -> (i32, i32) {
        (screen_pos.0 - self.wnd_width/2, self.wnd_height/2 - screen_pos.1) 
    }

}
