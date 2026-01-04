pub mod component;
pub mod state;
pub mod system;

use bevy::prelude::*;

use crate::plugin::{
    component::{Grid, PlayerMoveTimer},
    state::{GameState, OverworldSet},
};

pub struct RpgPlugin;

impl Plugin for RpgPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerMoveTimer {
            timer: Timer::from_seconds(0.25, TimerMode::Repeating),
        })
        .insert_resource(Grid::new(20, 20))
        .insert_state(GameState::MainMenu)
        .add_systems(
            Startup,
            (system::setup_camera, system::spawn_player)
                .in_set(OverworldSet)
                .run_if(in_state(GameState::Overworld)),
        )
        .add_systems(
            Update,
            system::handle_player_input
                .in_set(OverworldSet)
                .run_if(in_state(GameState::Overworld)),
        )
        .add_systems(
            PostUpdate,
            (system::project_positions)
                .in_set(OverworldSet)
                .run_if(in_state(GameState::Overworld)),
        );
    }
}
