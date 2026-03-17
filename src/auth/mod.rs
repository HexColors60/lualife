use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Player account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAccount {
    pub id: u64,
    pub username: String,
    pub display_name: String,
    pub rating: u32,
    pub games_played: u32,
    pub games_won: u32,
}

impl PlayerAccount {
    pub fn new(id: u64, username: String) -> Self {
        Self {
            id,
            display_name: username.clone(),
            username,
            rating: 1000,
            games_played: 0,
            games_won: 0,
        }
    }

    pub fn win_rate(&self) -> f32 {
        if self.games_played > 0 {
            self.games_won as f32 / self.games_played as f32
        } else {
            0.0
        }
    }
}

/// Authentication token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    pub player_id: u64,
    pub token: String,
    pub expires_at: u64, // Unix timestamp
}

impl AuthToken {
    pub fn new(player_id: u64, duration_secs: u64) -> Self {
        Self {
            player_id,
            token: uuid::Uuid::new_v4().to_string(),
            expires_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + duration_secs,
        }
    }

    pub fn is_valid(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now < self.expires_at
    }
}

/// Authentication state
#[derive(Resource, Debug, Clone, Default)]
pub struct AuthState {
    pub logged_in: bool,
    pub current_player: Option<PlayerAccount>,
    pub token: Option<AuthToken>,
}

impl AuthState {
    pub fn login(&mut self, player: PlayerAccount, token: AuthToken) {
        self.logged_in = true;
        self.current_player = Some(player);
        self.token = Some(token);
    }

    pub fn logout(&mut self) {
        self.logged_in = false;
        self.current_player = None;
        self.token = None;
    }

    pub fn is_authenticated(&self) -> bool {
        self.logged_in && self.token.as_ref().map(|t| t.is_valid()).unwrap_or(false)
    }
}

/// Authentication manager (server-side)
#[derive(Resource, Debug, Clone, Default)]
pub struct AuthManager {
    pub players: HashMap<u64, PlayerAccount>,
    pub tokens: HashMap<String, AuthToken>,
    pub username_to_id: HashMap<String, u64>,
    next_player_id: u64,
}

impl AuthManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, username: String) -> Result<PlayerAccount, String> {
        if self.username_to_id.contains_key(&username) {
            return Err("Username already taken".to_string());
        }

        let id = self.next_player_id;
        self.next_player_id += 1;

        let player = PlayerAccount::new(id, username.clone());
        self.players.insert(id, player.clone());
        self.username_to_id.insert(username, id);

        Ok(player)
    }

    pub fn login(&mut self, username: &str) -> Option<(PlayerAccount, AuthToken)> {
        let player_id = self.username_to_id.get(username)?;
        let player = self.players.get(player_id)?.clone();
        let token = AuthToken::new(player.id, 3600); // 1 hour

        self.tokens.insert(token.token.clone(), token.clone());

        Some((player, token))
    }

    pub fn validate_token(&self, token_str: &str) -> Option<&AuthToken> {
        let token = self.tokens.get(token_str)?;
        if token.is_valid() {
            Some(token)
        } else {
            None
        }
    }

    pub fn get_player(&self, player_id: u64) -> Option<&PlayerAccount> {
        self.players.get(&player_id)
    }
}

/// Authentication events
#[derive(Event, Debug, Clone)]
pub enum AuthEvent {
    LoginRequest {
        username: String,
    },
    RegisterRequest {
        username: String,
    },
    Logout,
    LoginSuccess {
        player: PlayerAccount,
        token: AuthToken,
    },
    LoginFailed {
        reason: String,
    },
}

/// System to process authentication events
pub fn auth_event_system(
    mut events: EventReader<AuthEvent>,
    mut auth_state: ResMut<AuthState>,
    mut auth_manager: ResMut<AuthManager>,
    mut game_log: ResMut<crate::ui::GameLog>,
) {
    for event in events.read() {
        match event {
            AuthEvent::LoginRequest { username } => {
                if let Some((player, token)) = auth_manager.login(username) {
                    auth_state.login(player.clone(), token.clone());
                    game_log.add(format!("Logged in as {}", player.display_name));
                } else {
                    game_log.add(format!("Login failed: user not found"));
                }
            }
            AuthEvent::RegisterRequest { username } => {
                match auth_manager.register(username.clone()) {
                    Ok(player) => {
                        let token = AuthToken::new(player.id, 3600);
                        auth_state.login(player.clone(), token);
                        game_log.add(format!(
                            "Registered and logged in as {}",
                            player.display_name
                        ));
                    }
                    Err(e) => {
                        game_log.add(format!("Registration failed: {}", e));
                    }
                }
            }
            AuthEvent::Logout => {
                auth_state.logout();
                game_log.add("Logged out".to_string());
            }
            _ => {}
        }
    }
}

/// Plugin for authentication system
pub struct AuthPlugin;

impl Plugin for AuthPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AuthState>()
            .init_resource::<AuthManager>()
            .add_event::<AuthEvent>()
            .add_systems(Update, auth_event_system);
    }
}
