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
