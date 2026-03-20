use bevy::audio::Volume;
use bevy::prelude::*;
use std::collections::HashMap;

/// Audio settings resource
#[derive(Resource, Debug, Clone)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub ambient_volume: f32,
    pub muted: bool,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            music_volume: 0.5,
            sfx_volume: 0.7,
            ambient_volume: 0.3,
            muted: false,
        }
    }
}

impl AudioSettings {
    pub fn effective_music_volume(&self) -> f32 {
        if self.muted {
            0.0
        } else {
            self.master_volume * self.music_volume
        }
    }

    pub fn effective_sfx_volume(&self) -> f32 {
        if self.muted {
            0.0
        } else {
            self.master_volume * self.sfx_volume
        }
    }

    pub fn effective_ambient_volume(&self) -> f32 {
        if self.muted {
            0.0
        } else {
            self.master_volume * self.ambient_volume
        }
    }
}

/// Sound effect identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SfxId {
    // UI sounds
    Click,
    Select,
    Deselect,
    Hover,
    OpenPanel,
    ClosePanel,

    // Game sounds
    Mining,
    Building,
    Combat,
    Death,
    Spawn,
    LevelUp,

    // Ambient
    Wind,
    Water,
    Forest,
}

/// Background music track identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MusicId {
    Menu,
    Game,
    Combat,
    Victory,
    Defeat,
}

/// Marker for background music entity
#[derive(Component)]
pub struct BackgroundMusic {
    pub track: MusicId,
}

/// Marker for ambient sound entity
#[derive(Component)]
pub struct AmbientSound {
    pub sound: SfxId,
}

/// Audio state resource
#[derive(Resource, Debug, Clone, Default)]
pub struct AudioState {
    pub current_music: Option<MusicId>,
    pub playing_ambient: HashMap<SfxId, Entity>,
}

/// Event to play a sound effect
#[derive(Event, Debug, Clone)]
pub struct PlaySfx {
    pub id: SfxId,
}

/// Event to change background music
#[derive(Event, Debug, Clone)]
pub struct ChangeMusic {
    pub id: MusicId,
}

/// System to handle sound effect playback
pub fn sfx_system(
    mut events: EventReader<PlaySfx>,
    settings: Res<AudioSettings>,
    mut commands: Commands,
) {
    for event in events.read() {
        let volume = Volume::new(settings.effective_sfx_volume());

        // Play the sound effect
        // Note: In a real implementation, you would load audio files
        // For now, we just log the sound
        match event.id {
            SfxId::Click | SfxId::Select | SfxId::Deselect | SfxId::Hover => {
                // UI sounds - short, immediate feedback
            }
            SfxId::Mining | SfxId::Building | SfxId::Combat => {
                // Game action sounds
            }
            SfxId::Death | SfxId::Spawn | SfxId::LevelUp => {
                // Important game events
            }
            SfxId::Wind | SfxId::Water | SfxId::Forest => {
                // Ambient sounds - looped
            }
            SfxId::OpenPanel | SfxId::ClosePanel => {
                // UI panel sounds
            }
        }
    }
}

/// System to handle background music changes
pub fn music_system(
    mut events: EventReader<ChangeMusic>,
    settings: Res<AudioSettings>,
    mut state: ResMut<AudioState>,
    mut commands: Commands,
    query: Query<Entity, With<BackgroundMusic>>,
) {
    for event in events.read() {
        // Skip if already playing this track
        if state.current_music == Some(event.id) {
            continue;
        }

        // Stop current music
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }

        // Start new music
        let volume = Volume::new(settings.effective_music_volume());

        // Note: In a real implementation, you would load audio files
        // commands.spawn((
        //     AudioBundle {
        //         source: asset_server.load(match event.id {
        //             MusicId::Menu => "audio/music/menu.ogg",
        //             MusicId::Game => "audio/music/game.ogg",
        //             MusicId::Combat => "audio/music/combat.ogg",
        //             MusicId::Victory => "audio/music/victory.ogg",
        //             MusicId::Defeat => "audio/music/defeat.ogg",
        //         }),
        //         settings: PlaybackSettings {
        //             mode: PlaybackMode::Loop,
        //             volume,
        //             ..default()
        //         },
        //     },
        //     BackgroundMusic { track: event.id },
        // ));

        state.current_music = Some(event.id);
    }
}

/// System to update audio volumes when settings change
pub fn audio_settings_system(
    settings: Res<AudioSettings>,
    mut query: Query<&mut AudioSink, With<BackgroundMusic>>,
) {
    if settings.is_changed() {
        for mut sink in query.iter_mut() {
            sink.set_volume(settings.effective_music_volume());
        }
    }
}

/// System to handle keyboard shortcuts for audio
pub fn audio_keyboard_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<AudioSettings>,
) {
    // Mute/unmute with M key (when not used for minimap)
    if keyboard.just_pressed(KeyCode::NumpadMultiply) {
        settings.muted = !settings.muted;
    }

    // Volume controls with +/- keys
    if keyboard.just_pressed(KeyCode::Equal) || keyboard.just_pressed(KeyCode::NumpadAdd) {
        settings.master_volume = (settings.master_volume + 0.1).min(1.0);
    }
    if keyboard.just_pressed(KeyCode::Minus) || keyboard.just_pressed(KeyCode::NumpadSubtract) {
        settings.master_volume = (settings.master_volume - 0.1).max(0.0);
    }
}

/// Plugin for audio system
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioSettings>()
            .init_resource::<AudioState>()
            .add_event::<PlaySfx>()
            .add_event::<ChangeMusic>()
            .add_systems(
                Update,
                (
                    sfx_system,
                    music_system,
                    audio_settings_system,
                    audio_keyboard_system,
                ),
            );
    }
}
