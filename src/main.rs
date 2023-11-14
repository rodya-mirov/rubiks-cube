use crate::moves::{parse_many, CanMove};

mod cube;
mod moves;

fn main() {
    let original = cube::Cube::make_solved();

    let moves = parse_many("R L R L R L R L");
    let shuffled_solved = original.clone().apply_many(&moves);

    assert_eq!(
        shuffled_solved, original,
        "The sequence of moves was a no-op"
    );

    let moves = parse_many("R L U R L U");
    let shuffled_not_solved = original.clone().apply_many(&moves);

    assert_ne!(shuffled_not_solved, original);

    println!("Hello, world!");
}
