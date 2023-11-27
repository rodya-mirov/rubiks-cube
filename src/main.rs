use crate::cube::Facelet;
use crate::moves::{parse_many, to_nice_str, ApplyMove, FullMove};
use crate::shadow::to_white_cross;
use crate::timed::timed;
use itertools::concat;
use std::time::Instant;

mod cube;
mod moves;
mod shadow;
mod solve;
mod thistlethwaite;
mod timed;

fn thistle_stuff(input: &str, thistle_cache: &thistlethwaite::PosCache) {
    let start = Instant::now();

    println!();
    println!("For thistlethwaite, starting with scramble: {}", input);

    let thistle_problem =
        cube::Cube::make_solved(Facelet::Green, Facelet::Yellow).apply_many(&parse_many(input));

    let g1_solution = timed("G0 to G1", || thistlethwaite::solve_to_g1(&thistle_problem));

    println!(
        "Found a solution for the G1 of length {}: {}",
        g1_solution.len(),
        to_nice_str(&g1_solution)
    );

    let g1_cube = thistle_problem.clone().apply_many(&g1_solution);

    let g2_solution = timed("G1 to G2", || thistlethwaite::solve_to_g2(&g1_cube));

    println!(
        "Found a solution for the G2 of length {}: {}",
        g2_solution.len(),
        to_nice_str(&g2_solution)
    );

    let g2_cube = g1_cube.clone().apply_many(&g2_solution);

    let g3_solution = timed("G2 to G3", || {
        thistlethwaite::solve_to_g3(&g2_cube, &thistle_cache)
    });

    println!(
        "Found a solution for the G3 of length {}: {}",
        g3_solution.len(),
        to_nice_str(&g3_solution)
    );

    let g3_cube = g2_cube.clone().apply_many(&g3_solution);

    let g4_solution = timed("G3 to G4", || thistlethwaite::solve_to_g4(&g3_cube));

    println!(
        "Found a solution for the G4 of length {}: {}",
        g4_solution.len(),
        to_nice_str(&g4_solution)
    );

    let g4_cube = g3_cube.clone().apply_many(&g4_solution);

    assert!(
        g4_cube.is_solved(),
        "Cube should be solved, that's the point"
    );

    let total_solution: Vec<FullMove> =
        concat([g1_solution, g2_solution, g3_solution, g4_solution]);

    println!(
        "Total solution has {} moves: {}",
        total_solution.len(),
        to_nice_str(&total_solution)
    );
    println!("Finding that solution took {:?}", start.elapsed());
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

    let thistle_cache = thistlethwaite::enumerate_g3_pos();

    for input in [
        // some hand-made examples i invented to get the basics going
        // 73 microseconds (seriously) (2/0/2/1)
        "R U F",
        // 9.62s (3/7/10/12) -- 8.88s in G2->G3 step
        "R U F R U F",
        // 154ms (4/7/7/11)
        "R U F R U F R U F",
        // 120ms (5/6/7/11)
        "R U F R U F R U F2",
        // the "superflip"
        // 265ms (7/5/8/11)
        "U R2 F B R B2 R U2 L B2 R U' D' R2 F R' L B2 U2 F2",
        // three random scrambles i got from a scrambler
        // 20.81s (5/8/11/11) -- 18.83s in G2->G3 step
        "B U F' L U R' L' F2 D' F2 L F' R' D L' D U2 R' U2 F' D' R2 F2 B' U2",
        // 1.16s (5/8/8/10)
        "L U B2 F2 D' B' R U2 F B L' R2 U2 B' F2 R' U B' D' L U' F D F2 B",
        // 4.00s (5/8/8/11)
        "B' L U2 R2 L' D L U F2 D' L2 D' L' R' B D' F2 B' U B' U L' U2 L F",
    ] {
        thistle_stuff(input, &thistle_cache);
    }
}
