//! The physical, time-independent state of a chess board
//! 
//! A [`Board`] is only concerned with keeping track of the placement of pieces
//! on a chess board.

use std::fmt::Display;

use crate::bits::{Bitboard, Square, Flippable};
use crate::position::util::{WHITE_KING, BLACK_KING};
use super::mailbox::Mailbox;
use super::{Color, Role, Piece};

/// The physical, time-independent state of a chess board
#[derive(PartialEq, Eq, Debug)]
pub struct Board {
    colors: [Bitboard; 2],
    roles: [Bitboard; 5],
    kings: [Square; 2],
    pieces: Mailbox,
}

/// # Create methods
impl Board {
    /// Creates a new chessboard
    ///
    /// By default the kings are on e1 and e8
    pub fn new() -> Self {
        let white_king = Square::new(4);
        let black_king = Square::new(60);

        Board {
            colors: [
                Bitboard::square(white_king), 
                Bitboard::square(black_king)
            ],
            roles: [Bitboard::empty(); 5],
            kings: [white_king, black_king],
            pieces: Mailbox::new(),
        }
    }

    /// Creates a board from the placement of pieces
    pub fn from_placement(pm: [Option<Piece>; 64]) -> Self {
        let mut board = Board::new();
        let mailbox = Mailbox::from_placement(pm);
        for (s, o_p) in mailbox {
            if let Some(p) = o_p {
                board.place(s, p);
            }
        }
        board.debug_verify();
        board
    }
}

impl Default for Board {
    fn default() -> Self {
        const WHITE_DEFAULT: Bitboard = Bitboard::new(0x0000_0000_0000_FFFF);
        const BLACK_DEFAULT: Bitboard = Bitboard::new(0xFFFF_0000_0000_0000);

        const PAWNS_DEFAULT: Bitboard = Bitboard::new(0x00FF_0000_0000_FF00);
        const KNIGHTS_DEFAULT: Bitboard = Bitboard::new(0x4200_0000_0000_0042);
        const BISHOPS_DEFAULT: Bitboard = Bitboard::new(0x2400_0000_0000_0024);
        const ROOKS_DEFAULT: Bitboard = Bitboard::new(0x8100_0000_0000_0081);
        const QUEENS_DEFAULT: Bitboard = Bitboard::new(0x0800_0000_0000_0008);

        Self { 
            colors: [WHITE_DEFAULT, BLACK_DEFAULT], 
            roles: [
                PAWNS_DEFAULT,
                KNIGHTS_DEFAULT,
                BISHOPS_DEFAULT,
                ROOKS_DEFAULT,
                QUEENS_DEFAULT
            ], 
            kings: [Square::new(4), Square::new(52)], 
            pieces: Default::default() 
        }
    }
}

/// # Read methods
impl Board {
    /// Gets the piece at a square `s`, if any
    #[inline]
    pub fn get(&self, s: Square) -> Option<Piece> {
        self.debug_verify();

        let o_p = self.pieces[s];

        debug_assert_eq!(o_p, self.get_bitboard(s));

        o_p
    }

    /// Gets the piece at a square `s`, if any, via bitboards instead of the 
    /// mailbox
    /// 
    /// Use for verification only, as this method is slower than checking the 
    /// mailbox
    #[inline]
    fn get_bitboard(&self, s: Square) -> Option<Piece> {
        self.debug_verify();

        if self.kings[0] == s { return Some(WHITE_KING) }
        else if self.kings[1] == s { return Some(BLACK_KING) }

        let white_b = self.colors[Color::White as usize];
        let black_b = self.colors[Color::Black as usize];

        let white = white_b.contains(s);
        let black = black_b.contains(s);

        let c = match (white, black) {
            (true, true) => panic!("Square can't contain white and black piece"),
            (false, false) => return None,
            (true, false) => Color::White,
            (false, true) => Color::Black,
        };

        for r in Role::ITER_PIECE {
            let role_b = self.roles[r as usize];
            if role_b.contains(s) {
                return Some(Piece(c, r));
            }
        }

        panic!("Found piece in color_b but not in role_b")
    }

    /// Gets the bitboard containing all squares of color `c`
    #[inline]
    pub fn color(&self, c: Color) -> Bitboard {
        self.colors[c as usize]
    }

    /// Gets the bitboard containing all squares of role `r`
    #[inline]
    pub fn role(&self, r: Role) -> Bitboard {
        debug_assert_ne!(r, Role::King);
        self.roles[r as usize]
    }

    /// Gets the bitboard containing all squares of piece `p`
    /// 
    /// # Precondition
    /// 
    /// If querying for the king, use [`king_square`](Self::king_square()) 
    /// instead
    #[inline]
    pub fn piece(&self, p: Piece) -> Bitboard {
        debug_assert_ne!(p.1, Role::King);
        let Piece(c, r) = p;
        self.color(c) & self.role(r)
    }

    /// Gets the square of the color `c` king
    #[inline]
    pub fn king_square(&self, c: Color) -> Square {
        self.kings[c as usize]
    }

    /// Gets the bitboard containing the square of the color `c` king
    #[inline]
    pub fn king_bitboard(&self, c: Color) -> Bitboard {
        Bitboard::square(self.king_square(c))
    }

    /// Gets the bitboard containing all the occupied squares
    #[inline]
    pub fn all(&self) -> Bitboard {
        self.colors[Color::White as usize] | self.colors[Color::Black as usize]
    }

    /// Gets the bitboard containing all the empty squares
    #[inline]
    pub fn none(&self) -> Bitboard {
        !self.all()
    }
}

/// # Update methods
impl Board {
    /// Attempts to place piece `p` on square `s`
    /// 
    /// If `s` is empty, it places `p` on `s` and returns `true`,
    /// otherwise does nothing and returns false
    /// 
    /// This method must case on the occupancy of a square, and thus must not be
    /// used for quickly moving pieces (see [`replace`](Self::replace()) and 
    /// [`move`](Self::move()))
    /// 
    /// In most cases, this method should only be used to place *new* pieces on 
    /// the board
    pub fn place(&mut self, s: Square, p: Piece) -> bool {
        self.debug_verify();

        let status = match (self.pieces[s], p) {
            // Nonempty square
            (Some(_), _) => false,
            
            // The king cannot be "placed", only moved
            (None, Piece(c, Role::King)) => {
                // Remove from board
                let old_s = self.kings[c as usize];
                self.colors[c as usize].remove(old_s);
                self.pieces[old_s] = None;
                
                // Add to board
                self.kings[c as usize] = s;
                self.colors[c as usize].insert(s);
                self.pieces[s] = Some(p);
                true
            }
            (None, Piece(c, r)) => {
                self.colors[c as usize].insert(s);
                self.roles[r as usize].insert(s);
                self.pieces[s] = Some(p);
                true
            }
        };
        
        self.debug_verify();

        status
    }

    /// Replaces the piece (if any) on square `s` with the piece `p`
    /// 
    /// # Preconditions
    /// 
    /// Cannot replace on `s` with a king, as the king can only be moved
    /// 
    /// Cannot replace a king, as there must always be one king for each color
    pub fn replace(&mut self, s: Square, p: Piece) -> Option<Piece> {
        debug_assert_ne!(p.1, Role::King);
        
        self.debug_verify();

        let captured = self.pieces[s];

        // Require that the replaced piece is not the king
        debug_assert!(captured.map_or(true, |p| p.1 != Role::King));
        
        self.colors[p.0 as usize].insert(s);
        self.roles[p.1 as usize].insert(s);
        self.pieces[s] = Some(p);

        self.debug_verify();

        captured
    }

    /// Moves a (non-king) piece from `s_from` to `s_to`
    /// 
    /// Returns the captured piece that was on `s_to`, if there was one
    /// 
    /// Use  [`king_move`][Self::king_move()] to move a king
    /// 
    /// # Requires
    /// 
    /// Neither the capturing nor captured pieces can be kings
    pub fn r#move(&mut self, s_from: Square, s_to: Square) -> Option<Piece> {
        self.debug_verify();

        let capturer = self.get(s_from);
        debug_assert!(capturer.is_some());
        let capturer = capturer.unwrap();
        debug_assert!(capturer.1 != Role::King);

        let captured = self.get(s_to);
        
        self.colors[capturer.0 as usize].remove(s_from);
        self.roles[capturer.1 as usize].remove(s_from);
        self.pieces[s_from] = None;

        self.colors[capturer.0 as usize].insert(s_to);
        self.roles[capturer.1 as usize].insert(s_to);
        self.pieces[s_to] = Some(capturer);

        if let Some(captured) = captured {
            debug_assert_ne!(captured.1, Role::King);
            debug_assert_ne!(capturer.0, captured.0);
            self.colors[captured.0 as usize].remove(s_to);
            self.roles[captured.1 as usize].remove(s_to);
        }

        self.debug_verify();

        captured
    }

    /// Moves the king of color `c` to `s_to`
    /// 
    /// Returns the captured piece that was on `s_to`, if there was one
    /// 
    /// # Requires
    /// 
    /// The captured piece cannot be a king
    pub fn king_move(&mut self, c: Color, s_to: Square) -> Option<Piece> {
        self.debug_verify();

        let s_from = self.kings[c as usize];

        let captured = self.get(s_to);

        self.colors[c as usize].remove(s_from);
        self.pieces[s_from] = None;

        self.colors[c as usize].insert(s_to);
        self.pieces[s_to] = Some(Piece(c, Role::King));
        self.kings[c as usize] = s_to;

        if let Some(captured) = captured {
            debug_assert_ne!(captured.1, Role::King);
            debug_assert_ne!(c, captured.0);
            self.colors[captured.0 as usize].remove(s_to);
            self.roles[captured.1 as usize].remove(s_to);
        }

        self.debug_verify();

        captured
    }
}

impl Flippable for Board {
    fn flipped(&self) -> Self {
        Board {
            colors: [self.colors[0].flipped(), self.colors[1].flipped()],
            roles: [
                self.roles[0].flipped(),
                self.roles[1].flipped(),
                self.roles[2].flipped(),
                self.roles[3].flipped(),
                self.roles[4].flipped(),
            ],
            kings: [self.kings[0].flipped(), self.kings[1].flipped()],
            pieces: self.pieces.flipped(),
        }
    }
}

/// # Debug methods
impl Board {
    /// Verifies that the board state is legal
    /// 
    /// Only does checks in dev/debug builds, and disappears in release builds
    pub fn debug_verify(&self) {
        if !cfg!(debug_assertions) { 
            return
        }

        use itertools::Itertools;

        log::debug!("debug_verify: Starting...");

        let white = self.colors[Color::White as usize];
        let black = self.colors[Color::Black as usize];

        let pawns = self.roles[Role::Pawn as usize];
        let knights = self.roles[Role::Knight as usize];
        let bishops = self.roles[Role::Bishop as usize];
        let rooks = self.roles[Role::Rook as usize];
        let queens = self.roles[Role::Queen as usize];
        
        let pieces = vec![pawns, knights, bishops, rooks, queens];
        let pieces_pairings = pieces.into_iter().combinations(2);

        log::debug!("  Checking that white and black pieces don't overlap");
        assert_eq!(white & black, Bitboard::empty());

        log::debug!("  Checking that roles don't overlap");
        for pair in pieces_pairings {
            assert!((pair[0] & pair[1]).is_empty());
        }

        log::debug!("  Verifying the king squares");
        assert_ne!(self.kings[Color::White as usize], 
                   self.kings[Color::Black as usize]);
        // TODO Check that the kings are not adjacent

        log::debug!("  Checking colors and roles overlap once and only once");
        for c in [Color::White, Color::Black] {
            let c_b = self.colors[c as usize];
            for s in c_b {
                let mut overlapped = false;
                for r_b in self.roles {
                    if r_b.contains(s) {
                        assert!(!overlapped);
                        overlapped = true;
                    }
                }
                if self.kings[c as usize] == s {
                    assert!(!overlapped);
                    overlapped = true;
                }
                assert!(overlapped);
            }
        }

        log::debug!("  Checking that the mailbox matches the bitboards");
        for (s, o_p) in self.pieces {
            match o_p {
                None => assert!(
                    !white.contains(s) && !black.contains(s)
                ),
                Some(Piece(c, r)) => {
                    assert!(self.colors[c as usize].contains(s));
                    if r == Role::King {
                        assert_eq!(self.kings[c as usize], s);
                    } else {
                        assert!(self.roles[r as usize].contains(s));
                    }
                }
            }
        }

        log::debug!("debug_verify: Passed!");
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pieces)
    }
}

#[cfg(test)]
mod board_tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_debug_verify() {
        init();

        let b = Board::new();
        b.debug_verify();
    }
}