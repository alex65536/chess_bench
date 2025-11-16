use chess_bench::{impls, selftest::Tester, Test};
use hex_literal::hex;
use sha2::{Digest, Sha256};

const INPUT_DATA: &str = include_str!("boards.fen");
const OUTPUT_HASH: [u8; 32] =
    hex!("e0d9b6799e2bd0ae9e69f9e2757acbb3e5072fa6c667ee3e5745d90cfd2ed18d");

fn run_test<T: Test>(test: T) {
    let mut hasher = Sha256::default();
    let mut tester = Tester::new(test, Default::default(), &mut hasher);
    tester.run_many(&mut INPUT_DATA.as_bytes());
    // Here, we only verify the output hash. To debug your code, use the command-line
    // utility `selftest`.
    assert_eq!(&hasher.finalize()[..], &OUTPUT_HASH[..]);
}

#[test]
fn test_chess() {
    run_test(impls::chess::Test);
}

#[test]
fn test_owlchess() {
    run_test(impls::owlchess::Test);
}
