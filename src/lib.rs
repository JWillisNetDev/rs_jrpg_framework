pub(crate) mod input;
pub(crate) mod ui;

pub mod main_menu;
pub mod overworld;

use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Overworld,
}

pub struct JRPGFrameworkPlugin;

impl Plugin for JRPGFrameworkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(AppState::MainMenu)
            .add_plugins((
                input::InputPlugin,
                overworld::OverworldPlugin,
                main_menu::MainMenuPlugin,
            ))
            .add_systems(Startup, startup_camera);
    }
}

fn startup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
