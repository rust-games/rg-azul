//! Every constants of the crate.

pub const NUMBER_OF_TILES_PER_COLOR: usize = 20;
pub const NUMBER_OF_COLOR: usize = 5;
pub const NUMBER_OF_TILES: usize = NUMBER_OF_TILES_PER_COLOR * NUMBER_OF_COLOR;

/// List of malus per tile index.
pub const MALUS: [i32; 7] = [1, 1, 2, 2, 2, 3, 3];

/// Point per full raw.
pub const POINT_FULL_RAW: u16 = 2;
/// Point per full column.
pub const POINT_FULL_COLUMN: u16 = 7;
/// Point per full color.
pub const POINT_FULL_COLOR: u16 = 10;
