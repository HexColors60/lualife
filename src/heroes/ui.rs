//! Hero UI for displaying hero information and abilities.
//!
//! Shows hero stats, abilities, cooldowns, and level progress.

use bevy::prelude::*;

use super::{Hero, HeroRegistry};

/// Marker for hero panel
#[derive(Component)]
pub struct HeroPanel;

/// Marker for selected hero
#[derive(Resource, Default)]
pub struct SelectedHero(pub Option<Entity>);

/// Plugin for hero UI
pub struct HeroUIPlugin;

impl Plugin for HeroUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedHero>()
            .add_systems(Startup, setup_hero_panel)
            .add_systems(Update, (update_hero_panel, handle_ability_buttons));
    }
}

fn setup_hero_panel(mut commands: Commands) {
    // Hero info panel (initially hidden)
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(60.0),
                right: Val::Px(10.0),
                width: Val::Px(200.0),
                padding: UiRect::all(Val::Px(8.0)),
                row_gap: Val::Px(4.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.9)),
            border_color: BorderColor(Color::srgb(0.4, 0.4, 0.5)),
            ..default()
        },
        HeroPanel,
    )).with_children(|parent| {
        // Hero name and level
        parent.spawn(TextBundle::from_section(
            "No Hero Selected",
            TextStyle {
                font_size: 14.0,
                color: Color::srgb(1.0, 0.9, 0.5),
                ..default()
            },
        ));

        // Stats
        parent.spawn(TextBundle::from_section(
            "",
            TextStyle {
                font_size: 11.0,
                color: Color::srgb(0.8, 0.8, 0.8),
                ..default()
            },
        ));

        // Abilities
        parent.spawn(TextBundle::from_section(
            "Abilities:",
            TextStyle {
                font_size: 11.0,
                color: Color::srgb(0.7, 0.7, 0.9),
                ..default()
            },
        ));
    });
}

fn update_hero_panel(
    selected: Res<SelectedHero>,
    heroes: Query<(Entity, &Hero, &crate::creeps::Creep)>,
    mut panels: Query<&mut Text, With<HeroPanel>>,
    registry: Res<HeroRegistry>,
) {
    if selected.is_changed() || registry.is_changed() {
        for mut text in panels.iter_mut() {
            if let Some(entity) = selected.0 {
                if let Ok((_, hero, creep)) = heroes.get(entity) {
                    let stats = hero.get_scaled_stats();

                    // Update hero info
                    text.sections[0].value = format!(
                        "{} Lv.{} {}",
                        hero.hero_type.name(),
                        hero.level,
                        if creep.faction_id == crate::factions::FactionId(0) { "(Yours)" } else { "" }
                    );

                    text.sections[1].value = format!(
                        "HP: {:.0}/{:.0}\nMana: {:.0}/{:.0}\nATK: {:.0} | DEF: {:.0}\nEXP: {}/{}",
                        creep.hp,
                        stats.max_hp,
                        hero.mana,
                        hero.max_mana,
                        stats.attack,
                        stats.defense,
                        hero.experience,
                        hero.exp_for_next_level()
                    );

                    // Show abilities with cooldowns
                    let abilities: Vec<String> = hero.available_abilities
                        .iter()
                        .map(|a| a.name().to_string())
                        .collect();

                    text.sections[2].value = format!("Abilities: {}", abilities.join(", "));
                }
            } else {
                text.sections[0].value = "No Hero Selected".to_string();
                text.sections[1].value = "Click a hero to view details".to_string();
                text.sections[2].value = "".to_string();
            }
        }
    }
}

fn handle_ability_buttons(
    keyboard: Res<ButtonInput<KeyCode>>,
    selected: Res<SelectedHero>,
    mut heroes: Query<(&mut Hero, &crate::creeps::Creep)>,
    mut events: EventWriter<super::AbilityExecutedEvent>,
    tick: Res<crate::core::TickNumber>,
) {
    if selected.0.is_none() {
        return;
    }

    let entity = selected.0.unwrap();
    if let Ok((mut hero, creep)) = heroes.get_mut(entity) {
        // Q, W, E for abilities 1, 2, 3
        let ability_keys = [
            (KeyCode::KeyQ, 0),
            (KeyCode::KeyW, 1),
            (KeyCode::KeyE, 2),
        ];

        for (key, index) in ability_keys {
            if keyboard.just_pressed(key) {
                if let Some(&ability) = hero.available_abilities.get(index) {
                    if hero.can_use_ability(ability, tick.0) {
                        events.send(super::AbilityExecutedEvent {
                            hero_entity: entity,
                            hero: hero.clone(),
                            faction_id: creep.faction_id,
                            ability,
                            target_pos: None,
                            target_entity: None,
                        });
                    }
                }
            }
        }
    }
}