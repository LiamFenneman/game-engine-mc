use std::time::Instant;

#[derive(Debug)]
pub struct FrameStats {
    pub current_fps: u32,
    last_time: Instant,
    pub delta_time: f64,
    pub frame_count: u32,
    frame_time: f64,
}

impl Default for FrameStats {
    fn default() -> Self {
        return Self {
            current_fps: 0,
            last_time: Instant::now(),
            delta_time: 0.0,
            frame_count: 0,
            frame_time: 0.0,
        };
    }
}

impl FrameStats {
    fn delta(&mut self) -> f64 {
        let current_time = Instant::now();
        let delta = self.last_time.elapsed().as_secs_f64();
        self.last_time = current_time;
        self.delta_time = delta;
        return delta;
    }

    pub fn fps(&mut self) {
        self.delta();
        self.frame_count += 1;
        self.frame_time += self.delta_time;

        let tmp;
        if self.frame_time >= 1.0 {
            tmp = self.frame_count;
            self.frame_count = 0;
            self.frame_time = 0.0;
            self.current_fps = tmp;
        }
    }
}
