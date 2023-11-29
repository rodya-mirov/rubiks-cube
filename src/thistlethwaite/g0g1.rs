use crate::cube::Cube;
use crate::dfs_util;
use crate::edge_orientation_state::EdgeOrientationState;
use crate::heuristic_caches::HeuristicCache;
use crate::moves::{Dir, FullMove};

/// Invariants from a cube in G0 to describe what's left to get to G1

const ALL_DIRS: [Dir; 6] = [Dir::U, Dir::D, Dir::B, Dir::F, Dir::L, Dir::R];

pub struct G0toG1Cache {
    heuristic_cache: HeuristicCache<EdgeOrientationState>,
}

impl G0toG1Cache {
    pub fn initialize() -> Self {
        G0toG1Cache {
            heuristic_cache: HeuristicCache::from_goal(
                EdgeOrientationState::make_solved(),
                &ALL_DIRS,
                &[],
            ),
        }
    }
}

/// Solves a given cube to G1. Assumes the input is in G0 (that is, solvable).
pub fn solve_to_g1(cube: &Cube, cache: &G0toG1Cache) -> Vec<FullMove> {
    // note: this should be 7? i'm not sure why i need to bump it to 8? it doesn't really matter,
    // it's still finding correct answers, but there's something funny here
    const MAX_MOVES: usize = 8;

    dfs_util::solve(
        EdgeOrientationState::from_cube(cube),
        &ALL_DIRS,
        &[],
        |s| s.is_solved(),
        |s| cache.heuristic_cache.evaluate(s),
        MAX_MOVES,
    )
}
