//! Struct for keeping track of what's on each square.

use std::ops::{Index, IndexMut};

use super::{Color, Role, Piece}; 
use crate::bits::{Square, Flippable};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Mailbox([Option<Piece>; 64]);

impl Mailbox {
    /// Creates a new mailbox
    /// 
    /// All legal chessboards must have a king on both sides. Thus, by default 
    /// the white and black kings are placed on `e1` and `e8`, respectively.
    pub fn new() -> Mailbox {
        let mut array =[None; 64];
        array[4] = Some(Piece(Color::White, Role::King));
        array[60] = Some(Piece(Color::Black, Role::King));
        Mailbox(array)
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
