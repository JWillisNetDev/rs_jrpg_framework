pub mod component;
pub mod state;
pub mod system;
pub mod ui;

use bevy::prelude::*;

use crate::plugin::{
    component::{Grid, PlayerMoveTimer},
    state::{GameState, MainMenuSet, OverworldSet},
};

pub struct RpgPlugin;

impl Plugin for RpgPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerMoveTimer {
            timer: Timer::from_seconds(0.25, TimerMode::Repeating),
        })
        .insert_resource(Grid::new(20, 20))
        .insert_state(GameState::MainMenu)
        .configure_sets(
            OnEnter(GameState::MainMenu),
            MainMenuSet.run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(
            OnEnter(GameState::MainMenu),
            (
                ui::spawn_main_menu,
                ui::startup_selection_menus.after(ui::spawn_main_menu),
            )
                .in_set(MainMenuSet),
        )
        .add_systems(
            Update,
            ui::update_handle_selection_menu_input
                .in_set(MainMenuSet)
                .run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(Startup, system::setup_camera)
        .add_systems(
            Startup,
            (system::spawn_player)
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
