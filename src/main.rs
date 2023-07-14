use patroclus::{
    position::{
        Position,
        zobrist::BuildZobristHasher
    }, 
    bits::{
        Flippable,
        Bitboard
    }
};

use std::fs::OpenOptions;

const LOG_PATH: &'static str = "logs/a.log";

fn main() {
    let log_file = OpenOptions::new()
                                                    .write(true)
                                                    .create(true)
                                                    .open(LOG_PATH)
                                                    .expect(format!("Could not open {}", LOG_PATH).as_str());

    env_logger::builder()
               .format_indent(Some(4))
               .target(env_logger::Target::Pipe(Box::new(log_file)))
               .init();


    log::info!("Hello, World!");
    // let fen_str = "r1b2r2/p2p1pk1/1pp2bp1/q5N1/7P/P4Q2/4RPP1/1NB2K1R w - - 0 20";
    // let p = Position::from_fen_string(fen_str.to_string()).unwrap();
    // log::debug!("\n{}", p);
    // log::debug!("{}", p.to_fen_string());
    // assert_eq!(p, p.flipped().flipped());
    // assert_eq!(fen_str, p.to_fen_string());

    // let bh = BuildZobristHasher::new();

    // let mut hs = std::collections::HashSet::with_hasher(bh);
    // hs.insert(p.clone());
    // assert!(hs.contains(&p));

    // let b = Bitboard::new(0x00034400);
    // for sub_b in b.subsets_slow() {
    //     assert!(sub_b.is_subset(b));
    // }
    // assert_eq!(b.subsets_slow().len(), (1 << b.count()) - 1);

    if cfg!(feature = "find-magics") {
        log::info!("Finding magic bitboards.");
    } else if cfg!(feature = "magics") {
        log::info!("Using magic bitboards for sliding move generation.");
    } else {
        log::info!("Using normal sliding move generation.");
    }

    log::info!("Goodbye, World!");
}
