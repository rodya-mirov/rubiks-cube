use full_state::G1State;

use crate::cube::Cube;
use crate::heuristic_caches::HeuristicCache;
use crate::moves::{Dir, FullMove};
use crate::thistlethwaite::dfs_util;
use crate::thistlethwaite::g1g2::full_state::{CornersState, EdgesState};

mod full_state;

const FREE_DIRS: [Dir; 4] = [Dir::B, Dir::F, Dir::L, Dir::R];
const HALF_DIRS: [Dir; 2] = [Dir::U, Dir::D];

pub struct G1toG2Cache {
    edge_heuristic: HeuristicCache<EdgesState>,
    corner_heuristic: HeuristicCache<CornersState>,
}

impl G1toG2Cache {
    pub fn initialize() -> Self {
        Self {
            edge_heuristic: HeuristicCache::from_goal(EdgesState::solved(), &FREE_DIRS, &HALF_DIRS),
            corner_heuristic: HeuristicCache::from_goal(
                CornersState::solved(),
                &FREE_DIRS,
                &HALF_DIRS,
            ),
        }
    }
}

/// Solve to G2. Assumes the input is already in G1, results not guaranteed if not.
pub fn solve_to_g2(cube: &Cube, cache: &G1toG2Cache) -> Vec<FullMove> {
    const MAX_MOVES: usize = 10;

    // TODO perf: can probably have a corners-specific and edges-specific heuristic for A* search
    dfs_util::solve(
        G1State::from_cube(cube),
        &FREE_DIRS,
        &HALF_DIRS,
        |s| s.is_solved(),
        |s| {
            cache
                .edge_heuristic
                .evaluate(&s.edges)
                .max(cache.corner_heuristic.evaluate(&s.corners))
        },
        MAX_MOVES,
    )
}
