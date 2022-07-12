pub trait Bench {
    fn name(&self) -> &'static str;
    fn perft(&self, fen: &str, depth: usize) -> u64;
    fn hperft(&self, fen: &str, depth: usize) -> u64;
}

pub(crate) const HPERFT_WHITE: u64 = 142867;
pub(crate) const HPERFT_BLACK: u64 = 285709;

pub mod chess {
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
                    .wrapping_mul(super::HPERFT_WHITE)
                    .wrapping_add(black.wrapping_mul(super::HPERFT_BLACK));
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

    impl super::Bench for Bench {
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
}

pub mod owlchess {
    use owlchess::{
        movegen::{legal, semilegal},
        moves, Board, Color,
    };

    pub struct Bench;

    impl Bench {
        fn do_perft(b: &mut Board, depth: usize) -> u64 {
            match depth {
                0 => 1,
                1 => legal::gen_all(b).len() as u64,
                _ => semilegal::gen_all(b)
                    .iter()
                    .map(|mv| {
                        let u = match unsafe { moves::try_make_move_unchecked(b, *mv) } {
                            Ok(u) => u,
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
                    .wrapping_mul(super::HPERFT_WHITE)
                    .wrapping_add(black.wrapping_mul(super::HPERFT_BLACK));
            }

            let mut result: u64 = 0;
            let moves = semilegal::gen_all(b);
            for mv in &moves {
                let u = match unsafe { moves::try_make_move_unchecked(b, *mv) } {
                    Ok(u) => u,
                    Err(_) => continue,
                };
                result = result.wrapping_add(Self::do_hperft(b, depth - 1));
                unsafe { moves::unmake_move_unchecked(b, *mv, u) };
            }
            result
        }
    }

    impl super::Bench for Bench {
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
}

pub mod shakmaty {
    use shakmaty::{fen::Fen, CastlingMode, Chess, Color, Position};

    pub struct Bench;

    impl Bench {
        fn do_hperft(pos: &Chess, depth: usize) -> u64 {
            if depth < 1 {
                let white = pos.board().by_color(Color::White).0;
                let black = pos.board().by_color(Color::Black).0;
                white
                    .wrapping_mul(super::HPERFT_WHITE)
                    .wrapping_add(black.wrapping_mul(super::HPERFT_BLACK))
            } else {
                let moves = pos.legal_moves();
                let mut result: u64 = 0;
                for m in &moves {
                    let mut child = pos.clone();
                    child.play_unchecked(m);
                    result = result.wrapping_add(Self::do_hperft(&child, depth - 1))
                }
                result
            }
        }
    }

    impl super::Bench for Bench {
        fn name(&self) -> &'static str {
            "shakmaty"
        }

        fn perft(&self, fen: &str, depth: usize) -> u64 {
            let pos: Chess = fen
                .parse::<Fen>()
                .expect("invalid fen")
                .into_position(CastlingMode::Standard)
                .expect("invalid setup");
            shakmaty::perft(&pos, depth as u32)
        }

        fn hperft(&self, fen: &str, depth: usize) -> u64 {
            let pos: Chess = fen
                .parse::<Fen>()
                .expect("invalid fen")
                .into_position(CastlingMode::Standard)
                .expect("invalid setup");
            Self::do_hperft(&pos, depth)
        }
    }
}

// TODO pleco
// TODO cozy-chess
