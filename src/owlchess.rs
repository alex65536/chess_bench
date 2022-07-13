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
