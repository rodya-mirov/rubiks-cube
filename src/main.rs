use crate::cube::Facelet;
use crate::moves::{parse_many, to_nice_str, CanMove};
use crate::shadow::to_white_cross;
use crate::timed::timed;

mod cube;
mod moves;
mod shadow;
mod solve;
mod thistlethwaite;
mod timed;

fn thistle_stuff() {
    let input = "R U F R U F";
    println!();
    println!("For thistlethwaite, starting with scramble: {}", input);

    let thistle_problem =
        cube::Cube::make_solved(Facelet::Green, Facelet::Yellow).apply_many(&parse_many(input));

    let g1_solution = timed("G0 to G1", || thistlethwaite::solve_to_g1(&thistle_problem));

    println!("Found a solution for the G1: {}", to_nice_str(&g1_solution));

    let g1_cube = thistle_problem.clone().apply_many(&g1_solution);
    assert!(
        !g1_cube.is_solved(),
        "solving to g1 shouldn't, like, solve it"
    );

    let g2_solution = timed("G1 to G0", || thistlethwaite::solve_to_g2(&g1_cube));

    println!("Found a solution for the G2: {}", to_nice_str(&g2_solution));

    let g2_cube = g1_cube.clone().apply_many(&g2_solution);
    assert!(
        !g2_cube.is_solved(),
        "solving to g2 shouldn't, like, solve it"
    )
}

#[allow(unused)]
fn wc_stuff() {
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
    );
}

fn main() {
    // this is just debug stuff, uncomment to allow
    // wc_stuff();

    thistle_stuff();
}
