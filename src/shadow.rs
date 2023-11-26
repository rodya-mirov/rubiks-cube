//! Module for "shadowing" a cube. That is, blocking out "irrelevant" pieces for the purpose
//! of solving to a particular point, such as white cross, F2L, OLL, and so on.

use crate::cube::{AmbigFacelet, Cube, FBFace, Facelet, LRFace, UDFace};

impl Into<AmbigFacelet> for Facelet {
    fn into(self) -> AmbigFacelet {
        match self {
            Facelet::Yellow => AmbigFacelet::Yellow,
            Facelet::White => AmbigFacelet::White,
            Facelet::Green => AmbigFacelet::Green,
            Facelet::Blue => AmbigFacelet::Blue,
            Facelet::Red => AmbigFacelet::Red,
            Facelet::Orange => AmbigFacelet::Orange,
        }
    }
}

fn apply_mask(f: Facelet, mask: bool) -> AmbigFacelet {
    if mask {
        f.into()
    } else {
        AmbigFacelet::Unknown
    }
}

impl LRFace<Facelet> {
    fn apply_mask(self, mask: LRFace<bool>) -> LRFace<AmbigFacelet> {
        LRFace {
            ub: apply_mask(self.ub, mask.ub),
            uc: apply_mask(self.uc, mask.uc),
            uf: apply_mask(self.uf, mask.uf),
            bc: apply_mask(self.bc, mask.bc),
            cc: apply_mask(self.cc, mask.cc),
            fc: apply_mask(self.fc, mask.fc),
            db: apply_mask(self.db, mask.db),
            dc: apply_mask(self.dc, mask.dc),
            df: apply_mask(self.df, mask.df),
        }
    }
}

impl UDFace<Facelet> {
    fn apply_mask(self, mask: UDFace<bool>) -> UDFace<AmbigFacelet> {
        UDFace {
            bl: apply_mask(self.bl, mask.bl),
            bc: apply_mask(self.bc, mask.bc),
            br: apply_mask(self.br, mask.br),
            lc: apply_mask(self.lc, mask.lc),
            cc: apply_mask(self.cc, mask.cc),
            rc: apply_mask(self.rc, mask.rc),
            fl: apply_mask(self.fl, mask.fl),
            fc: apply_mask(self.fc, mask.fc),
            fr: apply_mask(self.fr, mask.fr),
        }
    }
}

impl FBFace<Facelet> {
    fn apply_mask(self, mask: FBFace<bool>) -> FBFace<AmbigFacelet> {
        FBFace {
            ul: apply_mask(self.ul, mask.ul),
            uc: apply_mask(self.uc, mask.uc),
            ur: apply_mask(self.ur, mask.ur),
            lc: apply_mask(self.lc, mask.lc),
            cc: apply_mask(self.cc, mask.cc),
            rc: apply_mask(self.rc, mask.rc),
            dl: apply_mask(self.dl, mask.dl),
            dc: apply_mask(self.dc, mask.dc),
            dr: apply_mask(self.dr, mask.dr),
        }
    }
}

impl Cube {
    fn apply_mask(self, mask: CubeMask) -> Cube<AmbigFacelet> {
        let Self { u, d, f, b, l, r } = self;

        Cube {
            u: u.apply_mask(mask.u),
            d: d.apply_mask(mask.d),
            b: b.apply_mask(mask.b),
            f: f.apply_mask(mask.f),
            l: l.apply_mask(mask.l),
            r: r.apply_mask(mask.r),
        }
    }
}

type UDMask = UDFace<bool>;

impl UDMask {
    fn from_val(val: bool) -> Self {
        Self {
            bl: val,
            bc: val,
            br: val,
            lc: val,
            cc: val,
            rc: val,
            fl: val,
            fc: val,
            fr: val,
        }
    }
}

type FBMask = FBFace<bool>;

impl FBMask {
    fn from_val(val: bool) -> Self {
        Self {
            ul: val,
            uc: val,
            ur: val,
            lc: val,
            cc: val,
            rc: val,
            dl: val,
            dc: val,
            dr: val,
        }
    }
}

type LRMask = LRFace<bool>;

impl LRMask {
    fn from_val(val: bool) -> Self {
        Self {
            ub: val,
            uc: val,
            uf: val,
            bc: val,
            cc: val,
            fc: val,
            db: val,
            dc: val,
            df: val,
        }
    }
}

type CubeMask = Cube<bool>;

impl CubeMask {
    fn from_val(val: bool) -> Self {
        Cube {
            u: UDMask::from_val(val),
            d: UDMask::from_val(val),
            b: FBMask::from_val(val),
            f: FBMask::from_val(val),
            l: LRMask::from_val(val),
            r: LRMask::from_val(val),
        }
    }

    fn keep_centers(&mut self) {
        self.u.cc = true;
        self.l.cc = true;
        self.r.cc = true;
        self.d.cc = true;
        self.f.cc = true;
        self.b.cc = true;
    }

    fn keep_white_edges(&mut self, cube: &Cube) {
        for (edge_a, edge_b, mask_a, mask_b) in edges_mut(cube, self) {
            if edge_a == &Facelet::White || edge_b == &Facelet::White {
                *mask_a = true;
                *mask_b = true;
            }
        }
    }
}

fn edges_mut<'a>(
    cube: &'a Cube,
    mask: &'a mut CubeMask,
) -> [(&'a Facelet, &'a Facelet, &'a mut bool, &'a mut bool); 12] {
    [
        // top four edges ...
        (&cube.u.fc, &cube.f.uc, &mut mask.u.fc, &mut mask.f.uc),
        (&cube.u.rc, &cube.r.uc, &mut mask.u.rc, &mut mask.r.uc),
        (&cube.u.lc, &cube.l.uc, &mut mask.u.lc, &mut mask.l.uc),
        (&cube.u.bc, &cube.b.uc, &mut mask.u.bc, &mut mask.b.uc),
        // ... middle four edges ...
        (&cube.f.lc, &cube.l.fc, &mut mask.f.lc, &mut mask.l.fc),
        (&cube.f.rc, &cube.r.fc, &mut mask.f.rc, &mut mask.r.fc),
        (&cube.b.lc, &cube.l.bc, &mut mask.b.lc, &mut mask.l.bc),
        (&cube.b.rc, &cube.r.bc, &mut mask.b.rc, &mut mask.r.bc),
        // ... bottom four edges
        (&cube.d.fc, &cube.f.dc, &mut mask.d.fc, &mut mask.f.dc),
        (&cube.d.rc, &cube.r.dc, &mut mask.d.rc, &mut mask.r.dc),
        (&cube.d.lc, &cube.l.dc, &mut mask.d.lc, &mut mask.l.dc),
        (&cube.d.bc, &cube.b.dc, &mut mask.d.bc, &mut mask.b.dc),
    ]
}

pub fn to_white_cross(cube: Cube) -> Cube<AmbigFacelet> {
    let mut mask = CubeMask::from_val(false);

    mask.keep_centers();
    mask.keep_white_edges(&cube);

    cube.apply_mask(mask)
}

#[cfg(test)]
mod wc_tests {
    // messes up top layer; leaves first two layers alone
    const OLL_SCRAMBLE: &'static str = "R U2 R' U' R U' R' U'";

    // messes up bottom layer, leaves white cross alone
    const FL_SCRAMBLE: &'static str = "U F' U' F";

    // messes up second layer, leaves bottom layer alone
    const F2L_SCRAMBLE: &'static str = "U F' U' F U R U' R'";

    // messes up bottom layer, leaves white cross alone (OLL move that i flipped to the bottom)
    const OBL_SCRAMBLE: &'static str = "R' D2 R D R' D R D";

    // messes up yellow cross, leaves F2L alone
    const YELLOW_CROSS_SCRAMBLE: &'static str = "F R U R' U' F'";

    // messes up white cross, leaves other two layers alone
    const WHITE_CROSS_SCRAMBLE: &'static str = "F L D L' D' F'";

    use super::*;
    use crate::moves::{parse_many, ApplyMove};

    #[test]
    fn from_start_test() {
        use AmbigFacelet::*;

        let start = Cube::make_solved(Facelet::Orange, Facelet::White);

        let exp: Cube<AmbigFacelet> = Cube {
            // top : white
            u: UDFace {
                cc: White,
                // edges all kept (white) and center
                bc: White,
                lc: White,
                rc: White,
                fc: White,
                // corners blanked out
                bl: Unknown,
                br: Unknown,
                fl: Unknown,
                fr: Unknown,
            },
            // down: yellow
            d: UDFace {
                // center always kept (why not); others blanked out
                cc: Yellow,
                // everything else blanked out
                bl: Unknown,
                bc: Unknown,
                br: Unknown,
                lc: Unknown,
                rc: Unknown,
                fl: Unknown,
                fc: Unknown,
                fr: Unknown,
            },
            // left: blue face
            l: LRFace {
                // center kept
                cc: Blue,
                // top edge kept (white adjacent)
                uc: Blue,
                // everything else blank
                ub: Unknown,
                uf: Unknown,
                bc: Unknown,
                fc: Unknown,
                db: Unknown,
                dc: Unknown,
                df: Unknown,
            },
            // right: green
            r: LRFace {
                // keep center
                cc: Green,
                // keep top edge (white adjacent)
                uc: Green,
                // rest blank
                ub: Unknown,
                fc: Unknown,
                db: Unknown,
                dc: Unknown,
                uf: Unknown,
                bc: Unknown,
                df: Unknown,
            },
            // front: orange
            f: FBFace {
                // keep center
                cc: Orange,
                // keep top edge (white adjacent)
                uc: Orange,
                // rest blank
                ul: Unknown,
                ur: Unknown,
                lc: Unknown,
                rc: Unknown,
                dl: Unknown,
                dc: Unknown,
                dr: Unknown,
            },
            // back: red
            b: FBFace {
                // keep center
                cc: Red,
                // keep top edge (white adjacent)
                uc: Red,
                // rest blank
                ul: Unknown,
                ur: Unknown,
                lc: Unknown,
                rc: Unknown,
                dl: Unknown,
                dc: Unknown,
                dr: Unknown,
            },
        };

        let actual = to_white_cross(start.clone());

        assert_eq!(actual, exp);

        assert!(start.is_solved());
        assert!(actual.is_solved());
    }

    fn test_wc_state(start_state: Cube, moves: &str, should_be_solved: bool) {
        let parsed = parse_many(moves);
        let end_state = start_state.clone().apply_many(&parsed);
        let wc_masked = to_white_cross(end_state);

        if should_be_solved {
            assert!(wc_masked.is_solved());
        } else {
            assert!(!wc_masked.is_solved());
        }
    }

    #[test]
    fn oll_scramble_wc_solved() {
        // note white is on the bottom
        let start = Cube::make_solved(Facelet::Red, Facelet::Yellow);

        test_wc_state(start, OLL_SCRAMBLE, true);
    }

    #[test]
    fn f2l_scramble_wc_solved() {
        // note white is on the bottom
        let start = Cube::make_solved(Facelet::Green, Facelet::Yellow);

        test_wc_state(start, F2L_SCRAMBLE, true);
    }

    #[test]
    fn first_layer_scramble_wc_solved() {
        // note white is on the bottom
        let start = Cube::make_solved(Facelet::Green, Facelet::Yellow);

        test_wc_state(start, FL_SCRAMBLE, true);
    }

    #[test]
    fn first_layer_scrambled() {
        // note white is on the bottom
        let start = Cube::make_solved(Facelet::Green, Facelet::Yellow);

        // there will still be a "white cross" on the bottom, but two center bits are in the
        // wrong place, so this actually does mess up the wc
        test_wc_state(start, OBL_SCRAMBLE, false);
    }

    #[test]
    fn yc_scrambled() {
        // note white is on the bottom
        let start = Cube::make_solved(Facelet::Green, Facelet::Yellow);

        test_wc_state(start, YELLOW_CROSS_SCRAMBLE, true);
    }

    #[test]
    fn first_layer_wc_scrambled() {
        // note white is on the bottom
        let start = Cube::make_solved(Facelet::Green, Facelet::Yellow);

        test_wc_state(start, WHITE_CROSS_SCRAMBLE, false);
    }

    #[test]
    fn first_layer_wc_scrambled_flipped_cube() {
        // note white is on the TOP
        let start = Cube::make_solved(Facelet::Green, Facelet::White);

        // we scramble the bottom, so the white cross (which is on top) is unaffected
        test_wc_state(start, WHITE_CROSS_SCRAMBLE, true);
    }
}
