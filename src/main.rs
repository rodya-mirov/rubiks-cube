use std::time::Instant;

use crate::corner_orientation_state::CornerOrientationState;
use itertools::concat;

use crate::cube::Facelet;
use crate::edge_orientation_state::EdgeOrientationState;
use crate::edge_slice_state::EdgeMidSliceState;
use crate::moves::{parse_many, to_nice_str, ApplyMove, FullMove};
use crate::shadow::to_white_cross;
use crate::timed::timed;

mod corner_orientation_state;
mod corner_position_state;
mod cube;
mod dfs_util;
mod edge_orientation_state;
mod edge_position_state;
mod edge_slice_state;
mod heuristic_caches;
mod kociemba;
mod moves;
mod shadow;
mod solve;
mod thistlethwaite;
mod timed;
mod total_position_state;

fn kociemba_stuff(input: &str, kociemba_cache: &kociemba::KociembaCaches) {
    let start = Instant::now();

    let kociemba_problem =
        cube::Cube::make_solved(Facelet::Green, Facelet::Yellow).apply_many(&parse_many(input));

    let (h1_dur, h1_solution) =
        timed(|| kociemba::solve_to_h1(&kociemba_problem, &kociemba_cache.h0h1cache));

    let h1_cube = kociemba_problem.clone().apply_many(&h1_solution);

    assert!(
        EdgeOrientationState::from_cube(&h1_cube).is_solved(),
        "Should be edge orientated"
    );
    assert!(
        CornerOrientationState::from_cube(&h1_cube).is_solved(),
        "Should be corner orientated"
    );
    assert!(
        EdgeMidSliceState::from_cube(&h1_cube).is_solved(),
        "Should be mid-slice satisfied"
    );

    let (h2_dur, h2_solution) =
        timed(|| kociemba::solve_to_h2(&h1_cube, &kociemba_cache.h1h2cache));

    let h2_cube = h1_cube.clone().apply_many(&h2_solution);

    let total_solution: Vec<FullMove> = concat([h1_solution.clone(), h2_solution.clone()]);

    println!(
        "Total solution has {}+{} == {} moves: {}",
        h1_solution.len(),
        h2_solution.len(),
        total_solution.len(),
        to_nice_str(&total_solution)
    );

    let timings = vec![
        (h1_dur, "H0 to H1", h1_solution.len()),
        (h2_dur, "H1 to H2", h2_solution.len()),
    ];
    let max_time = timings.iter().max().copied().unwrap();
    println!(
        "Total time was {:?}; Slowest stage was {} ({} moves) at {:?}",
        start.elapsed(),
        max_time.1,
        max_time.2,
        max_time.0
    );

    assert!(h2_cube.is_solved());
}

fn thistle_stuff(input: &str, thistle_cache: &thistlethwaite::ThistlethwaiteCaches) {
    let start = Instant::now();

    let thistle_problem =
        cube::Cube::make_solved(Facelet::Green, Facelet::Yellow).apply_many(&parse_many(input));

    let (g1_dur, g1_solution) =
        timed(|| thistlethwaite::solve_to_g1(&thistle_problem, &thistle_cache.g0g1cache));

    let g1_cube = thistle_problem.clone().apply_many(&g1_solution);

    let (g2_dur, g2_solution) =
        timed(|| thistlethwaite::solve_to_g2(&g1_cube, &thistle_cache.g1g2cache));

    let g2_cube = g1_cube.clone().apply_many(&g2_solution);

    let (g3_dur, g3_solution) =
        timed(|| thistlethwaite::solve_to_g3(&g2_cube, &thistle_cache.g2g3cache));

    let g3_cube = g2_cube.clone().apply_many(&g3_solution);

    let (g4_dur, g4_solution) =
        timed(|| thistlethwaite::solve_to_g4(&g3_cube, &thistle_cache.g3g4cache));

    let g4_cube = g3_cube.clone().apply_many(&g4_solution);

    assert!(
        g4_cube.is_solved(),
        "Cube should be solved, that's the point"
    );

    let total_solution: Vec<FullMove> = concat([
        g1_solution.clone(),
        g2_solution.clone(),
        g3_solution.clone(),
        g4_solution.clone(),
    ]);

    println!(
        "Total solution has {}+{}+{}+{} == {} moves: {}",
        g1_solution.len(),
        g2_solution.len(),
        g3_solution.len(),
        g4_solution.len(),
        total_solution.len(),
        to_nice_str(&total_solution)
    );
    let timings = vec![
        (g1_dur, "G0 to G1", g1_solution.len()),
        (g2_dur, "G1 to G2", g2_solution.len()),
        (g3_dur, "G2 to G3", g3_solution.len()),
        (g4_dur, "G3 to G4", g4_solution.len()),
    ];
    let max_time = timings.iter().max().copied().unwrap();
    println!(
        "Total time was {:?}; Slowest stage was {} ({} moves) at {:?}",
        start.elapsed(),
        max_time.1,
        max_time.2,
        max_time.0
    );
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

    let (wc_dir, wc_solution) = timed(|| solve::solve_wc(bot_messed.clone()));

    println!(
        "Found a solution for the white cross in {:?}: {}",
        wc_dir,
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

#[allow(unused)]
fn kociemba_suite() {
    let start = Instant::now();
    let kociemba_cache = kociemba::KociembaCaches::initialize();

    // currently about 809.681667ms, arguable if this is "cheating" or not
    println!("Pre-populating the caches took {:?}", start.elapsed());

    // Some notes -- I want to ensure we flex the maxima for each stage to ensure we're doing
    // as well as we can. AFAIK the max length for each stage is:
    //      G0 to G1 -- 7 moves (the superflip hits this)
    //      G1 to G2 -- 10 moves (I don't have anything close to this; best I have is 8, with the long scrambles)
    //      G2 to G3 -- 13 moves (I don't have anything close to this; best I have is 10, with R U F R U F)
    //      G3 to G4 -- 15 moves (I don't have anything close to this; best I have is 12, with R U F R U F)

    for input in [
        // some hand-made examples i invented to get the basics going
        "R U F",
        "R U F R U F",
        "R U F R U F R U F",
        "R U F R U F R U F2",
        // the "superflip"
        "U R2 F B R B2 R U2 L B2 R U' D' R2 F R' L B2 U2 F2",
        // three random scrambles i got from a scrambler
        "B U F' L U R' L' F2 D' F2 L F' R' D L' D U2 R' U2 F' D' R2 F2 B' U2",
        "L U B2 F2 D' B' R U2 F B L' R2 U2 B' F2 R' U B' D' L U' F D F2 B",
        "B' L U2 R2 L' D L U F2 D' L2 D' L' R' B D' F2 B' U B' U L' U2 L F",
    ] {
        kociemba_stuff(input, &kociemba_cache);
    }
}

#[allow(unused)]
fn thistle_suite() {
    let start = Instant::now();
    let thistle_cache = thistlethwaite::ThistlethwaiteCaches::initialize();

    // currently about 809.681667ms, arguable if this is "cheating" or not
    println!("Pre-populating the caches took {:?}", start.elapsed());

    // Some notes -- I want to ensure we flex the maxima for each stage to ensure we're doing
    // as well as we can. AFAIK the max length for each stage is:
    //      G0 to G1 -- 7 moves (the superflip hits this)
    //      G1 to G2 -- 10 moves (I don't have anything close to this; best I have is 8, with the long scrambles)
    //      G2 to G3 -- 13 moves (I don't have anything close to this; best I have is 10, with R U F R U F)
    //      G3 to G4 -- 15 moves (I don't have anything close to this; best I have is 12, with R U F R U F)

    for input in [
        // some hand-made examples i invented to get the basics going
        // Total time was 43.875µs; Slowest stage was G2 to G3 (2 moves) at 20.958µs
        "R U F",
        // Total time was 6.927375ms; Slowest stage was G3 to G4 (12 moves) at 6.242792ms
        "R U F R U F",
        // Total time was 5.0325ms; Slowest stage was G3 to G4 (11 moves) at 2.397167ms
        "R U F R U F R U F",
        // Total time was 1.057542ms; Slowest stage was G3 to G4 (11 moves) at 870.75µs
        "R U F R U F R U F2",
        // the "superflip"
        // Total time was 992.125µs; Slowest stage was G3 to G4 (11 moves) at 955.708µs
        "U R2 F B R B2 R U2 L B2 R U' D' R2 F R' L B2 U2 F2",
        // three random scrambles i got from a scrambler
        // Total time was 4.823542ms; Slowest stage was G3 to G4 (11 moves) at 2.645959ms
        "B U F' L U R' L' F2 D' F2 L F' R' D L' D U2 R' U2 F' D' R2 F2 B' U2",
        // Total time was 694.125µs; Slowest stage was G2 to G3 (8 moves) at 289.667µs
        "L U B2 F2 D' B' R U2 F B L' R2 U2 B' F2 R' U B' D' L U' F D F2 B",
        // Total time was 3.845375ms; Slowest stage was G1 to G2 (8 moves) at 2.321916ms
        "B' L U2 R2 L' D L U F2 D' L2 D' L' R' B D' F2 B' U B' U L' U2 L F",
    ] {
        thistle_stuff(input, &thistle_cache);
    }
}

fn big_suite() {
    let start = Instant::now();
    let thistle_cache = thistlethwaite::ThistlethwaiteCaches::initialize();

    // currently about 758ms, arguable if this is "cheating" or not
    println!(
        "Pre-populating the thistlethwaite caches took {:?}",
        start.elapsed()
    );

    let start = Instant::now();

    let kociemba_cache = kociemba::KociembaCaches::initialize();

    // currently about 1332ms, arguable if this is "cheating" or not
    println!(
        "Pre-populating the kociemba caches took {:?}",
        start.elapsed()
    );

    // Some notes -- I want to ensure we flex the maxima for each stage to ensure we're doing
    // as well as we can. AFAIK the max length for each stage is:
    //      G0 to G1 -- 7 moves (the superflip hits this)
    //      G1 to G2 -- 10 moves (I don't have anything close to this; best I have is 8, with the long scrambles)
    //      G2 to G3 -- 13 moves (I don't have anything close to this; best I have is 10, with R U F R U F)
    //      G3 to G4 -- 15 moves (I don't have anything close to this; best I have is 12, with R U F R U F)
    //
    // Then for Kociemba worst case I'm not sure; worst case from the above is 17 to H1 and 28 to H2
    // but I think the truth is much, much lower than that. (Well, I know it is, but I don't know
    // how low).

    // Benchmarks to follow; first entry is Thistlethwaite, second is Kociemba (two-phase)

    for input in [
        // some hand-made examples i invented to get the basics going
        // Total time was 43.875µs; Slowest stage was G2 to G3 (2 moves) at 20.958µs
        // Total time was 42.625µs; Slowest stage was H1 to H2 (2 moves) at 28.208µs
        "R U F",
        // Total time was 6.927375ms; Slowest stage was G3 to G4 (12 moves) at 6.242792ms
        // Total time was 35.084µs; Slowest stage was H0 to H1 (5 moves) at 21.833µs
        "R U F R U F",
        // Total time was 5.0325ms; Slowest stage was G3 to G4 (11 moves) at 2.397167ms
        // Total time was 113.444625ms; Slowest stage was H1 to H2 (15 moves) at 113.38775ms
        "R U F R U F R U F",
        // Total time was 1.057542ms; Slowest stage was G3 to G4 (11 moves) at 870.75µs
        // Total time was 86.709µs; Slowest stage was H0 to H1 (8 moves) at 71.625µs
        "R U F R U F R U F2",
        // the "superflip"
        // Total time was 992.125µs; Slowest stage was G3 to G4 (11 moves) at 955.708µs
        // Total time was 10.012333ms; Slowest stage was H1 to H2 (13 moves) at 8.418791ms
        "U R2 F B R B2 R U2 L B2 R U' D' R2 F R' L B2 U2 F2",
        // three random scrambles i got from a scrambler
        // Total time was 4.823542ms; Slowest stage was G3 to G4 (11 moves) at 2.645959ms
        // Total time was 44.424333ms; Slowest stage was H1 to H2 (15 moves) at 42.236541ms
        "B U F' L U R' L' F2 D' F2 L F' R' D L' D U2 R' U2 F' D' R2 F2 B' U2",
        // Total time was 694.125µs; Slowest stage was G2 to G3 (8 moves) at 289.667µs
        // Total time was 2.935959ms; Slowest stage was H1 to H2 (12 moves) at 2.896625ms
        "L U B2 F2 D' B' R U2 F B L' R2 U2 B' F2 R' U B' D' L U' F D F2 B",
        // Total time was 3.845375ms; Slowest stage was G1 to G2 (8 moves) at 2.321916ms
        // Total time was 45.472833ms; Slowest stage was H1 to H2 (15 moves) at 45.114834ms
        "B' L U2 R2 L' D L U F2 D' L2 D' L' R' B D' F2 B' U B' U L' U2 L F",
    ] {
        println!("Operating on scramble: {}", input);

        println!("Thistlethwaite:");
        thistle_stuff(input, &thistle_cache);

        println!("Kociemba:");
        kociemba_stuff(input, &kociemba_cache);
        println!();
    }
}

fn main() {
    // this is just debug stuff, uncomment to allow
    // wc_stuff();

    // thistle_suite();
    // kociemba_suite();

    big_suite()
}
