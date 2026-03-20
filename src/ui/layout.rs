
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Panel {
    MapView,
    Minimap,
    LogPanel,
    PerfPanel,
    UnitPanel,
    BuildingPanel,
    FactionPanel,
}

#[derive(Debug, Clone)]
pub struct PanelLayout {
    pub panel: Panel,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl PanelLayout {
    pub fn new(panel: Panel, x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            panel,
            x,
            y,
            width,
            height,
        }
    }

    pub fn contains(&self, mouse_x: f32, mouse_y: f32) -> bool {
        mouse_x >= self.x
            && mouse_x <= self.x + self.width
            && mouse_y >= self.y
            && mouse_y <= self.y + self.height
    }
}

pub fn calculate_layouts(window_width: f32, window_height: f32) -> Vec<PanelLayout> {
    let mut layouts = Vec::new();

    // Main map view (left side, takes most space)
    let map_width = window_width * 0.7;
    layouts.push(PanelLayout::new(
        Panel::MapView,
        0.0,
        0.0,
        map_width,
        window_height,
    ));

    // Right side panels
    let right_x = map_width;
    let right_width = window_width - map_width;

    // Minimap (top right)
    let minimap_height = 200.0;
    layouts.push(PanelLayout::new(
        Panel::Minimap,
        right_x,
        0.0,
        right_width,
        minimap_height,
    ));

    // Unit panel (below minimap)
    let unit_panel_height = 300.0;
    layouts.push(PanelLayout::new(
        Panel::UnitPanel,
        right_x,
        minimap_height,
        right_width,
        unit_panel_height,
    ));

    // Log panel (bottom right)
    let log_panel_y = minimap_height + unit_panel_height;
    let log_panel_height = window_height - log_panel_y;
    layouts.push(PanelLayout::new(
        Panel::LogPanel,
        right_x,
        log_panel_y,
        right_width,
        log_panel_height,
    ));

    layouts
}
