use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub const ZERO: Self = Self { x: 0, y: 0 };
}

impl std::ops::Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl From<(i32, i32)> for Coord {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl From<Vec2> for Coord {
    fn from(Vec2 { x, y }: Vec2) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bounds {
    pub min: Coord,
    pub max: Coord,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tile {
    #[default]
    Empty,
    Impassable,
    Occupied,
}

#[derive(Resource, Debug, Clone)]
pub struct Grid {
    pub width: u32,
    pub height: u32,
    tiles: Vec<Tile>,
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            tiles: vec![Tile::Empty; (width * height) as usize],
        }
    }

    pub fn get_tile(&self, coord: Coord) -> Option<Tile> {
        if coord.x.unsigned_abs() >= self.width / 2 || coord.y.unsigned_abs() >= self.height / 2 {
            None
        } else {
            Some(self.tiles[self.coord_to_index(coord)])
        }
    }

    pub fn get_tile_mut(&mut self, coord: Coord) -> Option<&mut Tile> {
        if coord.x.unsigned_abs() >= self.width / 2 || coord.y.unsigned_abs() >= self.height / 2 {
            None
        } else {
            let idx = self.coord_to_index(coord);
            Some(&mut self.tiles[idx])
        }
    }

    pub(crate) fn swap(&mut self, a: Coord, b: Coord) {
        let idx_a = self.coord_to_index(a);
        let idx_b = self.coord_to_index(b);

        self.tiles.swap(idx_a, idx_b);
    }

    pub(crate) fn clear(&mut self) {
        self.tiles.fill(Tile::Empty);
    }

    pub(crate) fn origin(&self) -> Coord {
        Coord {
            x: self.width as i32 / 2,
            y: self.height as i32 / 2,
        }
    }

    fn coord_to_index(&self, Coord { x, y }: Coord) -> usize {
        let origin = self.origin();
        let x = (x + origin.x) as usize;
        let y = (y + origin.y) as usize;

        y * self.width as usize + x
    }
}

#[derive(Component, Debug, Clone, Copy, Default)]
#[require(Transform)]
pub struct OverworldPosition(pub Coord);

#[derive(Component, Debug, Clone)]
#[require(OverworldPosition)]
pub struct OverworldPlayer;

#[derive(Resource, Debug)]
pub struct PlayerMoveTimer(pub Timer);
