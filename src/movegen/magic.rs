use rand::random;

use crate::{
    bits::{
        Bitboard,
        Square,
    },
    position::{
        Role,
        board::Board
    },
    movegen::{
        util::*,
    },
};


struct MagicEntry {
    mask: Bitboard,
    magic: u64,
    index_bits: u8,
}

struct TableFillError;

// const ROOK_MAGICS: [MagicEntry; Square::COUNT] = todo!();
// const BISHOP_MAGICS: [MagicEntry; Square::COUNT] = todo!();

// const ROOK_MOVES: [&[Bitboard]; Square::COUNT] = todo!();
// const BISHOP_MOVES: [&[Bitboard]; Square::COUNT] = todo!();

fn find_mask(r: Role, s: Square, board: Board) -> Bitboard {
    debug_assert!(r != Role::Bishop || r != Role::Rook);
    Bitboard::EMPTY
}

fn try_make_table(
    r: Role, 
    s: Square, 
    board: Board, 
    entry: &MagicEntry
) -> Result<Vec<Bitboard>, TableFillError> {
    let mut table = vec!(Bitboard::EMPTY; 1 << entry.index_bits);
    for blockers in entry.mask.subsets() {
        let moves = todo!();
        let b = &mut table[magic_index(entry, blockers)];
        if b.is_empty() {
            *b = moves;
        } else if *b != moves {
            return Err(TableFillError)
        }
    }
    Ok(table)
}

fn find_magic(
    r: Role, 
    s: Square, 
    board: Board, 
    index_bits: u8
) -> (MagicEntry, Vec<Bitboard>) {
    let mask = find_mask(r, s, board);
    loop {
        let magic = random::<u64>() & random::<u64>() & random::<u64>();
        let entry = MagicEntry { mask, magic, index_bits };
        if let Ok(table) = try_make_table(r, s, board, &entry) {
            return (entry, table)
        }
    }
}

fn magic_index(entry: &MagicEntry, blockers: Bitboard) -> usize {
    let blockers = blockers & entry.mask;
    let hash = u64::from(blockers).wrapping_mul(entry.magic);
    let index = (hash >> (64 - entry.index_bits)) as usize;
    index
}

pub fn dump_magics() {

}