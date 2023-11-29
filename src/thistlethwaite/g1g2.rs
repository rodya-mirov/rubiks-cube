use crate::corner_orientation_state::CornersState;
use crate::cube::Cube;
use crate::dfs_util;
use crate::edge_slice_state::EdgesState;
use crate::heuristic_caches::HeuristicCache;
use crate::moves::{CanMove, Dir, FullMove};

const FREE_DIRS: [Dir; 4] = [Dir::B, Dir::F, Dir::L, Dir::R];
const HALF_DIRS: [Dir; 2] = [Dir::U, Dir::D];

/// Invariants from a cube in G0 to describe what's left to get to G2
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct G1State {
    pub edges: EdgesState,
    pub corners: CornersState,
}

impl G1State {
    pub fn is_solved(&self) -> bool {
        self.edges.is_solved() && self.corners.is_solved()
    }

    pub fn from_cube(cube: &Cube) -> G1State {
        G1State {
            edges: EdgesState::from_cube(cube),
            corners: CornersState::from_cube(cube),
        }
    }
}

impl CanMove for G1State {
    fn r(self) -> Self {
        Self {
            corners: self.corners.r(),
            edges: self.edges.r(),
        }
    }

    fn l(self) -> Self {
        Self {
            corners: self.corners.l(),
            edges: self.edges.l(),
        }
    }

    fn u(self) -> Self {
        panic!("U not supported")
    }

    fn u_two(self) -> Self {
        Self {
            corners: self.corners.u_two(),
            edges: self.edges.u_two(),
        }
    }

    fn d(self) -> Self {
        panic!("D not supported")
    }

    fn d_two(self) -> Self {
        Self {
            corners: self.corners.d_two(),
            edges: self.edges.d_two(),
        }
    }

    fn b(self) -> Self {
        Self {
            corners: self.corners.b(),
            edges: self.edges.b(),
        }
    }

    fn f(self) -> Self {
        Self {
            corners: self.corners.f(),
            edges: self.edges.f(),
        }
    }
}

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
