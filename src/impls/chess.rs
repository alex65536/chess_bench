use chess::{Board, Color, MoveGen};
use std::mem;
use std::str::FromStr;

pub struct Bench;

impl Bench {
    fn do_hperft(board: &Board, depth: usize) -> u64 {
        if depth == 0 {
            let white = board.color_combined(Color::White).0;
            let black = board.color_combined(Color::Black).0;
            return white
                .wrapping_mul(crate::HPERFT_WHITE)
                .wrapping_add(black.wrapping_mul(crate::HPERFT_BLACK));
        }

        let iterable = MoveGen::new_legal(board);
        let mut result: u64 = 0;

        for m in iterable {
            let mut bresult = mem::MaybeUninit::<Board>::uninit();
            unsafe {
                board.make_move(m, &mut *bresult.as_mut_ptr());
                result = result.wrapping_add(Self::do_hperft(&*bresult.as_ptr(), depth - 1));
            }
        }
        result
    }
}

impl crate::Bench for Bench {
    fn name(&self) -> &'static str {
        "chess"
    }

    fn perft(&self, fen: &str, depth: usize) -> u64 {
        let board = Board::from_str(fen).expect("invalid fen");
        MoveGen::movegen_perft_test(&board, depth) as u64
    }

    fn hperft(&self, fen: &str, depth: usize) -> u64 {
        let board = Board::from_str(fen).expect("invalid fen");
        Self::do_hperft(&board, depth)
    }
}
