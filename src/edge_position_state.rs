use ahash::HashSet;

use crate::cube::{Cube, Facelet};
use crate::moves::CanMove;

/// This indicates what the cubelet "is;" or equivalently, where it belongs.
/// 12 possible values, fits in a byte.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum SideCubelet {
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

impl SideCubelet {
    pub fn to_index(&self) -> u8 {
        // do not change these indices, they are in sync with EDGES array
        match self {
            // left side first
            SideCubelet::UL => 0,
            SideCubelet::FL => 1,
            SideCubelet::DL => 2,
            SideCubelet::BL => 3,
            // then mid
            SideCubelet::UF => 4,
            SideCubelet::DF => 5,
            SideCubelet::DB => 6,
            SideCubelet::UB => 7,
            // then right
            SideCubelet::UR => 8,
            SideCubelet::BR => 9,
            SideCubelet::DR => 10,
            SideCubelet::FR => 11,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct CubeEdgePositions {
    pub uf: SideCubelet,
    pub ur: SideCubelet,
    pub ub: SideCubelet,
    pub ul: SideCubelet,
    pub fl: SideCubelet,
    pub fr: SideCubelet,
    pub bl: SideCubelet,
    pub br: SideCubelet,
    pub df: SideCubelet,
    pub dr: SideCubelet,
    pub db: SideCubelet,
    pub dl: SideCubelet,
}

impl CubeEdgePositions {
    pub fn make_solved() -> Self {
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

    fn ind(&self, index: u8) -> SideCubelet {
        match index {
            0 => self.uf.clone(),
            1 => self.ur.clone(),
            2 => self.ub.clone(),
            3 => self.ul.clone(),
            4 => self.fl.clone(),
            5 => self.fr.clone(),
            6 => self.bl.clone(),
            7 => self.br.clone(),
            8 => self.df.clone(),
            9 => self.dr.clone(),
            10 => self.db.clone(),
            11 => self.dl.clone(),
            _ => panic!("Out of range index {index}"),
        }
    }

    /// Indicates if the edge position state is directly solvable. Essentially this has
    /// even parity (yes) or odd parity (no).
    ///
    /// Note: the behavior of this function is unspecified if the position state is not a
    /// permutation (i.e. it has repeats). It may panic or give any answer, and is subject
    /// to change.
    pub fn directly_solvable(&self) -> bool {
        let mut seen: HashSet<u8> = HashSet::default();

        let mut total_is_even = true;

        for i in 0..12 {
            if seen.contains(&i) {
                continue;
            }

            let mut cycle_length = 0;
            let mut next = i;

            while !seen.contains(&next) {
                seen.insert(next);
                next = self.ind(next).to_index();
                cycle_length += 1;
            }

            if cycle_length % 2 != 1 {
                total_is_even = !total_is_even;
            }
        }

        total_is_even
    }

    pub fn from_cube(cube: &Cube) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved_test() {
        let input = CubeEdgePositions::make_solved();

        assert!(input.directly_solvable());
    }

    #[test]
    fn one_swap_test() {
        let mut input = CubeEdgePositions::make_solved();
        input.uf = SideCubelet::BR;
        input.br = SideCubelet::UF;

        assert!(!input.directly_solvable());
    }

    #[test]
    fn one_triple_test() {
        let mut input = CubeEdgePositions::make_solved();
        input.uf = SideCubelet::BR;
        input.br = SideCubelet::DL;
        input.dl = SideCubelet::UF;

        assert!(input.directly_solvable());
    }

    #[test]
    fn two_swap_test() {
        let mut input = CubeEdgePositions::make_solved();

        input.uf = SideCubelet::BR;
        input.br = SideCubelet::UF;

        input.ul = SideCubelet::FL;
        input.fl = SideCubelet::UL;

        assert!(input.directly_solvable());
    }

    #[test]
    fn four_cycle_test() {
        let mut input = CubeEdgePositions::make_solved();
        input.fr = SideCubelet::DR;
        input.dr = SideCubelet::DL;
        input.dl = SideCubelet::BL;
        input.bl = SideCubelet::FR;

        assert!(!input.directly_solvable());
    }
}
