//! This module handles moving from G3 to G4.
//!
//! A design principle is that basically G3 is beyond annoying to describe precisely in terms of an
//! invariant that doesn't suck to compute. So here's what we're gonna do instead.
//!
//! G3 is the set of all configurations generated by half-turns: <L2, R2, U2, D2, F2, B2>. You can
//! compute a bunch of invariants, blah blah blah. But here's the real deal -- I can store a cube
//! in 54 bytes. The group G3 has exactly 663,552 elements, which comes out to about 34 megabytes
//! of storage. So fuck it -- I'll precompute the entire set, and we'll define membership in G3
//! by "membership in the set of elements in G3" and just be done.
//!
//! Perf: if that turns out to be slow we can just store permutations of the cubelets instead of
//! all the facelets, since we can't really mess up the orientations once we're in G2.

use crate::corner_position_state::CubeCornerPositions;
use crate::cube::Cube;
use crate::dfs_util;
use crate::edge_position_state::CubeEdgePositions;
use crate::heuristic_caches::{Heuristic, HeuristicCache};
use crate::moves::{FullMove, ALL_DIRS};
use crate::total_position_state::CubePositions;

pub struct G3toG4Cache {
    corner_heuristic: HeuristicCache<CubeCornerPositions>,
    edge_heuristic: HeuristicCache<CubeEdgePositions>,
}

impl G3toG4Cache {
    pub fn initialize() -> Self {
        Self {
            corner_heuristic: HeuristicCache::from_goal(
                CubeCornerPositions::make_solved(),
                &[],
                &ALL_DIRS,
            ),
            edge_heuristic: HeuristicCache::from_goal(
                CubeEdgePositions::make_solved(),
                &[],
                &ALL_DIRS,
            ),
        }
    }
}

impl Heuristic<CubePositions> for G3toG4Cache {
    fn evaluate(&self, state: &CubePositions) -> usize {
        let e = self.edge_heuristic.evaluate(&state.edges);
        let c = self.corner_heuristic.evaluate(&state.corners);
        e.max(c)
    }
}

#[inline(never)]
pub fn solve_to_g4(cube: &Cube, cache: &G3toG4Cache) -> Vec<FullMove> {
    const MAX_MOVES: usize = 16;

    dfs_util::solve(
        CubePositions::from_cube(cube),
        &[],
        &ALL_DIRS,
        |s| s.is_solved(),
        cache,
        MAX_MOVES,
    )
}
