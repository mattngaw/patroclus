//! A module for working with and keeping track of squares.

use std::ops::{BitOr, BitAnd, BitXor, Not};
use std::ops::{BitOrAssign, BitAndAssign, BitXorAssign};
use std::fmt::{Display, Debug};

use crate::util::PRINT_ORDER;

/// Used to find the flipped representation of a object (from opposite POV)
pub trait Flippable 
where Self: Sized {
    /// Returns the flipped representation
    fn flipped(&self) -> Self;
    
    /// Flips the representation of `self` in place
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
    /// Total number of squares on a chessboard
    pub const COUNT: usize = 64;

    /// Use to iterate over squares from index 0 to 63
    const ITER: [Square; Self::COUNT] = {
        let mut result = [Square::new(0); Self::COUNT];
        let mut i = 0;
        while i < Self::COUNT {
            result[i] = Square::new(i as u32);
            i += 1;
        }
        result
    };

}

impl Square {
    /// Creates a new square
    #[inline]
    pub const fn new(value: u32) -> Self {    
        debug_assert!(value < 64);
        Square(value)
    }

    /// Gets rank of square
    #[inline]
    pub fn rank(self) -> Rank {
        Rank::try_from(self.0 / 8).unwrap()
    }
    
    /// Gets file of square
    #[inline]
    pub fn file(self) -> File {
        File::try_from(self.0 % 8).unwrap()
    }

    /// Returns an iterator over all of the squares
    pub fn iter() -> std::array::IntoIter<Square, {Self::COUNT}> {
        Self::ITER.into_iter()
    }
}

impl From<Coords> for Square {
    /// Creates a square from a [`Coords`]
    #[inline]
    fn from(Coords(f, r): Coords) -> Self {
        Square::new(r as u32 * 8 + f as u32)
    }
}

impl Flippable for Square {
    #[inline]
    fn flipped(&self) -> Self {
        Square(63 - self.0)
    }
}

impl Display for Square {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = self.rank();
        let f = self.file();
        let r_ch = char::from(r);
        let f_ch = char::from(f);
        write!(formatter, "{}{}", f_ch, r_ch)
    }
}

impl From<Square> for usize {
    /// Returns the value of the square as a [`usize`]
    /// 
    /// # Note
    /// Typically used as an index (e.g. into a [`Vec`] or array)
    #[inline]
    fn from(s: Square) -> Self {
        s.0 as usize
    }
}


/// A row on a chessboard
#[allow(missing_docs)]
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

impl TryFrom<u32> for Rank {
    type Error = u32;

    fn try_from(i: u32) -> Result<Self, Self::Error> {
        match i {
            0 => Ok(Rank::First),
            1 => Ok(Rank::Second),
            2 => Ok(Rank::Third),
            3 => Ok(Rank::Fourth),
            4 => Ok(Rank::Fifth),
            5 => Ok(Rank::Sixth),
            6 => Ok(Rank::Seventh),
            7 => Ok(Rank::Eighth),
            _ => Err(i),
        }
    }
}

impl From<Rank> for char {
    fn from(r: Rank) -> Self {
        match r {
            Rank::First => '1',
            Rank::Second => '2',
            Rank::Third => '3',
            Rank::Fourth => '4',
            Rank::Fifth => '5',
            Rank::Sixth => '6',
            Rank::Seventh => '7',
            Rank::Eighth => '8',
        }
    }
}

impl TryFrom<char> for Rank {
    type Error = char;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            '1' => Ok(Rank::First),
            '2' => Ok(Rank::Second),
            '3' => Ok(Rank::Third),
            '4' => Ok(Rank::Fourth),
            '5' => Ok(Rank::Fifth),
            '6' => Ok(Rank::Sixth),
            '7' => Ok(Rank::Seventh),
            '8' => Ok(Rank::Eighth),
            _ => Err(ch),
        }
    }
}

impl Rank {
    const ITER: [Rank; 8] = [
        Rank::First, 
        Rank::Second, 
        Rank::Third,
        Rank::Fourth, 
        Rank::Fifth, 
        Rank::Sixth, 
        Rank::Seventh, 
        Rank::Eighth
    ];

    /// Returns an iterator over all of the ranks
    pub fn iter() -> std::array::IntoIter<Rank, 8> {
        Self::ITER.into_iter()
    }
}

/// A column on a chessboard
#[allow(missing_docs)]
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

impl TryFrom<u32> for File {
    type Error = u32;

    fn try_from(i: u32) -> Result<Self, Self::Error> {
        match i {
            0 => Ok(File::A),
            1 => Ok(File::B),
            2 => Ok(File::C),
            3 => Ok(File::D),
            4 => Ok(File::E),
            5 => Ok(File::F),
            6 => Ok(File::G),
            7 => Ok(File::H),
            _ => Err(i),
        }
    }
}

impl From<File> for char {
    fn from(f: File) -> Self {
        match f {
            File::A => 'a',
            File::B => 'b',
            File::C => 'c',
            File::D => 'd',
            File::E => 'e',
            File::F => 'f',
            File::G => 'g',
            File::H => 'h',
        }
    }
}

impl TryFrom<char> for File {
    type Error = char;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            'a' => Ok(File::A),
            'b' => Ok(File::B),
            'c' => Ok(File::C),
            'd' => Ok(File::D),
            'e' => Ok(File::E),
            'f' => Ok(File::F),
            'g' => Ok(File::G),
            'h' => Ok(File::H),
            _ => Err(ch),
        }
    }
}

impl File {
    const ITER: [File; 8] = [
        File::A,
        File::B,
        File::C,
        File::D,
        File::E,
        File::F,
        File::G,
        File::H,
    ];

    /// Returns an iterator over all of the files
    pub fn iter() -> std::array::IntoIter<File, 8> {
        Self::ITER.into_iter()
    }
}


/// The coordinates of a square on a chessboard
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Coords(pub File, pub Rank);


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


/// Create methods
impl Bitboard {

    /// Creates a new bitboard from a [`u64`]
     #[inline]
    pub const fn new(value: u64) -> Self {
        Bitboard(value)
    }

    /// Creates an empty bitboard
    #[inline]
    pub const fn empty() -> Self {
        Bitboard(0u64)
    }

    /// Creates a full bitboard
    #[inline]
    pub const fn full() -> Self {
        Bitboard(!0u64)
    }

    /// Creates a bitboard with the squares in rank `r` set
    #[inline]
    pub const fn rank(r: Rank) -> Self {
        Bitboard(Self::RANK_MASKS[r as usize])
    }

    /// Creates a bitboard with the squares in file `f` set
    #[inline]
    pub const fn file(f: File) -> Self {
        Bitboard(Self::FILE_MASKS[f as usize])
    }

    /// Creates a bitboard with the square `s` set
    #[inline]
    pub const fn square(s: Square) -> Self {
        Bitboard(1u64 << (s.0 as u64))
    }

    /// Creates a bitboard with the square at coords `c` set
    #[inline]
    pub fn coords(c: Coords) -> Self {
        Bitboard(1u64 << Square::from(c).0)
    }
}


/// # Read methods
impl Bitboard {
    /// Returns `true` if the bitboard is empty
    #[inline]
    pub fn is_empty(self) -> bool {
        self.0 == 0u64
    }

    /// Returns `true` if the bitboard is nonempty
    #[inline]
    pub fn is_any(self) -> bool {
        self.0 != 0u64
    }

    /// Returns `true` if the bitboard contains only one square
    #[inline]
    pub fn is_singular(self) -> bool {
        self.0.is_power_of_two()
    }
    
    /// Returns `true` if the bitboard contains the square `s`
    #[inline]
    pub fn contains(self, s: Square) -> bool {
        (Bitboard::square(s) & self).is_any()
    }

    /// Returns the number of squares set in the bitboard
    #[inline]
    pub fn count(self) -> u32 {
        self.0.count_ones()
    }

    /// Returns the square with the highest index, or [`None`] if the bitboard
    /// is empty 
    #[inline]
    pub fn largest_square(self) -> Option<Square> {
        self.is_any().then(|| {
            let value = 63 - self.0.leading_zeros();
            Square::new(value)
        })
    }

    /// Returns the square with the lowest index, or [`None`] if the bitboard 
    /// is empty
    #[inline]
    pub fn smallest_square(self) -> Option<Square> {
        self.is_any().then_some({
            let value = self.0.trailing_zeros(); 
            Square::new(value)
        })
    }
}

/// # Update methods
impl Bitboard {
    /// Inserts the square to the bitboard and returns `true` if the square was 
    /// newly inserted
    #[inline]
    pub fn insert(&mut self, s: Square) -> bool {
        let c = !self.contains(s);
        self.0 |= 1 << s.0;
        c
    }

    /// Removes the square from the bitboard and returns `true` if the square
    /// was in the bitboard
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

    /// Disjoint union of two bitboards.
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