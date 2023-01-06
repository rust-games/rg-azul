use crate::constant::NUMBER_OF_COLOR;
use bevy::prelude::*;

/// Generic structure of position.
#[derive(Debug, Component)]
pub struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Component)]
pub struct TilePosition {
    player: Player,
    position: Position,
}

#[derive(Debug, Component, Resource)]
pub struct PlayerInput {
    pub mouse_click: Option<(f32, f32)>,
}

#[derive(Debug, Component)]
pub struct Player {
    pub name: String,
    pub score: u16,
}

#[derive(Debug, Resource)]
pub struct CurrentPlayer {
    pub player: Player,
}

#[derive(Debug, Component)]
pub enum TileColor {
    Blue,
    Yellow,
    Red,
    Black,
    White,
}

#[derive(Debug, Component)]
pub enum TileRegion {
    Pool,
    MiddleGroup,
    Middle,
    BuildingBoard(usize), // With line index.
    Board(Position),      // With the position of the tile in the board.
    Malus(usize),         // With malus index.
}

/// Set of [`PlayerBoard`] and set of common [`Tile`].
#[derive(Debug, Resource)]
pub struct GlobalBoard {
    player_board: PlayerBoard,
    group_of_tiles: std::vec<std::vec<Tile>>,
    group_of_middle_tiles: std::vec<Tile>,
}

/// Set of Malus, tiles building (side) and [`Board`].
pub struct PlayerBoard {
    board: Board,
    tiles_building: [(usize, Color); NUMBER_OF_COLOR],
}

/// Board of all the tiles built.
pub struct Board {
    board: [[Option<Tile>; NUMBER_OF_COLOR]; NUMBER_OF_COLOR],
}
