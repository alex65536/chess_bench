use pleco::board::{perft, Board};
use pleco::{MoveList, Player};

pub struct Bench;

impl Bench {
    fn do_hperft(board: &mut Board, depth: usize) -> u64 {
        if depth == 0 {
            let white = board.get_occupied_player(Player::White).0;
            let black = board.get_occupied_player(Player::Black).0;
            return white
                .wrapping_mul(crate::HPERFT_WHITE)
                .wrapping_add(black.wrapping_mul(crate::HPERFT_BLACK));
        }

        let moves: MoveList = board.generate_moves();
        let mut count: u64 = 0;
        for mov in moves {
            board.apply_move(mov);
            count = count.wrapping_add(Self::do_hperft(board, depth - 1));
            board.undo_move();
        }
        count
    }
}

impl crate::Bench for Bench {
    fn name(&self) -> &'static str {
        "pleco"
    }

    fn perft(&self, fen: &str, depth: usize) -> u64 {
        let board = Board::from_fen(fen).expect("invalid fen");
        perft::perft(&board, depth as u16)
    }

    fn hperft(&self, fen: &str, depth: usize) -> u64 {
        let mut board = Board::from_fen(fen).expect("invalid fen");
        Self::do_hperft(&mut board, depth)
    }
}
