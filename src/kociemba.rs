//! Set of functionality corresponding to a Kociemba 2-phase algorithm solution
//!
//! See for instance: http://kociemba.org/twophase.htm
//!
//! Fundamentally this is a simplification of Thistlethwaite's 4-phase algorithm; you isolate
//! a subgroup generated by <L, R, F2, B2, U2, D2>, and observe that a cube is in that subgroup
//! if and only if its orientations are correct and the mid-slice edge cubelets are actually in
//! the mid-slice.
//!
//! So you have a two-phase algorithm:
//! 1.  Optimally get to the invariant of interest (orientation solved, mid-slice edge cubelets set)
//! 2.  Solve the cube
//!
//! In general this is a little slower than Thistlethwaite, but generates better solutions. In
//! general they may not be optimal though; getting optimally to the subgroup, then staying inside
//! it optimally to solved, may not be the shortest path.
//!
//! To set up some notation (and distinguish it from the other), we'll see:
//!
//!     H0  -- all configurations. Generated by <L, R, F, B, U, D>
//!     H1  -- everything where the orientations are fixed and the mid-slice edges are in mid-slice.
//!             Generated by <L, R, F2, B2, U2, D2>
//!     H2  -- solved cube. Generated by <>
//!
//! You can modify this to improve on the solutions but that's not what we're doing in this module.

pub use h0h1::{solve_to_h1, H0toH1Cache};
pub use h1h2::{solve_to_h2, H1toH2Cache};

use crate::cube::Cube;
use crate::moves::{ApplyMove, FullMove};

mod h0h1;
mod h1h2;

pub struct KociembaCaches {
    pub h0h1cache: H0toH1Cache,
    pub h1h2cache: H1toH2Cache,
}

impl KociembaCaches {
    pub fn initialize() -> Self {
        Self {
            h0h1cache: H0toH1Cache::initialize(),
            h1h2cache: H1toH2Cache::initialize(),
        }
    }
}

#[allow(unused)]
pub fn full_solve(cube: &Cube, cache: &KociembaCaches) -> Vec<FullMove> {
    let g0_solved = cube.clone();
    let g1_solution = solve_to_h1(cube, &cache.h0h1cache);
    let g1_solved = g0_solved.clone().apply_many(&g1_solution);
    let g2_solution = solve_to_h2(&g1_solved, &cache.h1h2cache);

    let mut full_solution = g1_solution;
    full_solution.extend(g2_solution);
    full_solution
}
