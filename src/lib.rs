pub trait Bench {
    fn name(&self) -> &'static str;
    fn perft(&self, fen: &str, depth: usize) -> u64;
    fn hperft(&self, fen: &str, depth: usize) -> u64;
}

pub(crate) const HPERFT_WHITE: u64 = 142867;
pub(crate) const HPERFT_BLACK: u64 = 285709;

pub mod chess;
pub mod cozy_chess;
pub mod owlchess;
pub mod pleco;
pub mod shakmaty;
