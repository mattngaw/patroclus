//! Convenient declarations and helper functions for the [`position`]() module

#![allow(missing_docs)]

use super::{Color, Role, Piece};

pub const WHITE_PAWN: Piece = Piece(Color::White, Role::Pawn);
pub const WHITE_KNIGHT: Piece = Piece(Color::White, Role::Knight);
pub const WHITE_BISHOP: Piece = Piece(Color::White, Role::Bishop);
pub const WHITE_ROOK: Piece  = Piece(Color::White, Role::Rook);
pub const WHITE_QUEEN: Piece  = Piece(Color::White, Role::Queen);
pub const WHITE_KING: Piece  = Piece(Color::White, Role::King);

pub const BLACK_PAWN: Piece = Piece(Color::Black, Role::Pawn);
pub const BLACK_KNIGHT: Piece = Piece(Color::Black, Role::Knight);
pub const BLACK_BISHOP: Piece = Piece(Color::Black, Role::Bishop);
pub const BLACK_ROOK: Piece  = Piece(Color::Black, Role::Rook);
pub const BLACK_QUEEN: Piece  = Piece(Color::Black, Role::Queen);
pub const BLACK_KING: Piece  = Piece(Color::Black, Role::King);
