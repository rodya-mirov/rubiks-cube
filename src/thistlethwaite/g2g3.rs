use std::collections::VecDeque;

use ahash::HashSet;

use crate::corner_position_state::CubeCornerPositions;
use crate::cube::Cube;
use crate::dfs_util;
use crate::edge_position_state::CubeEdgePositions;
use crate::heuristic_caches::{Heuristic, HeuristicCache};
use crate::moves::{Amt, ApplyMove, Dir, FullMove, ALL_DIRS};
use crate::total_position_state::CubePositions;

const G2_FREE_DIRS: [Dir; 2] = [Dir::L, Dir::R];
const G2_DOUBLE_DIRS: [Dir; 4] = [Dir::U, Dir::D, Dir::F, Dir::B];

pub struct G2toG3Cache {
    edges: HashSet<CubeEdgePositions>,
    edge_heuristic: HeuristicCache<CubeEdgePositions>,
    corners: HashSet<CubeCornerPositions>,
    corner_heuristic: HeuristicCache<CubeCornerPositions>,
}

impl G2toG3Cache {
    pub fn initialize() -> Self {
        let start: CubePositions = CubePositions::make_solved();

        let mut full_states: HashSet<CubePositions> = HashSet::default();
        full_states.insert(start.clone());

        let mut to_process = VecDeque::new();
        to_process.push_back(start);

        while let Some(next) = to_process.pop_front() {
            for dir in ALL_DIRS {
                let fm = FullMove { dir, amt: Amt::Two };

                let applied = next.clone().apply(fm);
                if full_states.insert(applied.clone()) {
                    to_process.push_back(applied);
                }
            }
        }

        let mut edge_states = HashSet::default();
        let mut corner_states = HashSet::default();

        for state in full_states.iter().cloned() {
            edge_states.insert(state.edges);
            corner_states.insert(state.corners);
        }

        // This just sort of makes me happy to double-check; it means that a position is in G3
        // if and only if the edge and corner states are
        assert_eq!(edge_states.len() * corner_states.len(), full_states.len());

        let corner_heuristic =
            HeuristicCache::from_set(&corner_states, &G2_FREE_DIRS, &G2_DOUBLE_DIRS);

        // TODO perf: constructing the edges might actually take so long (around 600ms) this is not a good use of time
        let edge_heuristic = HeuristicCache::from_set(&edge_states, &G2_FREE_DIRS, &G2_DOUBLE_DIRS);

        G2toG3Cache {
            edges: edge_states,
            edge_heuristic,
            corners: corner_states,
            corner_heuristic,
        }
    }
}

impl Heuristic<CubePositions> for G2toG3Cache {
    fn evaluate(&self, state: &CubePositions) -> usize {
        let e = self.edge_heuristic.evaluate(&state.edges);
        let c = self.corner_heuristic.evaluate(&state.corners);
        e.max(c)
    }
}

/// Given a cube in G2, solve to G3
#[inline(never)]
pub fn solve_to_g3(cube: &Cube, cache: &G2toG3Cache) -> Vec<FullMove> {
    let pos_state = CubePositions::from_cube(cube);

    const MAX_MOVES: usize = 14;

    dfs_util::solve(
        pos_state,
        &G2_FREE_DIRS,
        &G2_DOUBLE_DIRS,
        |s| cache.edges.contains(&s.edges) && cache.corners.contains(&s.corners),
        cache,
        MAX_MOVES,
    )
}
