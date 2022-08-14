use owlchess::{
    movegen::{self, legal, semilegal},
    moves::{self, make::TryUnchecked, RawUndo},
    selftest, Board, Color, Coord, File, Make, Move, MoveList, Rank, RawBoard,
};

pub struct Test;
pub struct Perft;

pub struct Undo {
    cur: RawBoard,
    undo: RawUndo,
}

impl crate::Test for Test {
    type Board = Board;
    type Move = Move;
    type MoveList = MoveList;
    type Undo = Undo;

    fn get_move<'a>(&self, list: &'a Self::MoveList, idx: usize) -> &'a Self::Move {
        &list[idx]
    }

    fn move_count(&self, list: &Self::MoveList) -> usize {
        list.len()
    }

    fn board_from_fen(&self, fen: &str) -> Self::Board {
        Board::from_fen(fen).expect("invalid fen")
    }

    fn make_move(&self, board: &mut Self::Board, mv: &Self::Move) -> Self::Undo {
        mv.semi_validate(board).expect("move is not semi-legal");
        let undo = unsafe { moves::make_move_unchecked(board, *mv) };
        Undo {
            cur: *board.raw(),
            undo,
        }
    }

    fn unmake_move(&self, board: &mut Self::Board, mv: &Self::Move, u: &Self::Undo) {
        assert_eq!(&u.cur, board.raw());
        unsafe { moves::unmake_move_unchecked(board, *mv, u.undo) };
    }

    fn move_str(&self, mv: &Self::Move) -> String {
        mv.to_string()
    }

    fn generate_moves(&self, b: &Self::Board) -> Self::MoveList {
        assert!(!b.is_opponent_king_attacked());
        semilegal::gen_all(b)
    }

    fn is_attacked(&self, b: &Self::Board, is_white: bool, cx: char, cy: char) -> bool {
        let color = if is_white { Color::White } else { Color::Black };
        let p = Coord::from_parts(File::from_char(cx).unwrap(), Rank::from_char(cy).unwrap());
        movegen::is_cell_attacked(b, p, color)
    }

    fn is_check(&self, b: &Self::Board) -> bool {
        b.is_check()
    }

    fn is_last_move_legal(&self, b: &Self::Board) -> bool {
        !b.is_opponent_king_attacked()
    }

    fn run_self_test(&self, b: &Self::Board) {
        selftest::selftest(b)
    }
}

impl Perft {
    fn do_perft(b: &mut Board, depth: usize) -> u64 {
        match depth {
            0 => 1,
            1 => legal::gen_all(b).len() as u64,
            _ => semilegal::gen_all(b)
                .iter()
                .map(|mv| {
                    let u = match unsafe { TryUnchecked::new(*mv) }.make_raw(b) {
                        Ok((_, u)) => u,
                        Err(_) => return 0,
                    };
                    let res = Self::do_perft(b, depth - 1);
                    unsafe { moves::unmake_move_unchecked(b, *mv, u) };
                    res
                })
                .sum(),
        }
    }

    fn do_hperft(b: &mut Board, depth: usize) -> u64 {
        if depth == 0 {
            let white: u64 = b.color(Color::White).flipped_rank().into();
            let black: u64 = b.color(Color::Black).flipped_rank().into();
            return white
                .wrapping_mul(crate::HPERFT_WHITE)
                .wrapping_add(black.wrapping_mul(crate::HPERFT_BLACK));
        }

        let mut result: u64 = 0;
        let moves = semilegal::gen_all(b);
        for mv in &moves {
            let u = match unsafe { TryUnchecked::new(*mv) }.make_raw(b) {
                Ok((_, u)) => u,
                Err(_) => continue,
            };
            result = result.wrapping_add(Self::do_hperft(b, depth - 1));
            unsafe { moves::unmake_move_unchecked(b, *mv, u) };
        }
        result
    }
}

impl crate::Perft for Perft {
    fn name(&self) -> &'static str {
        "owlchess"
    }

    fn perft(&self, fen: &str, depth: usize) -> u64 {
        let mut board = Board::from_fen(fen).expect("invalid fen");
        Self::do_perft(&mut board, depth)
    }

    fn hperft(&self, fen: &str, depth: usize) -> u64 {
        let mut board = Board::from_fen(fen).expect("invalid fen");
        Self::do_hperft(&mut board, depth)
    }
}
