use std::time::{Instant, Duration};

pub struct FrameTimer {
    last_frame: Instant
}

impl FrameTimer {

    pub fn new() -> Self {
        Self {
            last_frame: Instant::now()
        }
    }

    pub fn mark(&mut self, millis_perframe: u128) -> bool {
        let now = Instant::now();
        let new_frame = now.duration_since(self.last_frame).as_millis() > millis_perframe;
        if new_frame {
            self.last_frame = now;
        }
        return new_frame;
    }

}
