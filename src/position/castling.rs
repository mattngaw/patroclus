//! Keeping track of castling rights.

// TODO Needs a rework to make it more bit-friendly
// It's mainly CastlingRights::Both that is tough to use

use std::fmt::Display;

use super::{Color};

/// The direction of castling
#[allow(missing_docs)]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum CastlingSide {
    Kingside,
    Queenside,
}

/// The castling rights that a side has
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct CastlingRights(bool, bool);

impl CastlingRights {
    /// Creates a new `CastlingRights`
    pub fn new(ks: bool, qs: bool) -> Self {
        CastlingRights(ks, qs)
    }

    /// Returns `true` if castling on side `cs` is still possible
    pub fn can_castle(&self, cs: CastlingSide) -> bool {
        match cs {
            CastlingSide::Kingside => self.0,
            CastlingSide::Queenside => self.1,
        }
    }
}

/// The castling state of a chessboard
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Castling(CastlingRights, CastlingRights);

impl Default for Castling {
    fn default() -> Self {
        Castling(CastlingRights(true, true), CastlingRights(true, true))
    }
}

impl Castling {
    /// Creates a new `Castling` object
    pub fn new() -> Self {
        Castling(CastlingRights(false, false), CastlingRights(false, false))
    }

    /// Gets the castling rights for color `c`
    pub fn get(self, c: Color) -> CastlingRights {
        match c {
            Color::White => self.0,
            Color::Black => self.1,
        }
    }

    /// Sets the castling rights for color `c` to `cr`
    pub fn set(&mut self, c: Color, cr: CastlingRights) {
        match c {
            Color::White => self.0 = cr,
            Color::Black => self.1 = cr,
        }
    }

    /// Removes castling rights from a color `c` on the side `cs`
    pub fn remove(&mut self, c: Color, cs: CastlingSide) {
        let rights = &mut match (c, cs) {
            (Color::White, CastlingSide::Kingside) => self.0.0,
            (Color::White, CastlingSide::Queenside) => self.0.1,
            (Color::Black, CastlingSide::Kingside) => self.1.0,
            (Color::Black, CastlingSide::Queenside) => self.1.1,
        };
        *rights = false;
    }

    /// Clears the castling rights for both sides
    pub fn clear(&mut self) {
        (self.0, self.1) = (
            CastlingRights(false, false), CastlingRights(false, false)
        )
    }
}

impl Display for Castling {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cr_str = String::new();
        if self.0.0 { cr_str.push('K') }
        if self.0.1 { cr_str.push('Q') }
        if self.1.0 { cr_str.push('k') }
        if self.1.1 { cr_str.push('q') }
        if cr_str.is_empty() { cr_str.push('-') }
        write!(f, "{}", cr_str)
    }
}