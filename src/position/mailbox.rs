//! A square-centric data structure for keeping track of board state
//! 
//! "Is a square empty or occupied by a piece?"
//! 
//! A Mailbox allows for fast, direct access to the occupancy of a square.

use std::fmt::Display;
use std::ops::{Index, IndexMut};

use super::{Color, Role, Piece, util::*};
use crate::bits::{Square, Flippable};
use crate::util::*;

/// A square-centric data structure 
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Mailbox([Option<Piece>; 64]);

impl Mailbox {
    

    /// Creates a new mailbox
    /// 
    /// All legal chessboards must have a king on both sides. Thus, by default 
    /// the white and black kings are placed on `e1` and `e8`, respectively.
    pub fn new() -> Self {
        let mut array =[None; 64];
        array[4] = Some(Piece(Color::White, Role::King));
        array[60] = Some(Piece(Color::Black, Role::King));
        Mailbox(array)
    }

    /// Creates a mailbox from the placement of pieces
    pub fn from_placement(pm: [Option<Piece>; 64]) -> Self {
        Self(pm)
    }
}

impl Default for Mailbox {
    fn default() -> Self {
        const DEFAULT: [Option<Piece>; 64] = {
            let mut arr = [None; 64];
            
            (arr[0], arr[7]) = (Some(WHITE_ROOK), Some(WHITE_ROOK));
            (arr[1], arr[6]) = (Some(WHITE_KNIGHT), Some(WHITE_KNIGHT));
            (arr[2], arr[5]) = (Some(WHITE_BISHOP), Some(WHITE_BISHOP));
            (arr[3], arr[4]) = (Some(WHITE_QUEEN), Some(WHITE_KING));

            let mut i = 8;
            while i < 16 {
                arr[i] = Some(WHITE_PAWN);
                i += 1;
            }

            (arr[56], arr[63]) = (Some(BLACK_ROOK), Some(BLACK_ROOK));
            (arr[57], arr[62]) = (Some(BLACK_KNIGHT), Some(BLACK_KNIGHT));
            (arr[58], arr[61]) = (Some(BLACK_BISHOP), Some(BLACK_BISHOP));
            (arr[59], arr[60]) = (Some(BLACK_QUEEN), Some(BLACK_KING));

            let mut i = 48;
            while i < 56 {
                arr[i] = Some(BLACK_PAWN);
                i += 1;
            }

            arr
        };
        
        Mailbox(DEFAULT)
    }
}

impl Flippable for Mailbox {
    fn flipped(&self) -> Self {
        let mut flipped = Mailbox::new();
        for i in 0..64usize {
            flipped.0[i] = self.0[63 - i];
        }
        flipped
    }
}

impl IntoIterator for Mailbox {
    type Item = (Square, Option<Piece>);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let v: Vec<Self::Item> = self.0.into_iter()
            .enumerate()
            .map(|(i, p)| {
                (Square::new(i as u32), p)
            }).collect();
        
        v.into_iter()
    }
}

impl Index<Square> for Mailbox {
    type Output = Option<Piece>;

    fn index(&self, s: Square) -> &Self::Output {
        &self.0[usize::from(s)]
    }
}

impl IndexMut<Square> for Mailbox {
    fn index_mut(&mut self, s: Square) -> &mut Self::Output {
        &mut self.0[usize::from(s)]
    }
}

impl Display for Mailbox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut b_chars = vec!['.'; 64];
        for (s, op) in *self {
            if let Some(p) = op {
                b_chars[usize::from(s)] = char::from(p); 
            }
        }
        let mut b_str = String::new();
        for i in PRINT_ORDER {
            for j in i {
                b_str.push(b_chars[j]);
                b_str.push(' ');
            }
            b_str.push('\n');
        }
        write!(f, "{}", b_str)
    }
}
