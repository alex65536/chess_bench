use chess_bench::{impls, selftest::Tester, Test};
use hex_literal::hex;
use sha2::{Digest, Sha256};

const INPUT_DATA: &'static str = include_str!("boards.fen");
const OUTPUT_HASH: [u8; 32] =
    hex!("1ac232af9c1ede66b0cf423c87838324b09d178a5721b2c4ded7d87540a96318");

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
