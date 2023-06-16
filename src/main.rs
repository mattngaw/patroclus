use std::hash::{Hash, Hasher};

use patroclus::{
    position::{
        Position,
        board::Board,
        zobrist::ZobristHasher
    }, 
    bits::{
        Flippable
    }
};

fn main() {
    env_logger::builder().init();

    log::info!("Hello, World!");
    let fen_str = "r1b2r2/p2p1pk1/1pp2bp1/q5N1/7P/P4Q2/4RPP1/1NB2K1R w - - 0 20";
    let p = Position::from_fen_string(fen_str.to_string()).unwrap();
    log::debug!("{:?}", Board::ZOBRIST_PRNS);
    log::debug!("\n{}", p);
    log::debug!("{}", p.to_fen_string());
    assert_eq!(p, p.flipped().flipped());
    assert_eq!(fen_str, p.to_fen_string());

    let mut h = ZobristHasher::new();
    p.hash(&mut h);
    log::debug!("hash(p) = {:#018x}", h.finish());

    log::info!("Goodbye, World!");
}
