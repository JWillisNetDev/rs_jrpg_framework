use std::{collections::HashMap, sync::RwLock};

use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Input {
    enabled: bool,
    actions: RwLock<HashMap<InputAction, Vec<KeyCode>>>,
    btn_input: ButtonInput<KeyCode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Confirm,
    Menu,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Input::default())
            .add_systems(
                Startup,
                (setup_input, register_default_actions.after(setup_input)),
            )
            .add_systems(PreUpdate, handle_input);
    }
}

fn setup_input(mut commands: Commands) {
    commands.insert_resource(Input {
        enabled: true,
        ..default()
    });
}

fn register_default_actions(input: ResMut<Input>) {
    let mut actions = input.actions.write().unwrap();
    actions.insert(InputAction::MoveUp, vec![KeyCode::KeyW, KeyCode::ArrowUp]);
    actions.insert(InputAction::Menu, vec![KeyCode::Escape]);
    actions.insert(
        InputAction::MoveDown,
        vec![KeyCode::KeyS, KeyCode::ArrowDown],
    );
    actions.insert(
        InputAction::MoveLeft,
        vec![KeyCode::KeyA, KeyCode::ArrowLeft],
    );
    actions.insert(
        InputAction::MoveRight,
        vec![KeyCode::KeyD, KeyCode::ArrowRight],
    );
    actions.insert(
        InputAction::Confirm,
        vec![KeyCode::Space, KeyCode::Enter, KeyCode::KeyZ],
    );
}

fn handle_input(mut input: ResMut<Input>, keys: Res<ButtonInput<KeyCode>>) {
    if !input.enabled {
        return;
    }

    input.handle_input(keys);
}

impl Input {
    pub fn pressed(&self, action: InputAction) -> bool {
        if let Some(keys) = self.actions.read().unwrap().get(&action) {
            self.btn_input.any_pressed(keys.to_owned())
        } else {
            false
        }
    }

    pub fn released(&self, action: InputAction) -> bool {
        if let Some(keys) = self.actions.read().unwrap().get(&action) {
            !self.btn_input.any_pressed(keys.to_owned())
        } else {
            false
        }
    }

    pub fn just_pressed(&self, action: InputAction) -> bool {
        if let Some(keys) = self.actions.read().unwrap().get(&action) {
            self.btn_input.any_just_pressed(keys.to_owned())
        } else {
            false
        }
    }

    pub fn just_released(&self, action: InputAction) -> bool {
        if let Some(keys) = self.actions.read().unwrap().get(&action) {
            self.btn_input.any_just_released(keys.to_owned())
        } else {
            false
        }
    }

    pub fn any_pressed(&self, actions: &[InputAction]) -> bool {
        let keys = self
            .actions
            .read()
            .unwrap()
            .iter()
            .filter_map(|(action, keys)| {
                if actions.contains(action) {
                    Some(keys.to_owned())
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<_>>();
        self.btn_input.any_pressed(keys)
    }

    pub fn all_released(&self, actions: &[InputAction]) -> bool {
        let keys = self
            .actions
            .read()
            .unwrap()
            .iter()
            .filter_map(|(action, keys)| {
                if actions.contains(action) {
                    Some(keys.to_owned())
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<_>>();

        !self.btn_input.any_pressed(keys)
    }

    pub(crate) fn handle_input(&mut self, keys: Res<ButtonInput<KeyCode>>) {
        if !self.enabled {
            return;
        }

        self.btn_input = keys.clone();
    }
}
