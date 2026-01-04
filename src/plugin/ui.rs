use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
#[require(Node)]
pub struct SelectionMenuItem;

#[derive(Component, Debug, Clone, Default)]
#[require(Node)]
pub struct SelectionMenu {
    pub selected_index: usize,
    pub items: Vec<String>,
}

/// Marker component for the main menu UI
#[derive(Component, Debug, Clone, Copy)]
pub struct MainMenu;

pub fn spawn_main_menu(mut commands: Commands) {
    let window = (
        MainMenu,
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            ..default()
        },
    );

    let title_container = Node {
        width: Val::Percent(100.),
        height: Val::Percent(50.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let title_text = (
        Node::default(),
        Text::new("Hello, World!"),
        TextFont {
            font_size: 64.,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(Justify::Center),
    );

    let menu_container = Node {
        width: Val::Percent(100.),
        height: Val::Percent(50.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let menu_panel = (
        Node {
            width: Val::Percent(33.),
            height: Val::Percent(66.),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
    );

    commands.spawn((
        window,
        children![
            (title_container, children![(title_text)]),
            (
                menu_container,
                children![(
                    menu_panel,
                    children![(
                        Node {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(6.),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        SelectionMenu {
                            items: vec!["Start Game".into(), "Options".into(), "Exit".into()],
                            ..default()
                        }
                    )]
                )]
            ),
        ],
    ));
}

pub fn startup_selection_menus(mut commands: Commands, query: Query<(&SelectionMenu, Entity)>) {
    for (menu, ent) in query {
        if menu.items.is_empty() {
            continue;
        }

        let menu_items = menu
            .items
            .clone()
            .into_iter()
            .enumerate()
            .map(|(idx, label)| {
                let text_color = if idx == menu.selected_index {
                    Color::srgb(5., 5., 0.)
                } else {
                    Color::WHITE
                };

                commands
                    .spawn((
                        Node {
                            justify_content: JustifyContent::Center,
                            align_content: AlignContent::Center,
                            ..default()
                        },
                        Text::new(label),
                        TextColor(text_color),
                        SelectionMenuItem,
                    ))
                    .id()
            })
            .collect::<Vec<Entity>>();
        commands.entity(ent).add_children(&menu_items);
    }
}

pub fn update_handle_selection_menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut menus: Query<(&mut SelectionMenu, &Children)>,
    mut items: Query<&mut TextColor, With<SelectionMenuItem>>,
) {
    for (mut menu, children_items) in menus.iter_mut() {
        let len = menu.items.len();
        if keys.just_pressed(KeyCode::ArrowUp) {
            if menu.selected_index == 0 {
                menu.selected_index = len - 1;
            } else {
                menu.selected_index -= 1;
            }
        } else if keys.just_pressed(KeyCode::ArrowDown) {
            menu.selected_index = (menu.selected_index + 1) % len;
        } else {
            continue;
        }

        for (idx, child) in children_items.iter().enumerate() {
            let Ok(mut item_color) = items.get_mut(child) else {
                continue;
            };

            if idx == menu.selected_index {
                item_color.0 = Color::srgb(5., 5., 0.);
            } else {
                item_color.0 = Color::WHITE;
            }
        }
    }
}
