use bevy::prelude::*;

use crate::market::{MarketPrices, MarketOrder, OrderType};
use crate::resources::ResourceType;
use crate::ui::GameLog;

/// Market UI state
#[derive(Resource, Debug, Clone, Default)]
pub struct MarketUI {
    pub visible: bool,
    pub selected_resource: Option<ResourceType>,
}

/// System to toggle market UI
pub fn toggle_market_ui(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut market_ui: ResMut<MarketUI>,
) {
    if keyboard.just_pressed(KeyCode::KeyK) {
        market_ui.visible = !market_ui.visible;
    }
}

/// System to render market UI
pub fn market_ui_system(
    market_ui: Res<MarketUI>,
    market_prices: Res<MarketPrices>,
    orders: Query<&MarketOrder>,
    mut game_log: ResMut<GameLog>,
) {
    if !market_ui.visible {
        return;
    }

    // Display market info in game log
    if market_ui.is_changed() && market_ui.visible {
        game_log.add("=== Market ===".to_string());

        // Show prices for all resources
        game_log.add(format!("Power: {:.2} credits", market_prices.get_price(ResourceType::Power)));
        game_log.add(format!("Iron: {:.2} credits", market_prices.get_price(ResourceType::Iron)));
        game_log.add(format!("Copper: {:.2} credits", market_prices.get_price(ResourceType::Copper)));
        game_log.add(format!("Silicon: {:.2} credits", market_prices.get_price(ResourceType::Silicon)));
        game_log.add(format!("Crystal: {:.2} credits", market_prices.get_price(ResourceType::Crystal)));

        // Count active orders
        let buy_orders = orders.iter().filter(|o| o.order_type == OrderType::Buy).count();
        let sell_orders = orders.iter().filter(|o| o.order_type == OrderType::Sell).count();

        game_log.add(format!("Buy orders: {}", buy_orders));
        game_log.add(format!("Sell orders: {}", sell_orders));
        game_log.add("Press K to close".to_string());
    }
}

/// Plugin for market UI
pub struct MarketUIPlugin;

impl Plugin for MarketUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MarketUI>()
            .add_systems(Update, (
                toggle_market_ui,
                market_ui_system,
            ));
    }
}