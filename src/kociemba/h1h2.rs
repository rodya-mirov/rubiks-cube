use crate::corner_position_state::CubeCornerPositions;
use crate::cube::Cube;
use crate::dfs_util;
use crate::edge_position_state::CubeEdgePositions;
use crate::heuristic_caches::{CappedHeuristicCache, Heuristic, HeuristicCache};
use crate::moves::{CanMove, Dir, FullMove};

const FREE_DIRS: [Dir; 2] = [Dir::L, Dir::R];
const HALF_DIRS: [Dir; 4] = [Dir::U, Dir::D, Dir::F, Dir::B];

pub fn solve_to_h2(cube: &Cube, cache: &H1toH2Cache) -> Vec<FullMove> {
    let start_state = RunningState::from_cube(cube);

    // i have no idea
    const MAX_MOVES: usize = 18;

    dfs_util::solve(
        start_state,
        &FREE_DIRS,
        &HALF_DIRS,
        |s| s.is_solved(),
        cache,
        MAX_MOVES,
    )
}

pub struct H1toH2Cache {
    edge_pos: HeuristicCache<CubeEdgePositions>,
    corner_pos: HeuristicCache<CubeCornerPositions>,
    total_pos: CappedHeuristicCache<TotalState>,
}

type TotalState = (CubeEdgePositions, CubeCornerPositions);

const TOTAL_STATE_FUEL: usize = 7;

impl H1toH2Cache {
    pub fn initialize() -> Self {
        H1toH2Cache {
            edge_pos: HeuristicCache::from_goal(
                CubeEdgePositions::make_solved(),
                &FREE_DIRS,
                &HALF_DIRS,
            ),
            corner_pos: HeuristicCache::from_goal(
                CubeCornerPositions::make_solved(),
                &FREE_DIRS,
                &HALF_DIRS,
            ),
            total_pos: CappedHeuristicCache::from_goal(
                (
                    CubeEdgePositions::make_solved(),
                    CubeCornerPositions::make_solved(),
                ),
                &FREE_DIRS,
                &HALF_DIRS,
                TOTAL_STATE_FUEL,
            ),
        }
    }
}

impl Heuristic<RunningState> for H1toH2Cache {
    fn evaluate(&self, s: &RunningState) -> usize {
        let edges = self.edge_pos.evaluate(&s.edge_pos);
        let corners = self.corner_pos.evaluate(&s.corner_pos);
        let total = self
            .total_pos
            .evaluate(&(s.edge_pos.clone(), s.corner_pos.clone()));

        edges.max(corners).max(total)
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct RunningState {
    edge_pos: CubeEdgePositions,
    corner_pos: CubeCornerPositions,
}

impl RunningState {
    fn from_cube(cube: &Cube) -> RunningState {
        Self {
            edge_pos: CubeEdgePositions::from_cube(cube),
            corner_pos: CubeCornerPositions::from_cube(cube),
        }
    }

    fn is_solved(&self) -> bool {
        self.edge_pos == CubeEdgePositions::make_solved()
            && self.corner_pos == CubeCornerPositions::make_solved()
    }
}

impl CanMove for RunningState {
    fn r(self) -> Self {
        Self {
            edge_pos: self.edge_pos.r(),
            corner_pos: self.corner_pos.r(),
        }
    }

    fn l(self) -> Self {
        Self {
            edge_pos: self.edge_pos.l(),
            corner_pos: self.corner_pos.l(),
        }
    }

    fn u(self) -> Self {
        Self {
            edge_pos: self.edge_pos.u(),
            corner_pos: self.corner_pos.u(),
        }
    }

    fn d(self) -> Self {
        Self {
            edge_pos: self.edge_pos.d(),
            corner_pos: self.corner_pos.d(),
        }
    }

    fn b(self) -> Self {
        Self {
            edge_pos: self.edge_pos.b(),
            corner_pos: self.corner_pos.b(),
        }
    }

    fn f(self) -> Self {
        Self {
            edge_pos: self.edge_pos.f(),
            corner_pos: self.corner_pos.f(),
        }
    }
}
