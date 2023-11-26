use crate::cube::Cube;
use crate::moves::CanMove;
use corner_state::CornersState;
use edge_state::EdgesState;

mod corner_state {
    use crate::cube::{Cube, Facelet};
    use crate::moves::CanMove;

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
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

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
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
        pub fn is_solved(&self) -> bool {
            self == &Self {
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
}

mod edge_state {
    use crate::cube::{Cube, Facelet};
    use crate::moves::CanMove;

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub struct EdgesState {
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

    impl EdgesState {
        pub fn is_solved(&self) -> bool {
            self == &Self {
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

        pub fn from_cube(cube: &Cube) -> EdgesState {
            let l_color = cube.l.cc.clone();
            let r_color = cube.r.cc.clone();
            let is_lr_color = |f: &Facelet| f == &l_color || f == &r_color;
            // you belong in the middle if neither facelet is from a side
            let is_mid_slice = |a: &Facelet, b: &Facelet| !is_lr_color(a) && !is_lr_color(b);

            EdgesState {
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
    impl CanMove for EdgesState {
        fn r(self) -> Self {
            EdgesState {
                ur: self.fr,
                fr: self.dr,
                dr: self.br,
                br: self.ur,
                ..self
            }
        }

        fn l(self) -> Self {
            EdgesState {
                ul: self.bl,
                bl: self.dl,
                dl: self.fl,
                fl: self.ul,
                ..self
            }
        }

        fn u(self) -> Self {
            EdgesState {
                uf: self.ur,
                ur: self.ub,
                ub: self.ul,
                ul: self.uf,
                ..self
            }
        }

        fn d(self) -> Self {
            EdgesState {
                df: self.dr,
                dr: self.db,
                db: self.dl,
                dl: self.df,
                ..self
            }
        }

        fn b(self) -> Self {
            EdgesState {
                ub: self.br,
                br: self.db,
                db: self.bl,
                bl: self.ub,
                ..self
            }
        }

        fn f(self) -> Self {
            EdgesState {
                uf: self.fl,
                fl: self.df,
                df: self.fr,
                fr: self.uf,
                ..self
            }
        }
    }
}

/// Invariants from a cube in G0 to describe what's left to get to G2
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct G1State {
    edges: EdgesState,
    corners: CornersState,
}

impl G1State {
    pub fn is_solved(&self) -> bool {
        self.edges.is_solved() && self.corners.is_solved()
    }

    pub fn from_cube(cube: &Cube) -> G1State {
        G1State {
            edges: EdgesState::from_cube(cube),
            corners: CornersState::from_cube(cube),
        }
    }
}

impl CanMove for G1State {
    fn r(self) -> Self {
        Self {
            corners: self.corners.r(),
            edges: self.edges.r(),
        }
    }

    fn l(self) -> Self {
        Self {
            corners: self.corners.l(),
            edges: self.edges.l(),
        }
    }

    fn u(self) -> Self {
        panic!("U not supported")
    }

    fn u_two(self) -> Self {
        Self {
            corners: self.corners.u_two(),
            edges: self.edges.u_two(),
        }
    }

    fn d(self) -> Self {
        panic!("D not supported")
    }

    fn d_two(self) -> Self {
        Self {
            corners: self.corners.d_two(),
            edges: self.edges.d_two(),
        }
    }

    fn b(self) -> Self {
        Self {
            corners: self.corners.b(),
            edges: self.edges.b(),
        }
    }

    fn f(self) -> Self {
        Self {
            corners: self.corners.f(),
            edges: self.edges.f(),
        }
    }
}
