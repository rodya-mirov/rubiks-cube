use crate::corner_position_state::CubeCornerPositions;
use crate::cube::Cube;
use crate::edge_position_state::CubeEdgePositions;
use crate::moves::CanMove;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct CubePositions {
    pub edges: CubeEdgePositions,
    pub corners: CubeCornerPositions,
}

impl CubePositions {
    pub fn is_solved(&self) -> bool {
        self == &CubePositions::make_solved()
    }

    pub fn make_solved() -> CubePositions {
        CubePositions {
            edges: CubeEdgePositions::make_solved(),
            corners: CubeCornerPositions::make_solved(),
        }
    }

    pub fn from_cube(cube: &Cube) -> Self {
        Self {
            edges: CubeEdgePositions::from_cube(cube),
            corners: CubeCornerPositions::from_cube(cube),
        }
    }
}

impl CanMove for CubePositions {
    fn r(self) -> Self {
        Self {
            edges: self.edges.r(),
            corners: self.corners.r(),
        }
    }

    fn l(self) -> Self {
        Self {
            edges: self.edges.l(),
            corners: self.corners.l(),
        }
    }

    fn u(self) -> Self {
        Self {
            edges: self.edges.u(),
            corners: self.corners.u(),
        }
    }

    fn d(self) -> Self {
        Self {
            edges: self.edges.d(),
            corners: self.corners.d(),
        }
    }

    fn b(self) -> Self {
        Self {
            edges: self.edges.b(),
            corners: self.corners.b(),
        }
    }

    fn f(self) -> Self {
        Self {
            edges: self.edges.f(),
            corners: self.corners.f(),
        }
    }
}
