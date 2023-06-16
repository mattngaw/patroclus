use crate::{
    position::{
        Color,
        Position,
        board::Board,
        castling::Castling,
    },
    bits::{
        File
    }
};

use std::hash::{Hash, Hasher, BuildHasher};

struct ZobristHasher(u64);

impl Hasher for ZobristHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        let mut shamt = {
            #[cfg(target_endian="little")]
            {
                0
            }
            #[cfg(target_endian="big")]
            {
                7
            }
        };
        for &byte in bytes {
            shamt = {
                #[cfg(target_endian="little")]
                {
                    (shamt + 1) & 8
                }
                #[cfg(target_endian="big")]
                {
                    (shamt - 1) & 8
                }
            };
            self.0 ^= u64::from(byte) << (shamt * 8);
        }
    }
}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.board.hash(state);
        self.turn.hash(state);
        self.castling.hash(state);
        if let Some(ep_s) = self.en_passant {
            ep_s.file().hash(state);
        }
    }
}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!()
    }
}

impl Hash for Color {
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!()
    }
}

impl Hash for Castling {
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!()
    }
}

impl Hash for File {
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!()
    }
}

