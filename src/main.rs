use crate::cube::Facelet;
use crate::moves::{parse_many, CanMove};
use crate::shadow::to_white_cross;

mod cube;
mod moves;
mod shadow;

fn main() {
    let original = cube::Cube::make_solved(Facelet::Green, Facelet::White);

    let moves = parse_many("R L R L R L R L");
    let shuffled_solved = original.clone().apply_many(&moves);

    assert_eq!(
        shuffled_solved, original,
        "The sequence of moves was a no-op"
    );

    // this scramble does mess up the white cross
    let moves = parse_many("R L U R L U");
    let shuffled_not_solved = original.clone().apply_many(&moves);

    assert_ne!(shuffled_not_solved, original);

    let wc_masked = to_white_cross(shuffled_not_solved.clone());

    assert!(!wc_masked.is_solved());

    // this scramble is an OLL scramble; it doesn't mess up the white cross
    let top_messed = original
        .clone()
        .apply_many(&parse_many("R U2 R' U R U R' U"));

    let wc_masked = to_white_cross(shuffled_not_solved.clone());

    assert!(!top_messed.is_solved());
    assert!(wc_masked.is_solved());
}
