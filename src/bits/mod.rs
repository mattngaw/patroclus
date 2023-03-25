//! Structs for working with and keeping track of squares.

use std::ops::{BitOr, BitAnd, BitXor, Not};
use std::ops::{BitOrAssign, BitAndAssign, BitXorAssign};
use std::fmt::{Display, Debug};

use crate::util::PRINT_ORDER;

/// Used to find the flipped representation of a value (from black's POV)
pub trait Flippable 
where Self: Copy + Sized {
    fn flipped(&self) -> Self;

    fn flip(&mut self) {
        *self = self.flipped()
    }
}


/// An index on a chessboard
/// 
/// A [`Square`] indexes into a [`Board`] or [`Bitboard`] via rank-major order.
/// ```text
/// 8   56 57 58 59 60 61 62 63
/// 7   48 49 50 51 52 53 54 55
/// 6   40 41 42 43 44 45 46 47
/// 5   32 33 34 35 36 37 38 39
/// 4   24 25 26 27 28 29 30 31
/// 3   16 17 18 19 20 21 22 23
/// 2    8  9 10 11 12 13 14 15
/// 1    0  1  2  3  4  5  6  7
/// 
///      a  b  c  d  e  f  g  h
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Square(u32);

impl Square {
    pub const COUNT: u32 = 64;

    pub fn new(value: u32) -> Self {
        debug_assert!(value < 64);
        Square(value)
    }
}

impl From<Coords> for Square {
    fn from(Coords(r, f): Coords) -> Self {
        Square::new(r as u32 * 8 + f as u32)
    }
}

impl Flippable for Square {
    fn flipped(&self) -> Self {
        Square(63 - self.0)
    }
}

impl From<Square> for usize {
    fn from(s: Square) -> Self {
        s.0 as usize
    }
}


/// A row on a chessboard
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Rank {
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
    Fifth = 4,
    Sixth = 5,
    Seventh = 6,
    Eighth = 7,
}


/// A column on a chessboard
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}


/// The coordinates of a square on a chessboard
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Coords(Rank, File);


/// An occupancy set for a chessboard
/// 
/// A bitboard keeps track of which squares are occupied. They 
/// can be used to describe where pieces are, possible moves, or attack squares 
/// for a piece. 
/// 
/// e.g. Some bitboards from the starting position of a chess game:
///
/// ```text
/// 8  . x . . . . x .        8  . . . . . . . .         8  . . . . . . . .
/// 7  . . . . . . . .        7  . . . . . . . .         7  . . . . . . . .
/// 6  . . . . . . . .        6  . . . . . . . .         6  . . . . . . . .
/// 5  . . . . . . . .        5  . . . . . . . .         5  . . . . . . . .
/// 4  . . . . . . . .        4  . . . . . . . .         4  . . . . . . . .
/// 3  . . . . . . . .        3  . . . . . . . .         3  . . . . . . . .
/// 2  . . . . . . . .        2  x x x x x x x x         2  . . . . . . . .
/// 1  . x . . . . x .        1  x x x x x x x x         1  . x . . . . x .
/// 
///    a b c d e f g h           a b c d e f g h            a b c d e f g h
/// 
/// bitboard of knights     bitboard of white pieces   bitboard of white knights
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Bitboard(u64);


impl Bitboard {
    const RANK_MASKS: [u64; 8] = [
        0xFF00_0000_0000_0000,
        0x00FF_0000_0000_0000,
        0x0000_FF00_0000_0000,
        0x0000_00FF_0000_0000,
        0x0000_0000_FF00_0000,
        0x0000_0000_00FF_0000,
        0x0000_0000_0000_FF00,
        0x0000_0000_0000_00FF,
    ];

    const FILE_MASKS: [u64; 8] = [
        0x0101_0101_0101_0101,
        0x0202_0202_0202_0202,
        0x0404_0404_0404_0404,
        0x0808_0808_0808_0808,
        0x1010_1010_1010_1010,
        0x2020_2020_2020_2020,
        0x4040_4040_4040_4040,
        0x8080_8080_8080_8080,
    ];
}


/// **Creation methods**
impl Bitboard {

     #[inline]
    pub fn new(value: u64) -> Self {
        Bitboard(value)
    }

    #[inline]
    pub fn empty() -> Self {
        Bitboard(0u64)
    }

    #[inline]
    pub fn full() -> Self {
        Bitboard(!0u64)
    }

    #[inline]
    pub fn rank(r: Rank) -> Self {
        Bitboard(Self::RANK_MASKS[r as usize])
    }

    #[inline]
    pub fn file(f: File) -> Self {
        Bitboard(Self::FILE_MASKS[f as usize])
    }

    #[inline]
    pub fn square(s: Square) -> Self {
        Bitboard(1u64 << (s.0 as u64))
    }

    #[inline]
    pub fn coords(c: Coords) -> Self {
        Bitboard(1u64 << Square::from(c).0)
    }
}

/// **Status/reading methods**
impl Bitboard {
    #[inline]
    pub fn is_empty(self) -> bool {
        self.0 == 0u64
    }

    #[inline]
    pub fn is_any(self) -> bool {
        self.0 != 0u64
    }

    #[inline]
    pub fn is_singular(self) -> bool {
        self.0.is_power_of_two()
    }
    
    #[inline]
    pub fn contains(self, s: Square) -> bool {
        (Bitboard::square(s) & self).is_any()
    }

    #[inline]
    pub fn count(self) -> u32 {
        self.0.count_ones()
    }

    #[inline]
    pub fn largest_square(self) -> Option<Square> {
        self.is_any().then(|| {
            let value = (63 - self.0.leading_zeros()) as u32;
            Square::new(value)
        })
    }

    #[inline]
    pub fn smallest_square(self) -> Option<Square> {
        self.is_any().then_some({
            let value = self.0.trailing_zeros() as u32; 
            Square::new(value)
        })
    }
}

/// **Update/modifying methods**
impl Bitboard {
    #[inline]
    pub fn insert(&mut self, s: Square) -> bool {
        let c = !self.contains(s);
        self.0 |= 1 << s.0;
        c
    }

    #[inline]
    pub fn remove(&mut self, s: Square) -> bool {
        let c = self.contains(s);
        self.0 &= !(1 << s.0);
        c
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;

    /// Union of two bitboards.
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;

    /// Intersection of two bitboards.
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;

    /// Disjoin union of two bitboards.
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    /// Complement of a bitboard.
    #[inline]
    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl Flippable for Bitboard {
    #[inline]
    fn flipped(&self) -> Self {
        Bitboard(self.0.reverse_bits())
    }
}

impl Iterator for Bitboard {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        let s = self.largest_square();
        if let Some(s) = s { self.remove(s); }
        s
    }
}

impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut b_chars = vec!['.'; 64];
        for s in self.into_iter() {
            b_chars[s.0 as usize] = 'x';
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