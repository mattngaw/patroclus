pub mod magic;
pub mod util;

use crate::{
    bits::{
        Square,
        Bitboard,
    },
    position::{
        Role,
        castling::CastlingSide,
        Position,
    },
    movegen::{
        util::*,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North = 0,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest
}

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
        todo!();
        movelist
    }
}

fn generate_rook_moves(s: Square, blockers: Bitboard) -> Bitboard {
    let mut moves = Bitboard::EMPTY;
    moves
}

fn generate_bishop_moves(s: Square, blockers: Bitboard) -> Bitboard {
    let mut moves = Bitboard::EMPTY;

    let ne_b = Bitboard::new(RAYS[Direction::Northeast as usize][usize::from(s)]);
    for s in ne_b.into_iter().rev() {

    }

    let se_b = Bitboard::new(RAYS[Direction::Northeast as usize][usize::from(s)]);
    for s in se_b.into_iter() {

    }

    let sw_b = Bitboard::new(RAYS[Direction::Northeast as usize][usize::from(s)]);
    for s in sw_b.into_iter() {

    }
    
    let nw_b = Bitboard::new(RAYS[Direction::Northeast as usize][usize::from(s)]);
    for s in nw_b.into_iter() {
        
    }

    moves
}