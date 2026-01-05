use bevy::prelude::*;

use crate::{
    main_menu::component::MainMenu,
    ui::{SelectionMenu, SelectionMenuItem},
};

pub fn cleanup_main_menu(mut commands: Commands, main_menu: Query<Entity, With<MainMenu>>) {
    let Ok(main_menu) = main_menu.single() else {
        return;
    };

    commands.entity(main_menu).despawn();
}

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
            width: Val::Auto,
            height: Val::Auto,
            padding: UiRect::axes(Val::Px(10.), Val::Px(5.)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
    );

    let selection_menu = (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(10.),
            ..default()
        },
        SelectionMenu::new(vec![
            "Start Game".to_string(),
            "Settings".to_string(),
            "Exit".to_string(),
        ]),
    );

    commands.spawn((
        window,
        children![
            (title_container, children![(title_text)]),
            (
                menu_container,
                children![(menu_panel, children![(selection_menu)])]
            )
        ],
    ));
}

#[allow(clippy::type_complexity)]
pub fn handle_selection_menu(
    mut interactions: Query<
        (&crate::ui::Interaction, &mut TextColor, &SelectionMenuItem),
        (Changed<crate::ui::Interaction>, With<SelectionMenuItem>),
    >,
    mut state: ResMut<NextState<crate::AppState>>,
) {
    for (interaction, mut text_color, &SelectionMenuItem(index)) in interactions.iter_mut() {
        match *interaction {
            crate::ui::Interaction::None => {
                text_color.0 = Color::WHITE;
            }
            crate::ui::Interaction::Hovered => {
                text_color.0 = Color::srgb(5., 5., 0.);
            }
            crate::ui::Interaction::Pressed => {
                text_color.0 = Color::srgb(0.8, 0., 0.);
                if index == 0 {
                    state.set(crate::AppState::Overworld);
                }
            }
        }
    }
}
