use cozy_chess::{Board, Color};

pub struct Bench;

impl Bench {
    fn do_perft(board: &Board, depth: usize) -> u64 {
        if depth == 1 {
            let mut count = 0;
            board.generate_moves(|moves| {
                count += moves.len() as u64;
                false
            });
            return count;
        }

        let mut count: u64 = 0;
        board.generate_moves(|moves| {
            for mv in moves {
                let mut child = board.clone();
                child.play_unchecked(mv);
                count += Self::do_perft(&child, depth - 1);
            }
            false
        });
        count
    }

    fn do_hperft(board: &Board, depth: usize) -> u64 {
        if depth == 0 {
            let white = board.colors(Color::White).0;
            let black = board.colors(Color::Black).0;
            return white
                .wrapping_mul(super::HPERFT_WHITE)
                .wrapping_add(black.wrapping_mul(super::HPERFT_BLACK));
        }

        let mut count: u64 = 0;
        board.generate_moves(|moves| {
            for mv in moves {
                let mut child = board.clone();
                child.play_unchecked(mv);
                count = count.wrapping_add(Self::do_hperft(&child, depth - 1));
            }
            false
        });
        count
    }
}

impl super::Bench for Bench {
    fn name(&self) -> &'static str {
        "cozy_chess"
    }

    fn perft(&self, fen: &str, depth: usize) -> u64 {
        let board = Board::from_fen(fen, false).expect("invalid fen");
        Self::do_perft(&board, depth)
    }

    fn hperft(&self, fen: &str, depth: usize) -> u64 {
        let board = Board::from_fen(fen, false).expect("invalid fen");
        Self::do_hperft(&board, depth)
    }
}
