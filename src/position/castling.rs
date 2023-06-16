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

/// The castling state of a chessboard
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Castling {
    rights: [[bool; 2]; 2]
}

impl Default for Castling {
    fn default() -> Self {
        Castling { rights: [[true, true], [true, true]] }
    }
}

impl Castling {
    /// Creates a new `Castling` object
    pub fn new() -> Self {
        Castling { rights: [[false, false], [false, false]] }
    }

    /// Gets the castling rights for color `c`
    pub fn get(self, c: Color, cs: CastlingSide) -> bool {
        self.rights[c as usize][cs as usize]
    }

    /// Sets the castling rights for color `c` to `cr`
    pub fn set(&mut self, c: Color, cs: CastlingSide, cr: bool) {
        self.rights[c as usize][cs as usize] = cr;
    }

    /// Returns an iterator over the castling rights
    pub fn iter_rights(&self) -> std::array::IntoIter<(Color, CastlingSide, bool), 4> {
        [
            (Color::White, CastlingSide::Kingside, self.rights[0][0]),
            (Color::White, CastlingSide::Queenside, self.rights[0][1]),
            (Color::Black, CastlingSide::Kingside, self.rights[1][0]),
            (Color::Black, CastlingSide::Queenside, self.rights[1][1]),
        ].into_iter()
    }
}

impl Display for Castling {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cr_str = String::new();
        if self.rights[0][0] { cr_str.push('K') }
        if self.rights[0][1] { cr_str.push('Q') }
        if self.rights[1][0] { cr_str.push('k') }
        if self.rights[1][1] { cr_str.push('q') }
        if cr_str.is_empty() { cr_str.push('-') }
        write!(f, "{}", cr_str)
    }
}