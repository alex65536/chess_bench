use arrayvec::ArrayVec;
use std::io::{BufRead, Write};

#[derive(Copy, Clone, Debug)]
pub struct Options {
    pub big_depth: bool,
    pub dump_trace_chains: bool,
    pub run_self_test: bool,
    pub attack_heatmaps: bool,
}

impl Default for Options {
    #[inline]
    fn default() -> Self {
        Self {
            big_depth: true,
            dump_trace_chains: false,
            run_self_test: true,
            attack_heatmaps: true,
        }
    }
}

pub struct Tester<'a, T, W> {
    test: T,
    options: Options,
    writer: &'a mut W,
}

struct DepthCtx<'a> {
    spec: &'a DepthSpec,
    hash: u64,
    chain: String,
}

impl<'a> DepthCtx<'a> {
    fn new(spec: &'a DepthSpec) -> Self {
        Self {
            spec,
            hash: 0,
            chain: String::new(),
        }
    }

    fn grow_hash(&mut self, val: u64) {
        self.hash = self.hash.wrapping_mul(2579);
        self.hash = self.hash.wrapping_add(val);
    }
}

struct DepthSpec {
    depth: usize,
    with_heatmaps: bool,
}

impl DepthSpec {
    fn name(&self) -> String {
        match self.with_heatmaps {
            true => format!("{}-heatmaps", self.depth),
            false => format!("{}", self.depth),
        }
    }
}

impl<'a, T: crate::Test, W: Write> Tester<'a, T, W> {
    pub fn new(test: T, options: Options, writer: &'a mut W) -> Self {
        Self {
            test,
            options,
            writer,
        }
    }

    fn move_strings(&self, board: &mut T::Board, moves: &T::MoveList) -> Vec<String> {
        let t = &self.test;

        let count = t.move_count(moves);
        let mut result = Vec::with_capacity(count);
        for i in 0..count {
            let mv = t.get_move(moves, i);
            let u = t.make_move(board, mv);
            if t.is_last_move_legal(board) {
                result.push(t.move_str(mv));
            }
            t.unmake_move(board, mv, &u);
        }
        result.sort();
        result
    }

    fn move_hash(&self, mv: &T::Move) -> u64 {
        let s = self.test.move_str(mv);
        assert!(matches!(s.len(), 4 | 5));
        let s = s.as_bytes();
        let mut res = (s[0] as u64 - 'a' as u64) * 512
            + (s[1] as u64 - '1' as u64) * 64
            + (s[2] as u64 - 'a' as u64) * 8
            + (s[3] as u64 - '1' as u64);
        res *= 5;
        if s.len() == 5 {
            res += match s[4] {
                b'n' => 1,
                b'b' => 2,
                b'r' => 3,
                b'q' => 4,
                c => panic!("unexpected move char {}", c as char),
            };
        }
        res
    }

    fn depth_dump(
        &mut self,
        depth: usize,
        board: &mut T::Board,
        spec: &DepthSpec,
        ctx: &mut DepthCtx,
    ) {
        let t = &self.test;

        if depth == 0 {
            if self.options.dump_trace_chains {
                writeln!(self.writer, "cur-chain: {}", ctx.chain).unwrap();
            }

            if !self.options.attack_heatmaps {
                assert!(!ctx.spec.with_heatmaps);
            }
            if ctx.spec.with_heatmaps {
                for color in [true, false] {
                    for y in ('1'..='8').rev() {
                        let mut data: u64 = 0;
                        for x in 'a'..='h' {
                            data = data.wrapping_mul(2);
                            data = data.wrapping_add(t.is_attacked(board, color, x, y) as u64);
                        }
                        ctx.grow_hash(data);
                    }
                }
            }

            ctx.grow_hash(t.is_check(board) as u64);

            return;
        }

        let moves = t.generate_moves(board);
        let count = t.move_count(&moves);
        let mut move_ord: ArrayVec<(u64, usize), 256> = ArrayVec::new();
        for i in 0..count {
            let mv = t.get_move(&moves, i);
            move_ord.push((self.move_hash(mv), i));
        }
        move_ord.sort();

        ctx.grow_hash(519365819);
        for (val, idx) in move_ord {
            let t = &self.test;
            let mv = t.get_move(&moves, idx);
            let old_len = ctx.chain.len();
            let u = t.make_move(board, mv);
            if t.is_last_move_legal(board) {
                if self.options.dump_trace_chains {
                    ctx.chain += &(t.move_str(mv) + " ");
                }
                ctx.grow_hash(val);
                self.depth_dump(depth - 1, board, spec, ctx);
            }
            let t = &self.test;
            ctx.chain.truncate(old_len);
            t.unmake_move(board, mv, &u);
        }
        ctx.grow_hash(15967534195);
    }

    pub fn run_many<R: BufRead>(&mut self, reader: &mut R) {
        for line in reader.lines() {
            let line = line.expect("i/o error");
            let line = line.trim_end();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            self.run_one(line);
        }
    }

    pub fn run_one(&mut self, fen: &str) {
        let t = &self.test;

        let mut board = t.board_from_fen(fen);
        writeln!(self.writer, "fen: {}", fen).unwrap();
        if self.options.run_self_test {
            t.run_self_test(&board);
        }

        let moves = t.generate_moves(&board);
        let move_strs = self.move_strings(&mut board, &moves);

        writeln!(self.writer, "moves: [").unwrap();
        for s in &move_strs {
            writeln!(self.writer, "  {}", s).unwrap();
        }
        writeln!(self.writer, "]").unwrap();

        let is_check = match t.is_check(&board) {
            true => "true",
            false => "false",
        };
        writeln!(self.writer, "check?: {}", is_check).unwrap();

        if self.options.attack_heatmaps {
            for color in [true, false] {
                let color_str = match color {
                    true => "white",
                    false => "black",
                };
                writeln!(self.writer, "{}-heatmap: [", color_str).unwrap();
                for y in ('1'..='8').rev() {
                    write!(self.writer, "  ").unwrap();
                    for x in 'a'..='h' {
                        match t.is_attacked(&board, color, x, y) {
                            true => write!(self.writer, "#").unwrap(),
                            false => write!(self.writer, ".").unwrap(),
                        };
                    }
                    writeln!(self.writer).unwrap();
                }
                writeln!(self.writer, "]").unwrap();
            }
        }

        if self.options.run_self_test {
            for i in 0..t.move_count(&moves) {
                let mv = t.get_move(&moves, i);
                let u = t.make_move(&mut board, mv);
                if t.is_last_move_legal(&board) {
                    t.run_self_test(&board);
                }
                t.unmake_move(&mut board, mv, &u);
            }
        }

        let mut specs = vec![
            DepthSpec {
                depth: 1,
                with_heatmaps: self.options.attack_heatmaps,
            },
            DepthSpec {
                depth: 2,
                with_heatmaps: false,
            },
        ];
        if self.options.big_depth {
            if self.options.attack_heatmaps {
                specs.push(DepthSpec {
                    depth: 2,
                    with_heatmaps: true,
                });
            }
            specs.push(DepthSpec {
                depth: 3,
                with_heatmaps: false,
            });
        }

        for spec in &specs {
            let mut ctx = DepthCtx::new(spec);
            self.depth_dump(spec.depth, &mut board, spec, &mut ctx);
            writeln!(self.writer, "depth-dump-at-{}: {}", spec.name(), ctx.hash).unwrap();
        }

        writeln!(self.writer).unwrap();
    }
}
