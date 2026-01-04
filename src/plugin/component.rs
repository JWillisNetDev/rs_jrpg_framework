use bevy::{ecs::intern::Internable, math::USizeVec2, prelude::*};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl std::ops::Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl From<Coord> for USizeVec2 {
    fn from(Coord { x, y }: Coord) -> Self {
        USizeVec2 { x, y }
    }
}

impl From<Coord> for (usize, usize) {
    fn from(Coord { x, y }: Coord) -> Self {
        (x, y)
    }
}

#[derive(Component, Copy, Clone, Debug, PartialEq, Eq, Default)]
#[require(Transform)]
pub struct GridPos(pub Coord);

#[derive(Component)]
#[require(GridPos)]
pub struct Player;

#[derive(Resource)]
pub struct PlayerMoveTimer {
    pub timer: Timer,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum Tile {
    #[default]
    Empty,
    Impassable,
    Entity(Entity),
}

#[derive(Resource)]
pub struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![Tile::Empty; width.checked_mul(height).expect("grid size overflow")];
        Self {
            width,
            height,
            tiles,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_tile(&self, coord: impl Into<Coord>) -> Result<&Tile, &'static str> {
        let Coord { x, y } = coord.into();
        if x >= self.width || y >= self.height {
            Err(format!(
                "Coordinate ({}, {}) out of bounds ({}, {})",
                x, y, self.width, self.height
            )
            .as_str()
            .leak())
        } else {
            Ok(&self.tiles[y * self.width + x])
        }
    }

    pub fn get_tile_mut(&mut self, coord: impl Into<Coord>) -> Result<&mut Tile, &'static str> {
        let Coord { x, y } = coord.into();
        if x >= self.width || y >= self.height {
            Err(format!(
                "Coordinate ({}, {}) out of bounds ({}, {})",
                x, y, self.width, self.height
            )
            .as_str()
            .leak())
        } else {
            Ok(&mut self.tiles[y * self.width + x])
        }
    }

    pub fn swap_tiles(&mut self, coord: impl Into<Coord>, other: impl Into<Coord>) {
        let coord = coord.into();
        let other = other.into();
        self.tiles.swap(
            coord.y * self.width + coord.x,
            other.y * self.width + other.x,
        );
    }
}

impl<T: Into<Coord>> std::ops::Index<T> for Grid {
    type Output = Tile;

    fn index(&self, index: T) -> &Self::Output {
        self.get_tile(index.into()).unwrap()
    }
}

impl<T: Into<Coord>> std::ops::IndexMut<T> for Grid {
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        self.get_tile_mut(index.into()).unwrap()
    }
}
