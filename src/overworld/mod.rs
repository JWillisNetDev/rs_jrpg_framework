pub mod component;
pub mod system;

use bevy::prelude::*;

use crate::{
    AppState,
    overworld::component::{Grid, PlayerMoveTimer},
};

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OverworldSet;

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Grid::new(20, 20))
            .insert_resource(PlayerMoveTimer(Timer::from_seconds(0.3, TimerMode::Once)))
            .configure_sets(Update, OverworldSet.run_if(in_state(AppState::Overworld)))
            .add_systems(Startup, system::spawn_player.in_set(OverworldSet))
            .add_systems(
                OnEnter(AppState::Overworld),
                system::activate_overworld.in_set(OverworldSet),
            )
            .add_systems(
                OnExit(AppState::Overworld),
                system::deactivate_overworld.in_set(OverworldSet),
            )
            .add_systems(
                Update,
                (
                    system::handle_input,
                    system::handle_player_movement_input,
                    system::project_grid_positions,
                )
                    .chain()
                    .in_set(OverworldSet),
            );
    }
}
