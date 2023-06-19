mod util;

use crate::{
    bits::{
        Square,
    },
    position::{
        Role,
        castling::CastlingSide,
        Position,
    }
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Move {
    PawnMove {
        from: Square,
        to: Square,
        promotion: Option<Role>,
        en_passant: bool,
        capture: Option<Role>,
    },
    DoublePawnPush {
        from: Square,
        to: Square,
    },
    Normal {
        role: Role,
        from: Square,
        to: Square,
        capture: Option<Role>,
    },
    Castle {
        castling_side: CastlingSide,
    }

}

impl Position {
    pub fn generate(&self) -> Vec<Move> {
        let movelist = Vec::new();
        movelist
    }
}