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

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct CubeCornerPositions {
    pub fur: CornerCubelet,
    pub ful: CornerCubelet,
    pub bur: CornerCubelet,
    pub bul: CornerCubelet,
    pub fdr: CornerCubelet,
    pub fdl: CornerCubelet,
    pub bdr: CornerCubelet,
    pub bdl: CornerCubelet,
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

    pub fn from_cube(cube: &Cube) -> Self {
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
