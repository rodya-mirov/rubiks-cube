use crate::cube::Facelet;
use crate::moves::{parse_many, to_nice_str, CanMove};
use crate::shadow::to_white_cross;
use crate::timed::timed;

mod cube;
mod moves;
mod shadow;
mod solve;
mod timed;

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

    let input = "U2 F L D L' D' F'";

    println!("With green front, yellow top; applying moves {input}");

    let bot_messed =
        cube::Cube::make_solved(Facelet::Green, Facelet::Yellow).apply_many(&parse_many(input));

    let wc_solution = timed("Solving WC", || solve::solve_wc(bot_messed.clone()));

    println!(
        "Found a solution for the white cross: {}",
        to_nice_str(&wc_solution)
    );

    // after applying the solution, the WC should be solved, but the whole cube probably is not
    let altered = bot_messed.apply_many(&wc_solution);
    let altered_mask = to_white_cross(altered.clone());

    assert!(!altered.is_solved());
    assert!(altered_mask.is_solved());
    assert!(
        wc_solution.len() < parse_many(input).len(),
        "Solving WC is simpler than solving the whole cube"
    )
}
