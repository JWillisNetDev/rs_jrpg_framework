use bevy::prelude::*;

use crate::input::{Input, InputAction};

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Interaction {
    None,
    Hovered,
    Pressed,
}

#[derive(Component, Debug, Clone)]
#[require(Node)]
pub struct SelectionMenuItem(pub usize);

#[derive(Component, Debug, Clone)]
pub struct SelectionMenu {
    pub items: Vec<String>,
    selected_index: usize,
}

impl SelectionMenu {
    pub fn new(items: Vec<String>) -> Self {
        Self {
            items,
            selected_index: 0,
        }
    }
}

pub fn populate_selection_menus(mut commands: Commands, query: Query<(&SelectionMenu, Entity)>) {
    for (menu, entity) in query {
        if menu.items.is_empty() {
            continue;
        }

        let menu_items = menu
            .items
            .iter()
            .enumerate()
            .map(|(idx, label)| {
                let interaction = if idx == menu.selected_index {
                    Interaction::Hovered
                } else {
                    Interaction::None
                };

                commands
                    .spawn((
                        Node {
                            justify_content: JustifyContent::Center,
                            align_content: AlignContent::Center,
                            ..default()
                        },
                        Text::new(label),
                        SelectionMenuItem(idx),
                        interaction,
                    ))
                    .id()
            })
            .collect::<Vec<_>>();
        commands.entity(entity).add_children(&menu_items);
    }
}

pub fn handle_selection_menu_input(
    mut menus: Query<(&mut SelectionMenu, &Children)>,
    mut items: Query<&mut Interaction, With<SelectionMenuItem>>,
    mut commands: Commands,
    input: Res<Input>,
    assets: Res<AssetServer>,
) {
    for (mut menu, children) in menus.iter_mut() {
        let len = menu.items.len();
        if input.just_released(InputAction::Confirm) {
            for (idx, child) in children.iter().enumerate() {
                let Ok(mut interactions) = items.get_mut(child) else {
                    continue;
                };

                if idx == menu.selected_index {
                    *interactions = Interaction::Pressed;
                    break;
                }
            }
            break;
        }

        if input.just_pressed(InputAction::MoveUp) {
            if menu.selected_index == 0 {
                menu.selected_index = len - 1;
            } else {
                menu.selected_index -= 1;
            }
        } else if input.just_pressed(InputAction::MoveDown) {
            menu.selected_index = (menu.selected_index + 1) % len;
        } else {
            continue;
        }

        commands.spawn((
            AudioPlayer::<AudioSource>(assets.load("audio/navigate_menu.ogg")),
            PlaybackSettings::ONCE,
        ));

        for (idx, child) in children.iter().enumerate() {
            let Ok(mut interaction) = items.get_mut(child) else {
                continue;
            };

            if idx == menu.selected_index {
                *interaction = Interaction::Hovered;
            } else {
                *interaction = Interaction::None;
            }
        }
    }
}
