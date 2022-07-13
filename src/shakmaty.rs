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
