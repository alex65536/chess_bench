pub trait Bench {
    fn name(&self) -> &'static str;
    fn perft(&self, fen: &str, depth: usize) -> u64;
    fn hperft(&self, fen: &str, depth: usize) -> u64;
}

pub trait Test {
    type Board;
    type Move;
    type Undo;
    type MoveList;

    fn get_move<'a>(&self, list: &'a Self::MoveList, idx: usize) -> &'a Self::Move;
    fn move_count(&self, list: &Self::MoveList) -> usize;
    fn board_from_fen(&self, fen: &str) -> Self::Board;
    fn make_move(&self, board: &mut Self::Board, mv: &Self::Move) -> Self::Undo;
    fn unmake_move(&self, board: &mut Self::Board, mv: &Self::Move, u: &Self::Undo);
    fn move_str(&self, mv: &Self::Move) -> String;
    fn generate_moves(&self, b: &Self::Board) -> Self::MoveList;
    fn is_attacked(&self, b: &Self::Board, is_white: bool, cx: char, cy: char) -> bool;
    fn is_check(&self, b: &Self::Board) -> bool;
    fn is_last_move_legal(&self, b: &Self::Board) -> bool;
    fn run_self_test(&self, _b: &Self::Board) {}
}

pub(crate) const HPERFT_WHITE: u64 = 142867;
pub(crate) const HPERFT_BLACK: u64 = 285709;

pub mod impls;
pub mod selftest;
