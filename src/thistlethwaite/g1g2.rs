use full_state::G1State;

use crate::cube::Cube;
use crate::moves::{Amt, Dir, FullMove};
use crate::thistlethwaite::dfs_util;

mod full_state;

const FREE_DIRS: [Dir; 4] = [Dir::B, Dir::F, Dir::L, Dir::R];
const HALF_DIRS: [Dir; 2] = [Dir::U, Dir::D];
const ALL_AMTS: [Amt; 3] = [Amt::One, Amt::Two, Amt::Rev];

/// Solve to G2. Assumes the input is already in G1, results not guaranteed if not.
pub fn solve_to_g2(cube: &Cube) -> Vec<FullMove> {
    let edge_heuristic = full_state::EdgeHeuristic::initialize();
    let corner_heuristic = full_state::CornerHeuristic::initialize();

    const MAX_MOVES: usize = 10;

    // TODO perf: can probably have a corners-specific and edges-specific heuristic for A* search
    dfs_util::solve(
        G1State::from_cube(cube),
        &FREE_DIRS,
        &HALF_DIRS,
        |s| s.is_solved(),
        |s| {
            edge_heuristic
                .evaluate(&s.edges)
                .max(corner_heuristic.evaluate(&s.corners))
        },
        MAX_MOVES,
    )
}
