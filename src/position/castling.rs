//! Keeping track of castling rights.

use super::{Color};

/// The direction of castling
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum CastlingSide {
    Kingside,
    Queenside,
}

/// The castling rights that a side has
#[repr(u8)]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum CastlingRights {
    None,
    Kingside,
    Queenside,
    Both,
}

/// The castling state of a chessboard
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Castling(CastlingRights, CastlingRights);

impl Default for Castling {
    fn default() -> Self {
        Castling(CastlingRights::Both, CastlingRights::Both)
    }
}

impl Castling {
    /// Creates a new `Castling` object
    pub fn new() -> Self {
        Castling(CastlingRights::None, CastlingRights::None)
    }

    /// Gets the castling rights for color `c`
    pub fn get(self, c: Color) -> CastlingRights {
        match c {
            Color::White => self.0,
            Color::Black => self.1,
        }
    }

    /// Sets the castling rights for color `c` to `cr`
    pub fn set(&mut self, c: Color, cr: CastlingRights) -> () {
        match c {
            Color::White => self.0 = cr,
            Color::Black => self.1 = cr,
        }
    }

    /// Clears the castling rights for both sides
    pub fn clear(&mut self) -> () {
        (self.0, self.1) = (CastlingRights::None, CastlingRights::None)
    }
}