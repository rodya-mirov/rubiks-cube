//! Module for solving cubes in various ways and to various degrees

use crate::cube::{AmbigFacelet, Cube};
use crate::moves::{ApplyMove, FullMove, ALL_AMTS, ALL_DIRS};
use crate::shadow::to_white_cross;

type MaskedCube = Cube<AmbigFacelet>;

pub fn solve_wc(cube: Cube) -> Vec<FullMove> {
    let mask = to_white_cross(cube.clone());

    // iterative-deepening DFS; returns true if it found a solution, or false if not
    fn ida(cube: &MaskedCube, running: &mut Vec<FullMove>, max_depth: usize) -> bool {
        if cube.is_solved() {
            return true;
        } else if running.len() >= max_depth {
            return false;
        }

        for dir in ALL_DIRS.iter().copied() {
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

    // I actually don't know the right LUB for solving WC but experimentally this should be enough?
    const MAX_MOVES: usize = 12;

    for max_depth in 0..=MAX_MOVES {
        let mut attempt = Vec::with_capacity(max_depth);

        let found = ida(&mask, &mut attempt, max_depth);

        if found {
            return attempt;
        }
    }

    panic!("idk dude couldn't solve it in {MAX_MOVES} moves, maybe i'm broken")
}
