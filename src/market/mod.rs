use std::collections::HashMap;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::factions::FactionId;
use crate::resources::ResourceType;

/// Market order type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    Buy,
    Sell,
}

/// A market order
#[derive(Debug, Clone, Component)]
pub struct MarketOrder {
    pub id: u32,
    pub faction_id: FactionId,
    pub resource_type: ResourceType,
    pub order_type: OrderType,
    pub quantity: u32,
    pub price: f32, // Price per unit
    pub filled: u32,
}

impl MarketOrder {
    pub fn new(
        id: u32,
        faction_id: FactionId,
        resource_type: ResourceType,
        order_type: OrderType,
        quantity: u32,
        price: f32,
    ) -> Self {
        Self {
            id,
            faction_id,
            resource_type,
            order_type,
            quantity,
            price,
            filled: 0,
        }
    }

    pub fn remaining(&self) -> u32 {
        self.quantity.saturating_sub(self.filled)
    }

    pub fn is_complete(&self) -> bool {
        self.filled >= self.quantity
    }
}

/// Market building component
#[derive(Debug, Clone, Component)]
pub struct Market {
    pub fee_rate: f32, // Transaction fee (0.0 to 1.0)
    pub orders: Vec<u32>, // Order IDs
}

impl Default for Market {
    fn default() -> Self {
        Self::new()
    }
}

impl Market {
    pub fn new() -> Self {
        Self {
            fee_rate: 0.05, // 5% fee
            orders: Vec::new(),
        }
    }
}

/// Resource prices with history
#[derive(Debug, Clone, Resource)]
pub struct MarketPrices {
    prices: HashMap<ResourceType, ResourcePrice>,
    order_counter: u32,
}

impl Default for MarketPrices {
    fn default() -> Self {
        Self::new()
    }
}

impl MarketPrices {
    pub fn new() -> Self {
        let mut prices = Self {
            prices: HashMap::new(),
            order_counter: 0,
        };
        prices.initialize_prices();
        prices
    }

    fn initialize_prices(&mut self) {
        // Base prices for each resource
        let base_prices = [
            (ResourceType::Power, 1.0),
            (ResourceType::Iron, 2.0),
            (ResourceType::Copper, 3.0),
            (ResourceType::Silicon, 5.0),
            (ResourceType::Crystal, 8.0),
            (ResourceType::Carbon, 1.5),
            (ResourceType::Stone, 0.5),
            (ResourceType::Sulfur, 4.0),
            (ResourceType::Water, 0.3),
            (ResourceType::Biomass, 2.5),
        ];

        for (resource, price) in base_prices {
            self.prices.insert(resource, ResourcePrice::new(price));
        }
    }

    pub fn get_price(&self, resource: ResourceType) -> f32 {
        self.prices.get(&resource).map(|p| p.current).unwrap_or(1.0)
    }

    pub fn update_price(&mut self, resource: ResourceType, demand: f32, supply: f32) {
        if let Some(price) = self.prices.get_mut(&resource) {
            price.update(demand, supply);
        }
    }

    pub fn next_order_id(&mut self) -> u32 {
        self.order_counter += 1;
        self.order_counter
    }
}

/// Price history for a resource
#[derive(Debug, Clone)]
struct ResourcePrice {
    current: f32,
    base: f32,
    history: Vec<f32>,
    max_history: usize,
}

impl ResourcePrice {
    fn new(base: f32) -> Self {
        Self {
            current: base,
            base,
            history: Vec::new(),
            max_history: 100,
        }
    }

    fn update(&mut self, demand: f32, supply: f32) {
        // Simple supply/demand pricing
        let ratio = if supply > 0.0 { demand / supply } else { demand };
        let adjustment = (ratio - 1.0) * 0.1; // 10% adjustment per unit ratio difference
        self.current = (self.base * (1.0 + adjustment)).max(0.1);
        self.history.push(self.current);
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }
    }
}

/// Trade execution result
#[derive(Debug, Clone)]
pub struct TradeResult {
    pub buyer_id: FactionId,
    pub seller_id: FactionId,
    pub resource: ResourceType,
    pub quantity: u32,
    pub total_price: f32,
    pub fee: f32,
}

/// System to process market orders
pub fn market_system(
    mut orders: Query<(Entity, &mut MarketOrder)>,
    mut prices: ResMut<MarketPrices>,
    mut game_log: ResMut<crate::ui::GameLog>,
) {
    // Collect buy and sell orders
    let mut buy_orders: Vec<(Entity, MarketOrder)> = Vec::new();
    let mut sell_orders: Vec<(Entity, MarketOrder)> = Vec::new();

    for (entity, order) in orders.iter() {
        if order.is_complete() {
            continue;
        }
        match order.order_type {
            OrderType::Buy => buy_orders.push((entity, order.clone())),
            OrderType::Sell => sell_orders.push((entity, order.clone())),
        }
    }

    // Sort buy orders by price (highest first)
    buy_orders.sort_by(|a, b| b.1.price.partial_cmp(&a.1.price).unwrap_or(std::cmp::Ordering::Equal));

    // Sort sell orders by price (lowest first)
    sell_orders.sort_by(|a, b| a.1.price.partial_cmp(&b.1.price).unwrap_or(std::cmp::Ordering::Equal));

    // Match orders
    for (buy_entity, buy_order) in &buy_orders {
        for (sell_entity, sell_order) in &sell_orders {
            if buy_order.resource_type != sell_order.resource_type {
                continue;
            }
            if buy_order.faction_id == sell_order.faction_id {
                continue;
            }
            if buy_order.price < sell_order.price {
                continue; // No match possible
            }

            // Execute trade
            let quantity = buy_order.remaining().min(sell_order.remaining());
            let price = sell_order.price; // Use seller's price
            let total = price * quantity as f32;

            // Update orders
            if let Ok((_, mut order)) = orders.get_mut(*buy_entity) {
                order.filled += quantity;
            }
            if let Ok((_, mut order)) = orders.get_mut(*sell_entity) {
                order.filled += quantity;
            }

            // Log trade
            game_log.add(format!(
                "Trade: {} {} @ {:.2} (Buyer: {}, Seller: {})",
                quantity,
                format!("{:?}", buy_order.resource_type),
                price,
                buy_order.faction_id.0,
                sell_order.faction_id.0
            ));

            // Update market prices based on trade
            prices.update_price(buy_order.resource_type, quantity as f32, quantity as f32);
        }
    }
}

/// Plugin for market system
pub struct MarketPlugin;

impl Plugin for MarketPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MarketPrices>();
    }
}