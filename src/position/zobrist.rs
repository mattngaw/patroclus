//! Implementation of [Zobrist Hashing](https://www.chessprogramming.org/Zobrist_Hashing)

use crate::{
    position::{
        Color,
        Piece,
        Position,
        board::Board,
        castling::Castling,
    },
    bits::{
        File
    }
};

use std::hash::{Hash, Hasher, BuildHasher};

use const_random::const_random;

/// A ZobristHasher factory
pub struct BuildZobristHasher;

impl BuildZobristHasher {
    /// Creates a new `BuildZobristHasher`
    pub fn new() -> Self {
        BuildZobristHasher
    }
}

impl Default for BuildZobristHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl BuildHasher for BuildZobristHasher {
    type Hasher = ZobristHasher;

    fn build_hasher(&self) -> Self::Hasher {
        ZobristHasher::new()
    }
}

impl ZobristHasher {
    /// Creates a new Zobrist hasher
    pub fn new() -> Self {
        ZobristHasher(0)
    }
}

impl Default for ZobristHasher {
    fn default() -> Self {
        Self::new()
    }
}

/// A Zobrist hasher instance
pub struct ZobristHasher(u64);

impl Hasher for ZobristHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        let mut shamt = 0;
        for &byte in bytes {
            self.0 ^= u64::from(byte) << (shamt * 8);
            shamt = (shamt + 1) & 7;
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

impl Board {
    const ZOBRIST_PRNS: [[[u64; 6]; 2]; 64] = {
        let prbs = const_random!([u8; 6144]);
        unsafe {
            std::mem::transmute::<[u8; 6144], [[[u64; 6]; 2]; 64]>(prbs)
        }
    };
}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (s, o_p) in self.iter_pieces() {
            if let Some(Piece(c, r)) = o_p {
                let prn = Self::ZOBRIST_PRNS[usize::from(s)][c as usize][r as usize];
                state.write_u64(prn);
            }
        }
    }
}

impl Color {
    const ZOBRIST_PRN: u64 = const_random!(u64);
}

impl Hash for Color {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if *self == Color::Black { state.write_u64(Self::ZOBRIST_PRN) }
    }
}

impl Castling {
    const ZOBRIST_PRNS: [[u64; 2]; 2] = [[const_random!(u64); 2]; 2];
}

impl Hash for Castling {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (c, cs, cr) in self.iter_rights() {
            let prn = Self::ZOBRIST_PRNS[c as usize][cs as usize];
            if cr { state.write_u64(prn) }
        }
    }
}

impl File {
    const ZOBRIST_PRNS: [u64; 8] = [const_random!(u64); 8];
}

impl Hash for File {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let prn = Self::ZOBRIST_PRNS[*self as usize];
        state.write_u64(prn);
    }
}

