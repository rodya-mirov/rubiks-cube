//! Module for the "corner orientation state" -- ensuring the corners are correctly oriented
//! (that is, that the corners' facelets on the L/R faces actually match the L/R colors).

use crate::cube::{Cube, Facelet};
use crate::moves::CanMove;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum CornerOrientation {
    Good,
    // means it has been rotated once CW
    CW,
    // means it has been rotated once CCW
    CCW,
}

impl CornerOrientation {
    fn cw(self) -> Self {
        match self {
            CornerOrientation::Good => CornerOrientation::CW,
            CornerOrientation::CW => CornerOrientation::CCW,
            CornerOrientation::CCW => CornerOrientation::Good,
        }
    }

    fn ccw(self) -> Self {
        match self {
            CornerOrientation::Good => CornerOrientation::CCW,
            CornerOrientation::CW => CornerOrientation::Good,
            CornerOrientation::CCW => CornerOrientation::CW,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct CornersState {
    // each field is "this corner is in this orientation"
    // F corners
    ful: CornerOrientation,
    fur: CornerOrientation,
    fdl: CornerOrientation,
    fdr: CornerOrientation,
    // B corners
    bul: CornerOrientation,
    bur: CornerOrientation,
    bdl: CornerOrientation,
    bdr: CornerOrientation,
}

impl CornersState {
    #[inline(always)]
    pub fn solved() -> Self {
        Self {
            ful: CornerOrientation::Good,
            fur: CornerOrientation::Good,
            fdl: CornerOrientation::Good,
            fdr: CornerOrientation::Good,
            bul: CornerOrientation::Good,
            bur: CornerOrientation::Good,
            bdl: CornerOrientation::Good,
            bdr: CornerOrientation::Good,
        }
    }

    pub fn is_solved(&self) -> bool {
        self == &Self::solved()
    }

    pub fn from_cube(cube: &Cube) -> CornersState {
        let l_color = cube.l.cc.clone();
        let r_color = cube.r.cc.clone();
        let is_lr_color = |f: &Facelet| f == &l_color || f == &r_color;

        // feed it the side facelet, then the next facelet going CW from there, if you were
        // looking at the cubelet straight on
        let orientation = |side: &Facelet, next: &Facelet| {
            if is_lr_color(side) {
                CornerOrientation::Good
            } else if is_lr_color(next) {
                CornerOrientation::CW
            } else {
                CornerOrientation::CCW
            }
        };

        CornersState {
            ful: orientation(&cube.l.uf, &cube.u.fl),
            fur: orientation(&cube.r.uf, &cube.f.ur),
            fdl: orientation(&cube.l.df, &cube.f.dl),
            fdr: orientation(&cube.r.df, &cube.d.fr),
            bul: orientation(&cube.l.ub, &cube.b.ul),
            bur: orientation(&cube.r.ub, &cube.u.br),
            bdl: orientation(&cube.l.db, &cube.d.bl),
            bdr: orientation(&cube.r.db, &cube.b.dr),
        }
    }
}

impl CanMove for CornersState {
    fn r(self) -> Self {
        CornersState {
            // simple rotation of the r-corners; no orientation change
            fur: self.fdr,
            fdr: self.bdr,
            bdr: self.bur,
            bur: self.fur,
            ..self
        }
    }

    fn l(self) -> Self {
        CornersState {
            // simple rotation of the l-corners; no orientation change
            ful: self.bul,
            bul: self.bdl,
            bdl: self.fdl,
            fdl: self.ful,
            ..self
        }
    }

    fn u(self) -> Self {
        panic!("U is not allowed");
    }

    fn u_two(self) -> Self {
        CornersState {
            // U alters the orientation of the affected cubelets in a way that is sort of weird
            // but we're not allowed to use it anymore; U2 is a simple swap
            ful: self.bur,
            bur: self.ful,
            fur: self.bul,
            bul: self.fur,
            ..self
        }
    }

    fn d(self) -> Self {
        CornersState {
            // D alters the orientation of the affected cubelets in a way that is sort of weird
            fdl: self.bdl.cw(),
            bdl: self.bdr.ccw(),
            bdr: self.fdr.cw(),
            fdr: self.fdl.ccw(),
            ..self
        }
    }

    fn b(self) -> Self {
        CornersState {
            // B alters the orientation of the affected cubelets in a way that is sort of weird
            bul: self.bur.ccw(),
            bur: self.bdr.cw(),
            bdr: self.bdl.ccw(),
            bdl: self.bul.cw(),
            ..self
        }
    }

    fn f(self) -> Self {
        CornersState {
            // F alters the orientation of the affected cubelets in a way that is sort of weird
            ful: self.fdl.cw(),
            fdl: self.fdr.ccw(),
            fdr: self.fur.cw(),
            fur: self.ful.ccw(),
            ..self
        }
    }
}
