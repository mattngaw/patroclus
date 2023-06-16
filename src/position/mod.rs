//! Keeping track of chessboard state.

pub mod mailbox;
pub mod castling;
pub mod board;
pub mod util;

use crate::bits::*;
use self::castling::*;
use self::board::Board;
use self::util::*;

use std::fmt::Display;


/// The color of a piece, turn, etc.
#[allow(missing_docs)]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl From<Color> for char {
    fn from(c: Color) -> Self {
        match c {
            Color::White => 'w',
            Color::Black => 'b',
        }
    }
}

impl TryFrom<char> for Color {
    type Error = char;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'w' => Ok(Color::White),
            'b' => Ok(Color::Black),
            _ => Err(c),
        }
    }
}

/// The type of chess piece
#[allow(missing_docs)]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Role {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

impl Role {
    const NUM_ROLES: usize = 6;

    const ITER: [Role; Self::NUM_ROLES] = [
        Role::Pawn, 
        Role::Knight, 
        Role::Bishop, 
        Role::Rook, 
        Role::Queen, 
        Role::King
    ];

    const ITER_PIECE: [Role; Self::NUM_ROLES - 1] = [
        Role::Pawn, 
        Role::Knight, 
        Role::Bishop, 
        Role::Rook, 
        Role::Queen, 
    ]; 
}

/// A tuple of a [`Color`] and [`Role`] representing a piece on a chessboard
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Piece(pub Color, pub Role);

impl From<Piece> for char {
    fn from(p: Piece) -> Self {
        match p {
            WHITE_PAWN => 'P',
            WHITE_KNIGHT => 'N',
            WHITE_BISHOP => 'B',
            WHITE_ROOK => 'R',
            WHITE_QUEEN => 'Q',
            WHITE_KING => 'K',
            BLACK_PAWN => 'p',
            BLACK_KNIGHT => 'n',
            BLACK_BISHOP => 'b',
            BLACK_ROOK => 'r',
            BLACK_QUEEN => 'q',
            BLACK_KING => 'k',
        }
    }
}

impl TryFrom<char> for Piece {
    type Error = char;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'P' => Ok(WHITE_PAWN),
            'N' => Ok(WHITE_KNIGHT),
            'B' => Ok(WHITE_BISHOP),
            'R' => Ok(WHITE_ROOK),
            'Q' => Ok(WHITE_QUEEN),
            'K' => Ok(WHITE_KING),
            'p' => Ok(BLACK_PAWN),
            'n' => Ok(BLACK_KNIGHT),
            'b' => Ok(BLACK_BISHOP),
            'r' => Ok(BLACK_ROOK),
            'q' => Ok(BLACK_QUEEN),
            'k' => Ok(BLACK_KING),
            _ => Err(c),
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

/// A time-dependent representation of the state of a chess game
#[derive(PartialEq, Eq, Debug)]
pub struct Position {
    board: Board,
    turn: Color,
    castling: Castling,
    en_passant: Option<Square>,
    halfmove: u32,
    fullmove: u32,
}

impl Position {
    /// Creates a new, empty chess position
    pub fn new() -> Self {
        Position {
            board: Board::new(),
            turn: Color::White,
            castling: Castling::new(),
            en_passant: None,
            halfmove: 1,
            fullmove: 1,
        }
    }

    /// Attempts to create a chess position from a FEN string
    pub fn from_fen_string(fen: String) -> Result<Position, &'static str> {
        let tokens: Vec<&str> = fen.split(' ').collect();
        
        if tokens.len() != 6 {
            return Err("Invalid number of fields in FEN string")
        }
        
        let placement_str = tokens[0];
        let turn_str = tokens[1];
        let castling_str = tokens[2];
        let en_passant_str = tokens[3];
        let halfmove_str = tokens[4];
        let fullmove_str = tokens[5];
        
        let p = Position {
            board: Board::from_placement(get_placement(placement_str)),
            turn: get_turn(turn_str),
            castling: get_castling(castling_str),
            en_passant: get_en_passant(en_passant_str),
            halfmove: get_number(halfmove_str),
            fullmove: get_number(fullmove_str),
        };
        
        Ok(p)
    }
}

impl Default for Position {
    /// Creates the starting position of a chess game
    fn default() -> Self {
        Self { 
            board: Default::default(), 
            turn: Color::White, 
            castling: Default::default(), 
            en_passant: Default::default(), 
            halfmove: 0, 
            fullmove: 0
        }
    }
}

impl Flippable for Position {
    fn flipped(&self) -> Self {
        Position {
            board: self.board.flipped(),
            turn: self.turn,
            castling: self.castling,
            en_passant: self.en_passant.map(|s| s.flipped()),
            halfmove: self.halfmove,
            fullmove: self.fullmove,
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.board)
    }
}

impl Position {
    /// Creates a FEN string from the position
    pub fn to_fen_string(&self) -> String {
        let mut fen = String::new();
        
        let Position {
            board,
            turn,
            castling,
            en_passant,
            halfmove,
            fullmove,
        } = self;
        
        placement_str(board, &mut fen);
        fen.push(' ');

        fen.push(char::from(*turn));
        fen.push(' ');

        fen.push_str(&castling.to_string());
        fen.push(' ');

        match en_passant {
            Some(s) => fen.push_str(&s.to_string()),
            None => fen.push('-'),
        }
        fen.push(' ');

        fen.push_str(&halfmove.to_string());
        fen.push(' ');

        fen.push_str(&fullmove.to_string());

        fen
    }
}

// Position::to_fen_string helper functions

fn placement_str(board: &Board, fen: &mut String) {
    for r in Rank::iter().rev() {
        let mut space = 0;
        for f in File::iter() {
            let s = Square::from(Coords(f, r));
            match board.get(s) {
                Some(p) => {
                    if space > 0 {
                        fen.push_str(&space.to_string());
                    }
                    fen.push(char::from(p));
                    space = 0;
                }
                None => space += 1
            }
        }
    
        if space > 0 {
            fen.push_str(&space.to_string())
        }
    
        if r != Rank::First {
            fen.push('/');
        }
    }
}


// Position::from_fen_string helper functions

fn get_placement(s: &str) -> [Option<Piece>; 64] {
    let mut placement = [None; 64];
    let mut f_index = 0;
    let mut r_index = 7;
    for ch in s.chars() {
        let s_index = f_index + r_index * 8;
        if let Some(offset) = ch.to_digit(10) {
            let offset = offset as usize;
            assert!(0 < offset && offset < 9);
            for i in 0..offset {
                placement[s_index + i] = None;
            }
            f_index += offset;
        } else if ch == '/' {
            assert_eq!(f_index, 8);
            f_index = 0;
            r_index -= 1;
        } else {
            match Piece::try_from(ch) {
                Ok(p) => placement[s_index] = Some(p),
                Err(ch) => panic!("{ch} is not a valid FEN placement character"),
            }
            f_index += 1;
        }
    }
    placement
}

fn get_turn(s: &str) -> Color {
    assert_eq!(s.len(), 1);
    let ch = s.chars().next().unwrap();
    match Color::try_from(ch) {
        Ok(c) => c,
        Err(ch) => panic!("{ch} is not a valid FEN turn"),
    }
}

fn get_castling(s: &str) -> Castling {
    assert!(s.len() < 5);
    let mut castling = Castling::new();
    if s == "-" {
        return castling
    }
    let (mut w_ks, mut w_qs) = (false, false);
    let (mut b_ks, mut b_qs) = (false, false);
    for ch in s.chars() {
        match ch {
            'K' => w_ks = true,
            'Q' => w_qs = true,
            'k' => b_ks = true,
            'q' => b_qs = true,
            _ => panic!("{ch} is an invalid FEN castling character")
        }
    }
    let white_rights = CastlingRights::new(w_ks, w_qs);
    let black_rights = CastlingRights::new(b_ks, b_qs);
    castling.set(Color::White, white_rights);
    castling.set(Color::Black, black_rights);
    castling
}

fn get_en_passant(s: &str) -> Option<Square> {
    if s.len() == 1 {
        assert_eq!(s, "-");
        None
    } else if s.len() == 2 {
        let mut chs = s.chars();
        let f_ch = chs.next().unwrap();
        let r_ch = chs.next().unwrap();
        let f = match File::try_from(f_ch) {
            Ok(f) => f,
            Err(ch) => panic!("{ch} is an invalid file character"),
        };
        let r = match Rank::try_from(r_ch) {
            Ok(r) => r,
            Err(ch) => panic!("{ch} is an invalid rank character"),
        };
        Some(Square::from(Coords(f, r)))
    } else {
        panic!("{s} is invalid en_passant token");
    }
}

fn get_number(s: &str) -> u32 {
    match s.to_string().parse::<u32>() {
        Ok(n) => n,
        Err(_) => panic!("{s} is invalid move number"),
    }
}