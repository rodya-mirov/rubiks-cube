use ahash::HashSet;
use itertools::Itertools;

use crate::cube::{Cube, Facelet};
use crate::moves::CanMove;

/// This indicates what the cubelet "is;" or equivalently, where it belongs.
/// 8 possible values, fits in a byte.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum CornerCubelet {
    FUL,
    FUR,
    BUL,
    BUR,
    FDL,
    FDR,
    BDL,
    BDR,
}

impl CornerCubelet {
    fn to_index(&self) -> u8 {
        match self {
            CornerCubelet::FUL => 0,
            CornerCubelet::FUR => 1,
            CornerCubelet::BUL => 2,
            CornerCubelet::BUR => 3,
            CornerCubelet::FDL => 4,
            CornerCubelet::FDR => 5,
            CornerCubelet::BDL => 6,
            CornerCubelet::BDR => 7,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct CubeCornerPositions {
    pub ful: CornerCubelet,
    pub fur: CornerCubelet,
    pub bul: CornerCubelet,
    pub bur: CornerCubelet,
    pub fdl: CornerCubelet,
    pub fdr: CornerCubelet,
    pub bdl: CornerCubelet,
    pub bdr: CornerCubelet,
}

impl CubeCornerPositions {
    pub fn make_solved() -> Self {
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

    fn ind(&self, index: u8) -> CornerCubelet {
        match index {
            0 => self.ful.clone(),
            1 => self.fur.clone(),
            2 => self.bul.clone(),
            3 => self.bur.clone(),
            4 => self.fdl.clone(),
            5 => self.fdr.clone(),
            6 => self.bdl.clone(),
            7 => self.bdr.clone(),
            _ => panic!("Out of range index {index}"),
        }
    }

    /// Indicates if the corner position state is directly solvable. Essentially this has
    /// even parity (yes) or odd parity (no).
    ///
    /// Note: the behavior of this function is unspecified if the position state is not a
    /// permutation (i.e. it has repeats). It may panic or give any answer, and is subject
    /// to change.
    pub fn directly_solvable(&self) -> bool {
        let mut seen: HashSet<u8> = HashSet::default();

        let mut total_is_even = true;

        for i in 0..8 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved_test() {
        let input = CubeCornerPositions::make_solved();

        assert!(input.directly_solvable());
    }

    #[test]
    fn one_swap_test() {
        let mut input = CubeCornerPositions::make_solved();
        input.fdr = CornerCubelet::BDL;
        input.bdl = CornerCubelet::FDR;

        assert!(!input.directly_solvable());
    }

    #[test]
    fn one_triple_test() {
        let mut input = CubeCornerPositions::make_solved();
        input.fdr = CornerCubelet::BDR;
        input.bdr = CornerCubelet::BDL;
        input.bdl = CornerCubelet::FDR;

        assert!(input.directly_solvable());
    }

    #[test]
    fn two_swap_test() {
        let mut input = CubeCornerPositions::make_solved();

        input.fdr = CornerCubelet::BDR;
        input.bdr = CornerCubelet::FDR;

        input.bdl = CornerCubelet::BUR;
        input.bur = CornerCubelet::BDL;

        assert!(input.directly_solvable());
    }

    #[test]
    fn four_cycle_test() {
        let mut input = CubeCornerPositions::make_solved();
        input.fdr = CornerCubelet::BDR;
        input.bdr = CornerCubelet::BDL;
        input.bdl = CornerCubelet::BUR;
        input.bur = CornerCubelet::FDR;

        assert!(!input.directly_solvable());
    }
}
