use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct ScoreBoard {
    pub goals_collected: u32,
    draw_pos: (u32, u32),
    tally_width: u32,
    tally_height: u32,
    padding_width: u32
}

impl ScoreBoard {

    pub fn new(posx: u32, posy: u32) -> Self {
        Self {
            goals_collected: 0u32,
            draw_pos: (posx, posy),
            tally_width: 3u32,
            tally_height: 25u32,
            padding_width: 10u32
        }
    }

    pub fn draw_tallies(&self, canvas: &mut Canvas<Window>) {
        if self.goals_collected == 0u32 { return; }
        for i in 0..self.goals_collected {
            canvas.set_draw_color(Color::RGB(170, 170, 170));
            let tally = Rect::new((self.draw_pos.0 + (self.tally_width+self.padding_width)*(i as u32)).try_into().unwrap(),
                        (self.draw_pos.1).try_into().unwrap(), self.tally_width, self.tally_height);
            let _ = canvas.fill_rect(tally);
        }
    }

    pub fn collect_goal(&mut self) {
        self.goals_collected += 1;
    }

}
