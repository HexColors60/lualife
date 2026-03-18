use bevy::prelude::*;
use discord_rich_presence::{
    activity::{Activity, Assets, Button, Timestamps},
    DiscordIpc, DiscordIpcClient,
};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::core::{GameState, TickNumber};
use crate::factions::FactionRegistry;

pub const DISCORD_APPLICATION_ID: &str = "1326165701574774785";

#[derive(Resource, Debug, Clone)]
pub struct DiscordConfig {
    pub enabled: bool,
    pub application_id: String,
    pub show_tick_count: bool,
    pub show_faction_count: bool,
    pub update_interval_seconds: f32,
}

impl Default for DiscordConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            application_id: DISCORD_APPLICATION_ID.to_string(),
            show_tick_count: true,
            show_faction_count: true,
            update_interval_seconds: 15.0,
        }
    }
}

#[derive(Resource)]
pub struct DiscordClient {
    client: Arc<Mutex<Option<DiscordIpcClient>>>,
    connected: bool,
    last_update: Instant,
}

impl DiscordClient {
    pub fn new(application_id: &str) -> Self {
        let client = DiscordIpcClient::new(application_id)
            .map(|c| {
                let mut c = c;
                let _ = c.connect();
                Some(c)
            })
            .unwrap_or(None);

        Self {
            client: Arc::new(Mutex::new(client)),
            connected: false,
            last_update: Instant::now()
                .checked_sub(Duration::from_secs(100))
                .unwrap_or_else(Instant::now),
        }
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn connect(&mut self) -> bool {
        if let Ok(mut client) = self.client.lock() {
            if client.is_none() {
                *client = DiscordIpcClient::new(DISCORD_APPLICATION_ID).ok().map(|mut c| {
                    let _ = c.connect();
                    c
                });
            }
            self.connected = client.is_some();
            self.connected
        } else {
            false
        }
    }

    pub fn disconnect(&mut self) {
        if let Ok(mut client) = self.client.lock() {
            if let Some(mut c) = client.take() {
                let _ = c.close();
            }
        }
        self.connected = false;
    }

    pub fn update_activity(&mut self, activity: Activity) -> bool {
        if let Ok(mut client) = self.client.lock() {
            if let Some(ref mut c) = *client {
                let result = c.set_activity(activity);
                self.connected = result.is_ok();
                return self.connected;
            }
        }
        false
    }

    pub fn should_update(&self, interval_seconds: f32) -> bool {
        self.last_update.elapsed().as_secs_f32() >= interval_seconds
    }

    pub fn mark_updated(&mut self) {
        self.last_update = Instant::now();
    }
}

#[derive(Resource, Debug, Clone, Default)]
pub struct DiscordState {
    pub current_details: Option<String>,
    pub current_state: Option<String>,
    pub start_timestamp: Option<u64>,
}

impl DiscordState {
    pub fn new() -> Self {
        Self {
            current_details: None,
            current_state: None,
            start_timestamp: Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            ),
        }
    }
}

#[derive(Event, Debug, Clone)]
pub struct DiscordPresenceUpdate {
    pub details: Option<String>,
    pub state: Option<String>,
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
    pub buttons: Vec<(String, String)>,
}

impl Default for DiscordPresenceUpdate {
    fn default() -> Self {
        Self {
            details: None,
            state: None,
            large_image: Some("logo".to_string()),
            large_text: Some("LuaLife".to_string()),
            small_image: None,
            small_text: None,
            buttons: vec![],
        }
    }
}

pub fn discord_setup_system(
    mut commands: Commands,
    config: Res<DiscordConfig>,
) {
    if !config.enabled {
        return;
    }

    let client = DiscordClient::new(&config.application_id);
    let state = DiscordState::new();
    
    commands.insert_resource(client);
    commands.insert_resource(state);
    
    info!("Discord Rich Presence initialized");
}

pub fn discord_update_system(
    mut discord_client: ResMut<DiscordClient>,
    mut discord_state: ResMut<DiscordState>,
    config: Res<DiscordConfig>,
    game_state: Res<GameState>,
    tick: Res<TickNumber>,
    faction_registry: Res<FactionRegistry>,
    mut events: EventReader<DiscordPresenceUpdate>,
) {
    if !config.enabled {
        return;
    }

    if !discord_client.is_connected() {
        if !discord_client.connect() {
            return;
        }
    }

    if let Some(update) = events.read().last() {
        if let Some(ref details) = update.details {
            discord_state.current_details = Some(details.clone());
        }
        if let Some(ref state) = update.state {
            discord_state.current_state = Some(state.clone());
        }
    }

    if !discord_client.should_update(config.update_interval_seconds) {
        return;
    }

    let details = format!(
        "Tick: {} | Factions: {}",
        tick.0,
        faction_registry.count()
    );

    let state_text = match *game_state {
        GameState::Loading => "Loading...",
        GameState::Running => "In Game",
        GameState::Paused => "Paused",
        GameState::SingleStep => "Stepping",
        GameState::GameOver => "Game Over",
    };

    let mut activity = Activity::new()
        .details(&details)
        .state(state_text)
        .assets(
            Assets::new()
                .large_image("logo")
                .large_text("LuaLife - Autonomous Simulation")
        );

    if let Some(ts) = discord_state.start_timestamp {
        activity = activity.timestamps(Timestamps::new().start(ts as i64));
    }

    let buttons = vec![
        Button::new("GitHub", "https://github.com/lualife/lualife"),
    ];
    
    activity = activity.buttons(buttons);

    if discord_client.update_activity(activity) {
        discord_client.mark_updated();
    }
}

pub fn discord_cleanup_system(
    mut discord_client: ResMut<DiscordClient>,
) {
    discord_client.disconnect();
    info!("Discord Rich Presence disconnected");
}

pub fn discord_game_state_system(
    game_state: Res<GameState>,
    mut events: EventWriter<DiscordPresenceUpdate>,
) {
    if game_state.is_changed() {
        let state_text = match *game_state {
            GameState::Loading => "Loading...",
            GameState::Running => "In Game",
            GameState::Paused => "Paused",
            GameState::SingleStep => "Stepping",
            GameState::GameOver => "Game Over",
        };

        events.send(DiscordPresenceUpdate {
            state: Some(state_text.to_string()),
            ..default()
        });
    }
}

pub struct DiscordPlugin;

impl Plugin for DiscordPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DiscordConfig>()
            .init_resource::<DiscordState>()
            .add_event::<DiscordPresenceUpdate>()
            .add_systems(Startup, discord_setup_system)
            .add_systems(
                Update,
                (
                    discord_update_system,
                    discord_game_state_system,
                ),
            )
            .add_systems(OnExit(crate::core::GameState::GameOver), discord_cleanup_system);
    }
}