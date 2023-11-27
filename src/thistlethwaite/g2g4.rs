//! This module handles moving from G2 to G3, as well as G3 to G4.
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

use std::collections::{HashSet, VecDeque};
use std::time::Instant;

use ahash::RandomState;
use itertools::Itertools;

use crate::cube::{Cube, Facelet};
use crate::moves::{Amt, ApplyMove, CanMove, Dir, FullMove};

const ALL_DIRS: [Dir; 6] = [Dir::U, Dir::D, Dir::B, Dir::F, Dir::L, Dir::R];

const G2_FREE_DIRS: [Dir; 2] = [Dir::L, Dir::R];
const G2_DOUBLE_DIRS: [Dir; 4] = [Dir::U, Dir::D, Dir::F, Dir::B];
const ALL_AMTS: [Amt; 3] = [Amt::One, Amt::Two, Amt::Rev];

fn can_follow(last: Option<Dir>, next: Dir) -> bool {
    if last.is_none() {
        return true;
    }

    let last = last.unwrap();

    // can't repeat a direction, and if two directions commute, have to pick an order
    // so with no significance -- B before F, L before R, D before U
    if last == next {
        false
    } else if last == Dir::F && next == Dir::B {
        false
    } else if last == Dir::R && next == Dir::L {
        false
    } else if last == Dir::U && next == Dir::D {
        false
    } else {
        true
    }
}

/// This indicates what the cubelet "is;" or equivalently, where it belongs.
/// 12 possible values, fits in a byte.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum SideCubelet {
    UF,
    UR,
    UB,
    UL,
    FL,
    FR,
    BL,
    BR,
    DF,
    DR,
    DB,
    DL,
}

/// This indicates what the cubelet "is;" or equivalently, where it belongs.
/// 8 possible values, fits in a byte.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum CornerCubelet {
    FUL,
    FUR,
    BUL,
    BUR,
    FDL,
    FDR,
    BDL,
    BDR,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct CubePositions {
    edges: CubeEdgePositions,
    corners: CubeCornerPositions,
}

impl CubePositions {
    fn is_solved(&self) -> bool {
        self == &CubePositions::make_solved()
    }

    fn make_solved() -> CubePositions {
        CubePositions {
            edges: CubeEdgePositions::make_solved(),
            corners: CubeCornerPositions::make_solved(),
        }
    }

    fn from_cube(cube: &Cube) -> Self {
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

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct CubeEdgePositions {
    uf: SideCubelet,
    ur: SideCubelet,
    ub: SideCubelet,
    ul: SideCubelet,
    fl: SideCubelet,
    fr: SideCubelet,
    bl: SideCubelet,
    br: SideCubelet,
    df: SideCubelet,
    dr: SideCubelet,
    db: SideCubelet,
    dl: SideCubelet,
}

impl CubeEdgePositions {
    fn make_solved() -> Self {
        Self {
            uf: SideCubelet::UF,
            ur: SideCubelet::UR,
            ub: SideCubelet::UB,
            ul: SideCubelet::UL,
            fl: SideCubelet::FL,
            fr: SideCubelet::FR,
            bl: SideCubelet::BL,
            br: SideCubelet::BR,
            df: SideCubelet::DF,
            dr: SideCubelet::DR,
            db: SideCubelet::DB,
            dl: SideCubelet::DL,
        }
    }

    fn from_cube(cube: &Cube) -> Self {
        // sooooo much typing with this one
        // nothing interesting here

        let l = &cube.l.cc;
        let r = &cube.r.cc;
        let u = &cube.u.cc;
        let d = &cube.d.cc;
        let f = &cube.f.cc;
        let b = &cube.b.cc;

        let find_pos = |a_in: &Facelet, b_in: &Facelet| -> SideCubelet {
            let is_match = |exp_a: &Facelet, exp_b: &Facelet| -> bool {
                (a_in == exp_a && b_in == exp_b) || (a_in == exp_b && b_in == exp_a)
            };

            if is_match(u, f) {
                SideCubelet::UF
            } else if is_match(u, r) {
                SideCubelet::UR
            } else if is_match(u, l) {
                SideCubelet::UL
            } else if is_match(u, b) {
                SideCubelet::UB
            } else if is_match(f, l) {
                SideCubelet::FL
            } else if is_match(f, r) {
                SideCubelet::FR
            } else if is_match(b, l) {
                SideCubelet::BL
            } else if is_match(b, r) {
                SideCubelet::BR
            } else if is_match(d, f) {
                SideCubelet::DF
            } else if is_match(d, r) {
                SideCubelet::DR
            } else if is_match(d, l) {
                SideCubelet::DL
            } else if is_match(d, b) {
                SideCubelet::DB
            } else {
                panic!(
                    "idk couldn't find a side pos for colors {:?} / {:?}",
                    a_in, b_in
                );
            }
        };

        Self {
            uf: find_pos(&cube.u.fc, &cube.f.uc),
            ur: find_pos(&cube.u.rc, &cube.r.uc),
            ub: find_pos(&cube.u.bc, &cube.b.uc),
            ul: find_pos(&cube.u.lc, &cube.l.uc),

            fl: find_pos(&cube.f.lc, &cube.l.fc),
            fr: find_pos(&cube.f.rc, &cube.r.fc),
            bl: find_pos(&cube.b.lc, &cube.l.bc),
            br: find_pos(&cube.b.rc, &cube.r.bc),

            df: find_pos(&cube.d.fc, &cube.f.dc),
            dr: find_pos(&cube.d.rc, &cube.r.dc),
            db: find_pos(&cube.d.bc, &cube.b.dc),
            dl: find_pos(&cube.d.lc, &cube.l.dc),
        }
    }
}

impl CanMove for CubeEdgePositions {
    fn r(self) -> Self {
        CubeEdgePositions {
            ur: self.fr,
            fr: self.dr,
            dr: self.br,
            br: self.ur,
            ..self
        }
    }

    fn l(self) -> Self {
        CubeEdgePositions {
            ul: self.bl,
            bl: self.dl,
            dl: self.fl,
            fl: self.ul,
            ..self
        }
    }

    fn u(self) -> Self {
        CubeEdgePositions {
            uf: self.ur,
            ur: self.ub,
            ub: self.ul,
            ul: self.uf,
            ..self
        }
    }

    fn d(self) -> Self {
        CubeEdgePositions {
            df: self.dl,
            dl: self.db,
            db: self.dr,
            dr: self.df,
            ..self
        }
    }

    fn b(self) -> Self {
        CubeEdgePositions {
            ub: self.bl,
            bl: self.db,
            db: self.br,
            br: self.ub,
            ..self
        }
    }

    fn f(self) -> Self {
        CubeEdgePositions {
            uf: self.fl,
            fl: self.df,
            df: self.fr,
            fr: self.uf,
            ..self
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct CubeCornerPositions {
    fur: CornerCubelet,
    ful: CornerCubelet,
    bur: CornerCubelet,
    bul: CornerCubelet,
    fdr: CornerCubelet,
    fdl: CornerCubelet,
    bdr: CornerCubelet,
    bdl: CornerCubelet,
}

impl CubeCornerPositions {
    fn make_solved() -> Self {
        Self {
            fur: CornerCubelet::FUR,
            ful: CornerCubelet::FUL,
            bur: CornerCubelet::BUR,
            bul: CornerCubelet::BUL,
            fdr: CornerCubelet::FDR,
            fdl: CornerCubelet::FDL,
            bdr: CornerCubelet::BDR,
            bdl: CornerCubelet::BDL,
        }
    }

    fn from_cube(cube: &Cube) -> Self {
        // sooooo much typing with this one
        // nothing interesting here

        let l = &cube.l.cc;
        let r = &cube.r.cc;
        let u = &cube.u.cc;
        let d = &cube.d.cc;
        let f = &cube.f.cc;
        let b = &cube.b.cc;

        let find_pos = |a_in: &Facelet, b_in: &Facelet, c_in: &Facelet| -> CornerCubelet {
            let is_match = |exp_a: &Facelet, exp_b: &Facelet, exp_c: &Facelet| -> bool {
                let act = vec![a_in, b_in, c_in];

                for exp in vec![exp_a, exp_b, exp_c].into_iter().permutations(3) {
                    if act == exp {
                        return true;
                    }
                }

                false
            };

            if is_match(f, u, r) {
                CornerCubelet::FUR
            } else if is_match(f, u, l) {
                CornerCubelet::FUL
            } else if is_match(f, d, r) {
                CornerCubelet::FDR
            } else if is_match(f, d, l) {
                CornerCubelet::FDL
            } else if is_match(b, u, r) {
                CornerCubelet::BUR
            } else if is_match(b, u, l) {
                CornerCubelet::BUL
            } else if is_match(b, d, r) {
                CornerCubelet::BDR
            } else if is_match(b, d, l) {
                CornerCubelet::BDL
            } else {
                panic!(
                    "idk couldn't find a corner pos for colors {:?} / {:?} / {:?}",
                    a_in, b_in, c_in
                );
            }
        };

        Self {
            // F corners
            fur: find_pos(&cube.f.ur, &cube.u.fr, &cube.r.uf),
            ful: find_pos(&cube.f.ul, &cube.u.fl, &cube.l.uf),
            fdr: find_pos(&cube.f.dr, &cube.d.fr, &cube.r.df),
            fdl: find_pos(&cube.f.dl, &cube.d.fl, &cube.l.df),

            // B corners
            bur: find_pos(&cube.b.ur, &cube.u.br, &cube.r.ub),
            bul: find_pos(&cube.b.ul, &cube.u.bl, &cube.l.ub),
            bdr: find_pos(&cube.b.dr, &cube.d.br, &cube.r.db),
            bdl: find_pos(&cube.b.dl, &cube.d.bl, &cube.l.db),
        }
    }
}

impl CanMove for CubeCornerPositions {
    fn r(self) -> Self {
        Self {
            fur: self.fdr,
            fdr: self.bdr,
            bdr: self.bur,
            bur: self.fur,
            ..self
        }
    }

    fn l(self) -> Self {
        Self {
            ful: self.bul,
            bul: self.bdl,
            bdl: self.fdl,
            fdl: self.ful,
            ..self
        }
    }

    fn u(self) -> Self {
        Self {
            fur: self.bur,
            bur: self.bul,
            bul: self.ful,
            ful: self.fur,
            ..self
        }
    }

    fn d(self) -> Self {
        Self {
            fdr: self.fdl,
            fdl: self.bdl,
            bdl: self.bdr,
            bdr: self.fdr,
            ..self
        }
    }

    fn b(self) -> Self {
        Self {
            bur: self.bdr,
            bdr: self.bdl,
            bdl: self.bul,
            bul: self.bur,
            ..self
        }
    }

    fn f(self) -> Self {
        Self {
            fur: self.ful,
            ful: self.fdl,
            fdl: self.fdr,
            fdr: self.fur,
            ..self
        }
    }
}

pub struct PosCache(HashSet<CubePositions, RandomState>);

/// Takes about 650ms on my machine with stdlib hashmap
///     Down to 180ms with ahash
pub fn enumerate_g3_pos() -> PosCache {
    let start: CubePositions = CubePositions::make_solved();

    let mut running: HashSet<CubePositions, RandomState> = HashSet::default();
    running.insert(start.clone());

    let mut to_process = VecDeque::new();
    to_process.push_back(start);

    while let Some(next) = to_process.pop_front() {
        for dir in ALL_DIRS {
            let fm = FullMove { dir, amt: Amt::Two };

            let applied = next.clone().apply(fm);
            if running.insert(applied.clone()) {
                to_process.push_back(applied);
            }
        }
    }

    PosCache(running)
}

pub fn solve_to_g4(cube: &Cube) -> Vec<FullMove> {
    // just solve positions using ID-DFS
    let start_state = CubePositions::from_cube(cube);

    // iterative-deepening DFS; returns true if it found a solution, or false if not
    fn ida(cube: &CubePositions, running: &mut Vec<FullMove>, max_depth: usize) -> bool {
        if cube.is_solved() {
            return true;
        } else if running.len() >= max_depth {
            return false;
        }

        for dir in ALL_DIRS {
            if !can_follow(running.last().map(|fm| fm.dir), dir) {
                continue;
            }

            let amt = Amt::Two;

            let fm = FullMove { amt, dir };

            // TODO: make this next bit into a macro so I can reuse it?
            let next = cube.clone().apply(fm);

            // for WC there are so many blanks there is a good chance an individual move
            // will be a no-op, so this cuts runtime by two thirds (!)
            if &next == cube {
                continue;
            }

            running.push(fm);

            let found_solution = ida(&next, running, max_depth);

            if found_solution {
                return true;
            }

            running.pop();
        }

        false
    }

    // Magic math says this is the worst-possible solve length to get to solved
    const MAX_MOVES: usize = 15;

    let start = Instant::now();
    for fuel in 0..=MAX_MOVES {
        if start.elapsed().as_millis() > 3000 {
            println!(
                "   Getting to G4 going slow ... trying with fuel {} i guess (elapsed {:?})",
                fuel,
                start.elapsed()
            );
        }
        let mut running = Vec::new();
        let solved = ida(&start_state, &mut running, fuel);

        if solved {
            return running;
        }
    }

    panic!("Couldn't solve it I guess lol")
}

/// Given a cube in G2, solve to G3
pub fn solve_to_g3(cube: &Cube, cache: &PosCache) -> Vec<FullMove> {
    let start_state = CubePositions::from_cube(cube);

    // iterative-deepening DFS; returns true if it found a solution, or false if not
    fn ida(
        cube: &CubePositions,
        running: &mut Vec<FullMove>,
        max_depth: usize,
        cache: &PosCache,
    ) -> bool {
        if cache.0.contains(cube) {
            return true;
        } else if running.len() >= max_depth {
            return false;
        }

        for dir in G2_DOUBLE_DIRS {
            if !can_follow(running.last().map(|fm| fm.dir), dir) {
                continue;
            }

            let amt = Amt::Two;

            let fm = FullMove { amt, dir };

            // TODO: make this next bit into a macro so I can reuse it?
            let next = cube.clone().apply(fm);

            // for WC there are so many blanks there is a good chance an individual move
            // will be a no-op, so this cuts runtime by two thirds (!)
            if &next == cube {
                continue;
            }

            running.push(fm);

            let found_solution = ida(&next, running, max_depth, cache);

            if found_solution {
                return true;
            }

            running.pop();
        }

        for dir in G2_FREE_DIRS.iter().copied() {
            if running.last().map(|fm| fm.dir) == Some(dir) {
                continue;
            }

            for amt in ALL_AMTS.iter().copied() {
                let fm = FullMove { amt, dir };
                let next = cube.clone().apply(fm);

                running.push(fm);

                let found_solution = ida(&next, running, max_depth, cache);

                if found_solution {
                    return true;
                }

                running.pop();
            }
        }

        false
    }

    // Magic math says this is the worst-possible solve length to get to G3
    const MAX_MOVES: usize = 13;

    let start = Instant::now();
    for fuel in 0..=MAX_MOVES {
        if start.elapsed().as_millis() > 3000 {
            println!(
                "   Getting to G3 going slow ... trying with fuel {} i guess (elapsed {:?})",
                fuel,
                start.elapsed()
            );
        }
        let mut running = Vec::new();
        let solved = ida(&start_state, &mut running, fuel, cache);

        if solved {
            return running;
        }
    }

    panic!("Couldn't solve it I guess lol")
}
