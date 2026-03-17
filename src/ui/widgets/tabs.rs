/// Tab widget for UI panels
pub struct Tabs {
    pub tabs: Vec<String>,
    pub active: usize,
}

impl Tabs {
    pub fn new(tabs: Vec<String>) -> Self {
        Self {
            tabs,
            active: 0,
        }
    }

    pub fn select(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.active = index;
        }
    }

    pub fn active_tab(&self) -> &str {
        self.tabs.get(self.active).map(|s| s.as_str()).unwrap_or("")
    }
}