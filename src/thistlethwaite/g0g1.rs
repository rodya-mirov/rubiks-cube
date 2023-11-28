use crate::cube::{Cube, Facelet};
use crate::moves::{CanMove, Dir, FullMove};
use crate::thistlethwaite::dfs_util;

/// Invariants from a cube in G0 to describe what's left to get to G1
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct G0State {
    // each field is "this edge is good"
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

impl G0State {
    fn is_solved(&self) -> bool {
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

impl CanMove for G0State {
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

fn to_g1_invariant(cube: &Cube) -> G0State {
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
    let ud_mid_good =
        |ud: &Facelet, fb: &Facelet| (!is_fb_color(ud)) && !(is_ud_color(ud) && is_lr_color(fb));

    G0State {
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

const ALL_DIRS: [Dir; 6] = [Dir::U, Dir::D, Dir::B, Dir::F, Dir::L, Dir::R];

/// Solves a given cube to G1. Assumes the input is in G0 (that is, solvable).
pub fn solve_to_g1(cube: &Cube) -> Vec<FullMove> {
    // Apparently you can solve G0 -> G1 in 7 moves, idk
    const MAX_MOVES: usize = 7;

    dfs_util::solve(
        cube,
        &ALL_DIRS,
        &[],
        to_g1_invariant,
        |s| s.is_solved(),
        |_| 0,
        MAX_MOVES,
    )
}
