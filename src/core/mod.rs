mod game_state;
mod rng;
mod scheduler;
mod sim_clock;
mod tick;

pub use game_state::*;
pub use rng::*;
pub use scheduler::*;
pub use sim_clock::*;
pub use tick::*;

use bevy::prelude::*;

use crate::config::GameConfig;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TickNumber>()
            .init_resource::<SimClock>()
            .init_resource::<GameState>()
            .init_resource::<GameRng>()
            .init_resource::<Scheduler>();

        app.add_systems(Startup, setup_core);
        app.add_systems(Update, update_tick.run_if(resource_exists::<Time>));
    }
}

fn setup_core(mut commands: Commands, config: Res<GameConfig>) {
    tracing::info!("Core systems initialized");
    if config.start_paused {
        commands.insert_resource(GameState::Paused);
    }
}

fn update_tick(
    mut tick: ResMut<TickNumber>,
    mut sim_clock: ResMut<SimClock>,
    game_state: Res<GameState>,
    time: Res<Time>,
) {
    if *game_state == GameState::Running {
        sim_clock.accumulator += time.delta_seconds();

        let tick_duration = sim_clock.tick_duration();
        while sim_clock.accumulator >= tick_duration {
            sim_clock.accumulator -= tick_duration;
            tick.0 += 1;
        }
    }
}