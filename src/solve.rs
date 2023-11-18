//! Module for solving cubes in various ways

use crate::cube::{AmbigFacelet, Cube};
use crate::moves::{Amt, CanMove, Dir, FullMove};
use crate::shadow::to_white_cross;

type MaskedCube = Cube<AmbigFacelet>;

const ALL_DIRS: [Dir; 6] = [Dir::U, Dir::D, Dir::B, Dir::F, Dir::L, Dir::R];
const ALL_AMTS: [Amt; 3] = [Amt::One, Amt::Two, Amt::Rev];

pub fn solve_wc(cube: Cube) -> Vec<FullMove> {
    let mask = to_white_cross(cube.clone());

    // iterative-deepening DFS; returns true if it found a solution, or false if not
    fn ida(cube: MaskedCube, running: &mut Vec<FullMove>, max_depth: usize) -> bool {
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
                running.push(fm);

                let found_solution = ida(next, running, max_depth);

                if found_solution {
                    return true;
                }

                running.pop();
            }
        }

        false
    }

    for max_depth in 0..8 {
        let mut attempt = Vec::with_capacity(max_depth);

        let found = ida(mask.clone(), &mut attempt, max_depth);

        if found {
            return attempt;
        }
    }

    panic!("idk dude couldn't solve it")
}
