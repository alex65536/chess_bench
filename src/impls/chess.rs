use arrayvec::ArrayVec;
use chess::{Board, ChessMove, Color, File, MoveGen, Piece, Rank, Square};
use std::mem;
use std::str::FromStr;
use crate::MoveNotLegal;

pub struct Perft;
pub struct Test;

impl crate::Test for Test {
    type Board = Board;
    type Move = ChessMove;
    type Undo = Board;
    type MoveList = ArrayVec<ChessMove, 256>;

    fn get_move<'a>(&self, list: &'a Self::MoveList, idx: usize) -> &'a Self::Move {
        &list[idx]
    }

    fn move_count(&self, list: &Self::MoveList) -> usize {
        list.len()
    }

    fn board_from_fen(&self, fen: &str) -> Self::Board {
        Board::from_str(fen).expect("invalid fen")
    }

    fn try_make_move(&self, board: &mut Self::Board, mv: &Self::Move) -> Result<Self::Undo, MoveNotLegal> {
        let old = *board;
        old.make_move(*mv, board);
        Ok(old)
    }

    fn unmake_move(&self, board: &mut Self::Board, _mv: &Self::Move, u: &Self::Undo) {
        *board = *u;
    }

    fn move_str(&self, mv: &Self::Move) -> String {
        mv.to_string()
    }

    fn generate_moves(&self, b: &Self::Board) -> Self::MoveList {
        MoveGen::new_legal(b).collect()
    }

    fn is_attacked(&self, b: &Self::Board, is_white: bool, cx: char, cy: char) -> bool {
        let color = if is_white { Color::White } else { Color::Black };
        let file = File::from_index(cx as usize - 'a' as usize);
        let rank = Rank::from_index(cy as usize - '1' as usize);
        let pos = Square::make_square(rank, file);
        let our = b.color_combined(color);
        let all = b.combined();

        let attackers = (chess::get_knight_moves(pos) & b.pieces(Piece::Knight) & our)
            | (chess::get_king_moves(pos) & b.pieces(Piece::King) & our)
            | chess::get_pawn_attacks(pos, !color, b.pieces(Piece::Pawn) & our);
        if attackers != chess::EMPTY {
            return true;
        }

        let long_attack = b.color_combined(color)
            & ((chess::get_bishop_rays(pos) & (b.pieces(Piece::Bishop) | b.pieces(Piece::Queen)))
                | (chess::get_rook_rays(pos) & (b.pieces(Piece::Rook) | b.pieces(Piece::Queen))));
        for p in long_attack {
            if (chess::between(p, pos) & all) == chess::EMPTY {
                return true;
            }
        }

        false
    }

    fn is_check(&self, b: &Self::Board) -> bool {
        b.checkers() != &chess::EMPTY
    }

    fn run_self_test(&self, _b: &Self::Board) {}
}

impl Perft {
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

impl crate::Perft for Perft {
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
