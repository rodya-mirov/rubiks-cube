use crate::corner_orientation_state::CornerOrientationState;
use crate::cube::Cube;
use crate::dfs_util;
use crate::edge_orientation_state::EdgeOrientationState;
use crate::edge_slice_state::EdgeMidSliceState;
use crate::heuristic_caches::{CappedHeuristicCache, Heuristic, HeuristicCache};
use crate::moves::{CanMove, FullMove, ALL_DIRS};

pub fn solve_to_h1(cube: &Cube, cache: &H0toH1Cache) -> Vec<FullMove> {
    let start_state = RunningState::from_cube(cube);

    // i have no idea
    const MAX_MOVES: usize = 17;

    dfs_util::solve(
        start_state,
        &ALL_DIRS,
        &[],
        |s| s.is_solved(),
        cache,
        MAX_MOVES,
    )
}

type TotalState = (
    EdgeOrientationState,
    (CornerOrientationState, EdgeMidSliceState),
);

const TOTAL_STATE_MAX_FUEL: usize = 6;

pub struct H0toH1Cache {
    edge_orientation: HeuristicCache<EdgeOrientationState>,
    corner_orientation: HeuristicCache<CornerOrientationState>,
    edge_slice_state: HeuristicCache<EdgeMidSliceState>,
    total_state: CappedHeuristicCache<TotalState>,
}

impl H0toH1Cache {
    pub fn initialize() -> Self {
        H0toH1Cache {
            edge_orientation: HeuristicCache::from_goal(
                EdgeOrientationState::make_solved(),
                &ALL_DIRS,
                &[],
            ),
            corner_orientation: HeuristicCache::from_goal(
                CornerOrientationState::solved(),
                &ALL_DIRS,
                &[],
            ),
            edge_slice_state: HeuristicCache::from_goal(
                EdgeMidSliceState::solved(),
                &ALL_DIRS,
                &[],
            ),
            total_state: CappedHeuristicCache::from_goal(
                (
                    EdgeOrientationState::make_solved(),
                    (
                        CornerOrientationState::solved(),
                        EdgeMidSliceState::solved(),
                    ),
                ),
                &ALL_DIRS,
                &[],
                TOTAL_STATE_MAX_FUEL,
            ),
        }
    }
}

impl Heuristic<RunningState> for H0toH1Cache {
    fn evaluate(&self, s: &RunningState) -> usize {
        let slice = self.edge_slice_state.evaluate(&s.mid_slice);
        let edges = self.edge_orientation.evaluate(&s.edge_or);
        let corners = self.corner_orientation.evaluate(&s.corner_or);

        // TODO: can probably improve this so we don't need to do the clones?
        let total_state = self.total_state.evaluate(&(
            s.edge_or.clone(),
            (s.corner_or.clone(), s.mid_slice.clone()),
        ));

        slice.max(edges).max(corners).max(total_state)
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct RunningState {
    edge_or: EdgeOrientationState,
    corner_or: CornerOrientationState,
    mid_slice: EdgeMidSliceState,
}

impl RunningState {
    fn from_cube(cube: &Cube) -> RunningState {
        Self {
            edge_or: EdgeOrientationState::from_cube(cube),
            corner_or: CornerOrientationState::from_cube(cube),
            mid_slice: EdgeMidSliceState::from_cube(cube),
        }
    }

    fn is_solved(&self) -> bool {
        self.edge_or.is_solved() && self.corner_or.is_solved() && self.mid_slice.is_solved()
    }
}

impl CanMove for RunningState {
    fn r(self) -> Self {
        Self {
            edge_or: self.edge_or.r(),
            corner_or: self.corner_or.r(),
            mid_slice: self.mid_slice.r(),
        }
    }

    fn l(self) -> Self {
        Self {
            edge_or: self.edge_or.l(),
            corner_or: self.corner_or.l(),
            mid_slice: self.mid_slice.l(),
        }
    }

    fn u(self) -> Self {
        Self {
            edge_or: self.edge_or.u(),
            corner_or: self.corner_or.u(),
            mid_slice: self.mid_slice.u(),
        }
    }

    fn d(self) -> Self {
        Self {
            edge_or: self.edge_or.d(),
            corner_or: self.corner_or.d(),
            mid_slice: self.mid_slice.d(),
        }
    }

    fn b(self) -> Self {
        Self {
            edge_or: self.edge_or.b(),
            corner_or: self.corner_or.b(),
            mid_slice: self.mid_slice.b(),
        }
    }

    fn f(self) -> Self {
        Self {
            edge_or: self.edge_or.f(),
            corner_or: self.corner_or.f(),
            mid_slice: self.mid_slice.f(),
        }
    }
}
