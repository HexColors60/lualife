//! Construction progress bar visualization.

use bevy::prelude::*;

use crate::buildings::ConstructionSite;

/// Marker for construction progress bar background
#[derive(Component)]
pub struct ConstructionProgressBarBg;

/// Marker for construction progress bar fill
#[derive(Component)]
pub struct ConstructionProgressBarFill {
    /// The construction site entity this bar is attached to
    pub site_entity: Entity,
}

/// Spawn progress bars for construction sites
pub fn spawn_construction_progress_bars(
    mut commands: Commands,
    sites: Query<(Entity, &ConstructionSite, &Transform), Added<ConstructionSite>>,
) {
    for (entity, site, transform) in sites.iter() {
        let bar_width = 30.0;
        let bar_height = 4.0;
        let offset_y = 20.0;

        // Background bar (dark)
        let bg_entity = commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.2, 0.2, 0.2, 0.8),
                    custom_size: Some(Vec2::new(bar_width, bar_height)),
                    ..default()
                },
                transform: Transform::from_translation(
                    transform.translation + Vec3::new(0.0, offset_y, 2.0)
                ),
                ..default()
            },
            ConstructionProgressBarBg,
        )).id();

        // Fill bar (green/yellow based on progress)
        let fill_width = bar_width * site.progress_ratio();
        let fill_color = if site.progress_ratio() < 0.5 {
            Color::srgba(0.8, 0.8, 0.2, 0.9) // Yellow for early progress
        } else {
            Color::srgba(0.2, 0.8, 0.2, 0.9) // Green for later progress
        };

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: fill_color,
                    custom_size: Some(Vec2::new(fill_width, bar_height - 1.0)),
                    ..default()
                },
                transform: Transform::from_translation(
                    transform.translation + Vec3::new(-bar_width / 2.0 + fill_width / 2.0, offset_y, 2.1)
                ),
                ..default()
            },
            ConstructionProgressBarFill { site_entity: entity },
        ));
    }
}

/// Update progress bar fills based on construction progress
pub fn update_construction_progress_bars(
    sites: Query<&ConstructionSite, Changed<ConstructionSite>>,
    mut bars: Query<(&mut ConstructionProgressBarFill, &mut Sprite, &mut Transform)>,
) {
    for (fill, mut sprite, mut transform) in bars.iter_mut() {
        if let Ok(site) = sites.get(fill.site_entity) {
            let bar_width = 30.0;
            let bar_height = 4.0;
            let ratio = site.progress_ratio();
            let fill_width = bar_width * ratio;

            // Update fill size
            sprite.custom_size = Some(Vec2::new(fill_width, bar_height - 1.0));

            // Update position (centered based on fill width)
            transform.translation.x = -bar_width / 2.0 + fill_width / 2.0 + transform.translation.x.floor();

            // Update color based on progress
            sprite.color = if ratio < 0.5 {
                Color::srgba(0.8, 0.8, 0.2, 0.9)
            } else if ratio < 0.9 {
                Color::srgba(0.2, 0.8, 0.2, 0.9)
            } else {
                Color::srgba(0.2, 1.0, 0.4, 1.0) // Bright green when nearly done
            };
        }
    }
}

/// Plugin for construction progress visualization
pub struct ConstructionProgressPlugin;

impl Plugin for ConstructionProgressPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_construction_progress_bars, update_construction_progress_bars));
    }
}