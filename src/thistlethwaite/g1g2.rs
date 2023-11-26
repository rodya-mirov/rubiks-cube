use crate::cube::Cube;
use crate::moves::{Amt, CanMove, Dir, FullMove};

mod full_state;

use full_state::G1State;

const FREE_DIRS: [Dir; 4] = [Dir::B, Dir::F, Dir::L, Dir::R];
const ALL_AMTS: [Amt; 3] = [Amt::One, Amt::Two, Amt::Rev];

/// Solve to G2. Assumes the input is already in G1, results not guaranteed if not.
pub fn solve_to_g2(cube: &Cube) -> Vec<FullMove> {
    let state = G1State::from_cube(cube);

    println!("Got g0 state {state:?}");

    // iterative-deepening DFS; returns true if it found a solution, or false if not
    fn ida(cube: &G1State, running: &mut Vec<FullMove>, max_depth: usize) -> bool {
        if cube.is_solved() {
            return true;
        } else if running.len() >= max_depth {
            return false;
        }

        for dir in [Dir::U, Dir::D] {
            if running.last().map(|fm| fm.dir) == Some(dir) {
                continue;
            }

            let amt = Amt::Two;

            let fm = FullMove { amt, dir };

            // TODO: make this next bit into a macro so I can reuse it?
            let next = cube.clone().apply(fm);

            // for WC there are so many blanks there is a good chance an individual move
            // will be a no-op, so this cuts runtime by two thirds (!)
            if &next == cube {
                continue;
            }

            running.push(fm);

            let found_solution = ida(&next, running, max_depth);

            if found_solution {
                return true;
            }

            running.pop();
        }

        for dir in FREE_DIRS.iter().copied() {
            if running.last().map(|fm| fm.dir) == Some(dir) {
                continue;
            }

            for amt in ALL_AMTS.iter().copied() {
                let fm = FullMove { amt, dir };
                let next = cube.clone().apply(fm);

                // for WC there are so many blanks there is a good chance an individual move
                // will be a no-op, so this cuts runtime by two thirds (!)
                if &next == cube {
                    continue;
                }

                running.push(fm);

                let found_solution = ida(&next, running, max_depth);

                if found_solution {
                    return true;
                }

                running.pop();
            }
        }

        false
    }

    // Apparently you can solve G1 -> G2 in 10 moves, idk
    const MAX_MOVES: usize = 10;

    for max_depth in 0..=MAX_MOVES {
        let mut attempt = Vec::with_capacity(max_depth);

        let found = ida(&state, &mut attempt, max_depth);

        if found {
            return attempt;
        }
    }

    panic!("idk dude couldn't solve it in {MAX_MOVES} moves, maybe i'm broken")
}
