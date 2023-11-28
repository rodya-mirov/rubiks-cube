use full_state::G1State;

use crate::cube::Cube;
use crate::moves::{Dir, FullMove};
use crate::thistlethwaite::dfs_util;

mod full_state;

const FREE_DIRS: [Dir; 4] = [Dir::B, Dir::F, Dir::L, Dir::R];

/// Solve to G2. Assumes the input is already in G1, results not guaranteed if not.
pub fn solve_to_g2(cube: &Cube) -> Vec<FullMove> {
    // Apparently you can solve G1 -> G2 in 10 moves, idk
    const MAX_MOVES: usize = 10;

    dfs_util::solve(
        cube,
        &FREE_DIRS,
        &[Dir::U, Dir::D],
        G1State::from_cube,
        |s| s.is_solved(),
        |_| 0,
        MAX_MOVES,
    )
}
