use crate::cube::{Cube, Facelet};
use crate::moves::CanMove;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct EdgeOrientationState {
    // each field is "this edge is good"
    // top layer
    pub uf: bool,
    pub ub: bool,
    pub ul: bool,
    pub ur: bool,
    // mid layer
    pub fl: bool,
    pub fr: bool,
    pub bl: bool,
    pub br: bool,
    // bot layer
    pub df: bool,
    pub db: bool,
    pub dl: bool,
    pub dr: bool,
}

impl EdgeOrientationState {
    pub fn from_cube(cube: &Cube) -> Self {
        // The suggested algorithm in Thistlethwaite is annoying to deal with in person and in code
        // A simpler way to check orientation is presented in: http://cube.rider.biz/zz.php?p=eoline
        // One quirk is we're trying to be faithful to Thistlethwaite's original group notation,
        // which defines orientation in terms of avoiding U and D, while in many more modern sources
        // (such as the above) define orientation in terms of avoiding F and B. So the spirit still
        // works but all of the details need to be changed.

        let l_color = cube.l.cc.clone();
        let r_color = cube.r.cc.clone();
        let u_color = cube.u.cc.clone();
        let d_color = cube.d.cc.clone();
        let f_color = cube.f.cc.clone();
        let b_color = cube.b.cc.clone();

        let is_lr_color = |f: &Facelet| f == &l_color || f == &r_color;
        let is_ud_color = |f: &Facelet| f == &u_color || f == &d_color;
        let is_fb_color = |f: &Facelet| f == &f_color || f == &b_color;

        // Look at the edges on the L/R faces. If you see:
        //      F/B color it's bad
        //      U/D color you need to look at the side of the edge. If the side is L/R it's bad.
        let lr_good = |lr: &Facelet, other: &Facelet| {
            (!is_fb_color(lr)) && !(is_ud_color(lr) && is_lr_color(other))
        };

        // Then look at the U/D edges on the mid slice. If you see:
        //      F/B color it's bad
        //      U/D color you need to look at the side of the edge. If the side is L/R it's bad.
        let ud_mid_good = |ud: &Facelet, fb: &Facelet| {
            (!is_fb_color(ud)) && !(is_ud_color(ud) && is_lr_color(fb))
        };

        Self {
            // ud mid edges ...
            uf: ud_mid_good(&cube.u.fc, &cube.f.uc),
            ub: ud_mid_good(&cube.u.bc, &cube.b.uc),
            df: ud_mid_good(&cube.d.fc, &cube.f.dc),
            db: ud_mid_good(&cube.d.bc, &cube.b.dc),
            // l edges
            ul: lr_good(&cube.l.uc, &cube.u.lc),
            fl: lr_good(&cube.l.fc, &cube.f.lc),
            bl: lr_good(&cube.l.bc, &cube.b.lc),
            dl: lr_good(&cube.l.dc, &cube.d.lc),
            // r edges
            ur: lr_good(&cube.r.uc, &cube.u.rc),
            fr: lr_good(&cube.r.fc, &cube.f.rc),
            br: lr_good(&cube.r.bc, &cube.b.rc),
            dr: lr_good(&cube.r.dc, &cube.d.rc),
        }
    }

    #[inline(always)]
    pub fn make_solved() -> Self {
        Self {
            uf: true,
            ub: true,
            ul: true,
            ur: true,
            fl: true,
            fr: true,
            bl: true,
            br: true,
            df: true,
            db: true,
            dl: true,
            dr: true,
        }
    }

    pub fn is_solvable(&self) -> bool {
        // solvable if an even number of pieces are out of orientation
        let is_flipped = self.uf
            ^ self.ub
            ^ self.ul
            ^ self.ur
            ^ self.fl
            ^ self.fr
            ^ self.bl
            ^ self.br
            ^ self.df
            ^ self.db
            ^ self.dl
            ^ self.dr;

        !is_flipped
    }

    pub fn is_solved(&self) -> bool {
        self.uf
            && self.ub
            && self.ul
            && self.ur
            && self.fl
            && self.fr
            && self.bl
            && self.br
            && self.df
            && self.db
            && self.dl
            && self.dr
    }
}

impl CanMove for EdgeOrientationState {
    fn r(self) -> Self {
        Self {
            ur: self.fr,
            fr: self.dr,
            dr: self.br,
            br: self.ur,
            ..self
        }
    }

    fn l(self) -> Self {
        Self {
            ul: self.bl,
            bl: self.dl,
            dl: self.fl,
            fl: self.ul,
            ..self
        }
    }

    fn u(self) -> Self {
        // negates the top layer as it rotates them; otherwise still
        Self {
            uf: !self.ur,
            ur: !self.ub,
            ub: !self.ul,
            ul: !self.uf,
            ..self
        }
    }

    fn d(self) -> Self {
        // negates the bottom layer as it rotates them; otherwise still
        Self {
            df: !self.dl,
            dl: !self.db,
            db: !self.dr,
            dr: !self.df,
            ..self
        }
    }

    fn b(self) -> Self {
        Self {
            ub: self.br,
            br: self.db,
            db: self.bl,
            bl: self.ub,
            ..self
        }
    }

    fn f(self) -> Self {
        Self {
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
        let input = EdgeOrientationState::make_solved();

        assert!(input.is_solvable());
    }

    #[test]
    fn one_flip_test() {
        let mut input = EdgeOrientationState::make_solved();

        input.uf = false;

        assert!(!input.is_solvable());
    }

    #[test]
    fn two_flip_test() {
        let mut input = EdgeOrientationState::make_solved();

        input.uf = false;
        input.db = false;

        assert!(input.is_solvable());
    }

    #[test]
    fn three_flip_test() {
        let mut input = EdgeOrientationState::make_solved();

        input.uf = false;
        input.ul = false;
        input.fl = false;

        assert!(!input.is_solvable());
    }
}
