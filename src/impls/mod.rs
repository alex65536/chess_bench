pub mod chess;
pub mod cozy_chess;
pub mod owlchess;
pub mod pleco;
pub mod shakmaty;

pub fn all_perft() -> Vec<Box<dyn super::Perft>> {
    vec![
        Box::new(chess::Perft),
        Box::new(owlchess::Perft),
        Box::new(shakmaty::Perft),
        Box::new(pleco::Perft),
        Box::new(cozy_chess::Perft),
    ]
}
