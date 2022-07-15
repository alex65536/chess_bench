use clap::Parser;

use std::io::{self, BufRead};

use chess_bench::{
    impls,
    selftest::{Options, Tester},
    Test,
};

#[derive(Parser)]
#[clap(
    name = "selftest",
    version,
    about = "Runs various chess implementations on a set of positions from stdin"
)]
struct Cli {
    #[clap(value_parser)]
    #[clap(help = "Chess implementation name")]
    name: String,

    #[clap(short, long, action)]
    #[clap(help = "Dump trace chains (bigger and more time-consuming)")]
    large_chains: bool,

    #[clap(short = 'A', long, action)]
    #[clap(help = "Disable attack heatmaps")]
    no_attack_heatmaps: bool,

    #[clap(short = 'D', long, action)]
    #[clap(help = "Run on reduced depth")]
    reduced_depth: bool,
}

impl Cli {
    fn options(&self) -> Options {
        Options {
            dump_trace_chains: self.large_chains,
            attack_heatmaps: !self.no_attack_heatmaps,
            big_depth: !self.reduced_depth,
            ..Default::default()
        }
    }
}

fn run_with<R: BufRead, T: Test>(r: &mut R, t: T, opts: Options) {
    let mut stdout = io::stdout().lock();
    let mut tester = Tester::new(t, opts, &mut stdout);
    tester.run_many(r);
}

fn run<R: BufRead>(name: &str, r: &mut R, opts: Options) {
    match name {
        "chess" => run_with(r, impls::chess::Test, opts),
        "owlchess" => run_with(r, impls::owlchess::Test, opts),
        _ => panic!("unknown implementation {}", name),
    }
}

fn main() {
    let cli = Cli::parse();
    let mut stdin = io::stdin().lock();
    run(&cli.name, &mut stdin, cli.options());
}
