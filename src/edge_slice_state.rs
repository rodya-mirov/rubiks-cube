//! Module for the "edge slice state" -- ensuring the edges which belong in the middle slice
//! are actually _in_ the middle slice.

use crate::cube::{Cube, Facelet};
use crate::moves::CanMove;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct EdgeMidSliceState {
    // each field is "this edge belongs in the middle slice (not on L or R)"
    // top layer
    uf: bool,
    ub: bool,
    ul: bool,
    ur: bool,
    // mid layer
    fl: bool,
    fr: bool,
    bl: bool,
    br: bool,
    // bot layer
    df: bool,
    db: bool,
    dl: bool,
    dr: bool,
}

impl EdgeMidSliceState {
    #[inline(always)]
    pub fn solved() -> Self {
        Self {
            uf: true,
            ub: true,
            ul: false,
            ur: false,
            fl: false,
            fr: false,
            bl: false,
            br: false,
            df: true,
            db: true,
            dl: false,
            dr: false,
        }
    }

    pub fn is_solved(&self) -> bool {
        self == &Self::solved()
    }

    pub fn from_cube(cube: &Cube) -> EdgeMidSliceState {
        let l_color = cube.l.cc.clone();
        let r_color = cube.r.cc.clone();
        let is_lr_color = |f: &Facelet| f == &l_color || f == &r_color;
        // you belong in the middle if neither facelet is from a side
        let is_mid_slice = |a: &Facelet, b: &Facelet| !is_lr_color(a) && !is_lr_color(b);

        EdgeMidSliceState {
            uf: is_mid_slice(&cube.u.fc, &cube.f.uc),
            ub: is_mid_slice(&cube.u.bc, &cube.b.uc),
            ul: is_mid_slice(&cube.u.lc, &cube.l.uc),
            ur: is_mid_slice(&cube.u.rc, &cube.r.uc),
            fl: is_mid_slice(&cube.f.lc, &cube.l.fc),
            fr: is_mid_slice(&cube.f.rc, &cube.r.fc),
            bl: is_mid_slice(&cube.b.lc, &cube.l.bc),
            br: is_mid_slice(&cube.b.rc, &cube.r.bc),
            df: is_mid_slice(&cube.d.fc, &cube.f.dc),
            db: is_mid_slice(&cube.d.bc, &cube.b.dc),
            dl: is_mid_slice(&cube.d.lc, &cube.l.dc),
            dr: is_mid_slice(&cube.d.rc, &cube.r.dc),
        }
    }
}

// all simple permutations, no tricks with this
impl CanMove for EdgeMidSliceState {
    fn r(self) -> Self {
        EdgeMidSliceState {
            ur: self.fr,
            fr: self.dr,
            dr: self.br,
            br: self.ur,
            ..self
        }
    }

    fn l(self) -> Self {
        EdgeMidSliceState {
            ul: self.bl,
            bl: self.dl,
            dl: self.fl,
            fl: self.ul,
            ..self
        }
    }

    fn u(self) -> Self {
        EdgeMidSliceState {
            uf: self.ur,
            ur: self.ub,
            ub: self.ul,
            ul: self.uf,
            ..self
        }
    }

    fn d(self) -> Self {
        EdgeMidSliceState {
            df: self.dl,
            dl: self.db,
            db: self.dr,
            dr: self.df,
            ..self
        }
    }

    fn b(self) -> Self {
        EdgeMidSliceState {
            ub: self.br,
            br: self.db,
            db: self.bl,
            bl: self.ub,
            ..self
        }
    }

    fn f(self) -> Self {
        EdgeMidSliceState {
            uf: self.fl,
            fl: self.df,
            df: self.fr,
            fr: self.uf,
            ..self
        }
    }
}
