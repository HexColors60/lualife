/// Status line widget for displaying game state
pub struct StatusLine {
    pub text: String,
    pub tick: u64,
    pub paused: bool,
    pub speed: f32,
}

impl Default for StatusLine {
    fn default() -> Self {
        Self::new()
    }
}

impl StatusLine {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            tick: 0,
            paused: false,
            speed: 1.0,
        }
    }

    pub fn update(&mut self, tick: u64, paused: bool, speed: f32) {
        self.tick = tick;
        self.paused = paused;
        self.speed = speed;
        self.text = format!(
            "Tick: {} | {} | Speed: {:.1}x",
            tick,
            if paused { "PAUSED" } else { "RUNNING" },
            speed
        );
    }
}
