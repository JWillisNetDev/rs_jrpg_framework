pub mod component;
pub mod system;

use bevy::prelude::*;

use crate::{
    AppState,
    ui::{self, handle_selection_menu_input},
};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MainMenuSet;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, MainMenuSet.run_if(in_state(AppState::MainMenu)))
            .add_systems(
                OnEnter(AppState::MainMenu),
                (system::spawn_main_menu, ui::populate_selection_menus)
                    .chain()
                    .in_set(MainMenuSet),
            )
            .add_systems(
                OnExit(AppState::MainMenu),
                system::cleanup_main_menu.in_set(MainMenuSet),
            )
            .add_systems(
                Update,
                (
                    ui::handle_selection_menu_input,
                    system::handle_selection_menu.after(handle_selection_menu_input),
                )
                    .in_set(MainMenuSet),
            );
    }
}
