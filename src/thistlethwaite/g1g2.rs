use full_state::G1State;

use crate::cube::Cube;
use crate::moves::{Dir, FullMove};
use crate::thistlethwaite::dfs_util;

mod full_state;

const FREE_DIRS: [Dir; 4] = [Dir::B, Dir::F, Dir::L, Dir::R];

/// Solve to G2. Assumes the input is already in G1, results not guaranteed if not.
pub fn solve_to_g2(cube: &Cube) -> Vec<FullMove> {
    const MAX_MOVES: usize = 10;

    dfs_util::solve(
        G1State::from_cube(cube),
        &FREE_DIRS,
        &[Dir::U, Dir::D],
        |s| s.is_solved(),
        |_| 0,
        MAX_MOVES,
    )
}
