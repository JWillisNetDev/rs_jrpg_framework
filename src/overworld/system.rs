use bevy::prelude::*;

use crate::{
    AppState,
    input::{Input, InputAction},
    overworld::component::{
        Coord, Grid, OverworldPlayer, OverworldPosition, PlayerMoveTimer, Tile,
    },
};

const TILE_SIZE: f32 = 10.;

pub fn deactivate_overworld(mut player: Query<&mut Visibility, With<OverworldPlayer>>) {
    let Ok(mut player_visibility) = player.single_mut() else {
        return;
    };

    *player_visibility = Visibility::Hidden;
}

pub fn activate_overworld(mut player: Query<&mut Visibility, With<OverworldPlayer>>) {
    let Ok(mut player_visibility) = player.single_mut() else {
        return;
    };

    *player_visibility = Visibility::Visible;
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut grid: ResMut<Grid>,
) {
    let mesh = meshes.add(Circle::new(TILE_SIZE));
    let material = materials.add(Color::srgb(1., 1., 1.));
    *grid.get_tile_mut(Coord::ZERO).unwrap() = Tile::Occupied;
    commands.spawn((
        OverworldPlayer,
        OverworldPosition(Coord::ZERO),
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Visibility::Hidden,
    ));
}

pub fn project_grid_positions(mut positionables: Query<(&mut Transform, &OverworldPosition)>) {
    for (mut trans, &OverworldPosition(coord)) in positionables.iter_mut() {
        let world_pos = Vec3::new(
            coord.x as f32 * TILE_SIZE * 2.,
            coord.y as f32 * TILE_SIZE * 2.,
            0.,
        );

        trans.translation = world_pos;
    }
}

pub fn handle_player_movement_input(
    input: Res<Input>,
    time: Res<Time>,
    mut player: Query<&mut OverworldPosition, With<OverworldPlayer>>,
    mut grid: ResMut<Grid>,
    mut move_timer: ResMut<PlayerMoveTimer>,
) {
    let timer = &mut move_timer.0;
    timer.tick(time.delta());

    let Ok(mut player_pos) = player.single_mut() else {
        return;
    };

    if timer.is_finished()
        && input.any_pressed(&[
            InputAction::MoveUp,
            InputAction::MoveDown,
            InputAction::MoveLeft,
            InputAction::MoveRight,
        ])
    {
        if input.pressed(InputAction::MoveUp) {
            try_move_player(&mut player_pos, Coord { x: 0, y: 1 }, &mut grid);
        } else if input.pressed(InputAction::MoveDown) {
            try_move_player(&mut player_pos, Coord { x: 0, y: -1 }, &mut grid);
        } else if input.pressed(InputAction::MoveLeft) {
            try_move_player(&mut player_pos, Coord { x: -1, y: 0 }, &mut grid);
        } else if input.pressed(InputAction::MoveRight) {
            try_move_player(&mut player_pos, Coord { x: 1, y: 0 }, &mut grid);
        }

        timer.reset();
    }
}

pub fn handle_input(input: Res<Input>, mut state: ResMut<NextState<AppState>>) {
    if input.just_pressed(InputAction::Menu) {
        state.set(AppState::MainMenu);
    }
}

fn try_move_player(player_pos: &mut OverworldPosition, translation: Coord, grid: &mut Grid) {
    let target_coord = player_pos.0 + translation;
    if !can_move_player_to(target_coord, grid) {
        return;
    }

    grid.swap(player_pos.0, target_coord);
    player_pos.0 = target_coord;
}

fn can_move_player_to(target: Coord, grid: &Grid) -> bool {
    grid.get_tile(target) == Some(Tile::Empty)
}
