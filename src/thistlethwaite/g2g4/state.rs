use itertools::Itertools;

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
