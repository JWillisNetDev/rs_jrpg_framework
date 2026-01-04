use bevy::prelude::*;

use crate::plugin::component::{Coord, Grid, GridPos, Player, PlayerMoveTimer, Tile};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

const PLAYER_SIZE: f32 = 10.;

pub fn spawn_player(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    grid: ResMut<Grid>,
) {
    make_player(commands, meshes, materials, grid);
}

fn make_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut grid: ResMut<Grid>,
) -> Entity {
    let mesh = meshes.add(Circle::new(PLAYER_SIZE));
    let material = materials.add(Color::srgb(1., 0., 0.));
    let grid = &mut grid[Coord { x: 0, y: 0 }];
    let entity = commands
        .spawn((
            Player,
            GridPos::default(),
            Mesh2d(mesh),
            MeshMaterial2d(material),
        ))
        .id();
    *grid = Tile::Entity(entity);
    entity
}

pub fn project_positions(positionables: Query<(&mut Transform, &GridPos)>) {
    for (mut trans, &GridPos(Coord { x, y })) in positionables {
        let new_pos = Vec3::new(
            (x as f32) * PLAYER_SIZE * 2.,
            (y as f32) * PLAYER_SIZE * 2.,
            0.,
        );
        trans.translation = new_pos;
    }
}

pub fn handle_player_input(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut timer: ResMut<PlayerMoveTimer>,
    mut query: Query<&mut GridPos, With<Player>>,
    grid: Res<Grid>,
) {
    let timer = &mut timer.timer;
    timer.tick(time.delta());

    if (timer.just_finished() || timer.is_paused())
        && keys.any_pressed([KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD])
    {
        let Ok(mut player_pos) = query.single_mut() else {
            return;
        };

        timer.unpause();

        let mov_vec: (i32, i32) = if keys.pressed(KeyCode::KeyW) {
            (0, 1)
        } else if keys.pressed(KeyCode::KeyS) {
            (0, -1)
        } else if keys.pressed(KeyCode::KeyA) {
            (-1, 0)
        } else if keys.pressed(KeyCode::KeyD) {
            (1, 0)
        } else {
            unreachable!()
        };

        let move_to_pos = Coord {
            x: (mov_vec.0 + player_pos.0.x as i32) as usize,
            y: (mov_vec.1 + player_pos.0.y as i32) as usize,
        };

        if can_move_player_to(&grid, move_to_pos) {
            player_pos.0 = move_to_pos;
        }

        return;
    } else if timer.just_finished()
        && !keys.any_pressed([KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD])
    {
        timer.pause();
    }
}

fn can_move_player_to(grid: &Grid, coord: Coord) -> bool {
    grid.get_tile(coord).is_ok_and(|&tile| tile == Tile::Empty)
}
