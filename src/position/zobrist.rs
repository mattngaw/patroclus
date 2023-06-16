//! Implementation of [Zobrist Hashing](https://www.chessprogramming.org/Zobrist_Hashing)

use crate::{
    position::{
        Color,
        Role,
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

impl ZobristHasher {
    /// Creates a new Zobrist hasher
    pub fn new() -> Self {
        ZobristHasher(0)
    }
}

pub struct ZobristHasher(u64);

impl Hasher for ZobristHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        let mut shamt = {
            #[cfg(target_endian="little")] { 0 }
            #[cfg(target_endian="big")] { 7 }
        };
        for &byte in bytes {
            shamt = {
                #[cfg(target_endian="little")]
                { (shamt + 1) & 8 }
                #[cfg(target_endian="big")]
                { (shamt - 1) & 8 }
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

impl Board {
    pub const ZOBRIST_PRNS: [[[u64; 6]; 2]; 64] = {
        let mut _prns = const_random!([u8; 64]);
        let mut prns = [[[0; 6]; 2]; 64];

        let mut i = 0;
        while i < 64 {
            let mut j = 0;
            while j < 2 {
                let mut k = 0;
                while k < 6 {
                    prns[i][j][k] = const_random!(u64);
                    k += 1;
                }
                j += 1;
            }
            i += 1;
        }
        prns
    };
}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (s, o_p) in self.iter_pieces() {
            if let Some(Piece(c, r)) = o_p {
                let prn = Self::ZOBRIST_PRNS[usize::from(s)][c as usize][r as usize];
                state.write_u64(prn);
                log::debug!("board({s}, {:?}): {:#018x}", o_p, state.finish());
            }
        }
        log::debug!("board: {:#018x}", state.finish());
    }
}

impl Color {
    const ZOBRIST_PRN: u64 = const_random!(u64);
}

impl Hash for Color {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if *self == Color::Black { state.write_u64(Self::ZOBRIST_PRN) }
        log::debug!("color: {:#018x}", state.finish());
    }
}

impl Castling {
    const ZOBRIST_PRNS: [[u64; 2]; 2] = [[const_random!(u64); 2]; 2];
}

impl Hash for Castling {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (c, cs, cr) in self.iter_rights() {
            if cr { state.write_u64(Self::ZOBRIST_PRNS[c as usize][cs as usize]) }
        }
        log::debug!("castling: {:#018x}", state.finish());
    }
}

impl File {
    const ZOBRIST_PRNS: [u64; 8] = [const_random!(u64); 8];
}

impl Hash for File {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(Self::ZOBRIST_PRNS[*self as usize]);
        log::debug!("en_passant: {:#018x}", state.finish());
    }
}

