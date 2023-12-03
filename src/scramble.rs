use rand::prelude::*;

use crate::corner_orientation_state::{CornerOrientation, CornerOrientationState};
use crate::corner_position_state::{CornerCubelet, CubeCornerPositions};
use crate::cube::{Cube, Facelet, CORNERS, EDGES};
use crate::edge_orientation_state::EdgeOrientationState;
use crate::edge_position_state::{CubeEdgePositions, SideCubelet};
use crate::total_position_state::CubePositions;

/// Gives a random solvable scramble of a cube with no restrictions, except the rotation is fixed
/// (green from and yellow top).
pub fn scramble_any() -> Cube {
    use CornerCubelet::*;
    use SideCubelet::*;

    let mut rng = thread_rng();

    // simply; put each edge and each corner in randomly
    // if the position is unsolvable, swap two edges
    // if the edge orientation is unsolvable, flip the last edge placed
    // if the corner orientation is unsolvable

    // starting with position ...
    let mut corners: Vec<CornerCubelet> = vec![FUL, FUR, BUL, BUR, FDL, FDR, BDL, BDR];
    corners.shuffle(&mut rng);
    let corner_pos = make_corner_pos(&corners);

    let mut edges: Vec<SideCubelet> = vec![UF, UR, UB, UL, FL, FR, BL, BR, DF, DR, DB, DL];
    edges.shuffle(&mut rng);
    let edge_pos = make_edge_pos(&edges);

    let mut total_pos = CubePositions {
        edges: edge_pos,
        corners: corner_pos,
    };

    if !total_pos.directly_solvable() {
        std::mem::swap(&mut total_pos.edges.uf, &mut total_pos.edges.ur);
    }

    assert!(
        total_pos.directly_solvable(),
        "Total position should be solvable"
    );

    // then edge orientation ...
    let mut edge_orientation = EdgeOrientationState {
        uf: rng.gen_bool(0.5),
        ub: rng.gen_bool(0.5),
        ul: rng.gen_bool(0.5),
        ur: rng.gen_bool(0.5),
        fl: rng.gen_bool(0.5),
        fr: rng.gen_bool(0.5),
        bl: rng.gen_bool(0.5),
        br: rng.gen_bool(0.5),
        df: rng.gen_bool(0.5),
        db: rng.gen_bool(0.5),
        dl: rng.gen_bool(0.5),
        dr: rng.gen_bool(0.5),
    };

    if !edge_orientation.is_solvable() {
        edge_orientation.uf = !edge_orientation.uf;
    }

    assert!(edge_orientation.is_solvable());

    // then corner orientation ...
    let mut rand_orientation = || {
        let p: f64 = rng.gen_range(0.0..1.0);
        if p < 0.333 {
            CornerOrientation::Good
        } else if p < 0.667 {
            CornerOrientation::CW
        } else {
            CornerOrientation::CCW
        }
    };

    let mut corner_orientation = CornerOrientationState {
        ful: rand_orientation(),
        fur: rand_orientation(),
        fdl: rand_orientation(),
        fdr: rand_orientation(),
        bul: rand_orientation(),
        bur: rand_orientation(),
        bdl: rand_orientation(),
        bdr: rand_orientation(),
    };

    for _ in 0..3 {
        if !corner_orientation.is_solvable() {
            corner_orientation.ful = corner_orientation.ful + CornerOrientation::CW;
        }
    }

    assert!(corner_orientation.is_solvable());

    make_cube(
        total_pos.edges,
        total_pos.corners,
        edge_orientation,
        corner_orientation,
    )
}

fn make_edge_pos(edges: &Vec<SideCubelet>) -> CubeEdgePositions {
    assert_eq!(edges.len(), 12);

    CubeEdgePositions {
        uf: edges[0].clone(),
        ur: edges[1].clone(),
        ub: edges[2].clone(),
        ul: edges[3].clone(),
        fl: edges[4].clone(),
        fr: edges[5].clone(),
        bl: edges[6].clone(),
        br: edges[7].clone(),
        df: edges[8].clone(),
        dr: edges[9].clone(),
        db: edges[10].clone(),
        dl: edges[11].clone(),
    }
}

fn make_corner_pos(corners: &Vec<CornerCubelet>) -> CubeCornerPositions {
    assert_eq!(corners.len(), 8);

    CubeCornerPositions {
        ful: corners[0].clone(),
        fur: corners[1].clone(),
        bul: corners[2].clone(),
        bur: corners[3].clone(),
        fdl: corners[4].clone(),
        fdr: corners[5].clone(),
        bdl: corners[6].clone(),
        bdr: corners[7].clone(),
    }
}

fn get_corner_facelets(
    left_color: Facelet,
    right_color: Facelet,
    cubelet: CornerCubelet,
    orientation: CornerOrientation,
) -> [Facelet; 3] {
    // this is the set of facelets that is this corner, but the offset is a little dubious
    let mut facelets = CORNERS[cubelet.to_index() as usize].clone();

    // rotate so that the first element of the facelet array is indeed the side facelet
    let good_offset = (0..3_usize)
        .filter(|&i| facelets[i] == left_color || facelets[i] == right_color)
        .next()
        .expect("At least one facelet on a corner should be an L/R color");

    facelets.rotate_left(good_offset);

    assert!(facelets[0] == left_color || facelets[0] == right_color);

    // then rotate the array so that the element that's first is the one that, if placed in the
    // side position, would result in the desired orientation
    let offset = match orientation {
        CornerOrientation::Good => 0,
        CornerOrientation::CW => 1,
        CornerOrientation::CCW => 2,
    };

    facelets.rotate_right(offset);

    facelets
}

/// Returns the edge facelets for the given position, in some order (unspecified)
fn get_edge_facelets(cubelet: SideCubelet) -> [Facelet; 2] {
    EDGES[cubelet.to_index() as usize].clone()
}

fn set_corner_facelets(cube: &mut Cube, pos: CubeCornerPositions, orr: CornerOrientationState) {
    let l_color = cube.l.cc.clone();
    let r_color = cube.r.cc.clone();

    let set_facelets = |pos: CornerCubelet,
                        orr: CornerOrientation,
                        a: &mut Facelet,
                        b: &mut Facelet,
                        c: &mut Facelet| {
        let [a_new, b_new, c_new] = get_corner_facelets(l_color.clone(), r_color.clone(), pos, orr);
        *a = a_new;
        *b = b_new;
        *c = c_new;
    };

    // FUL, FUR, BUR, BUL
    set_facelets(
        pos.ful,
        orr.ful,
        &mut cube.l.uf,
        &mut cube.u.fl,
        &mut cube.f.ul,
    );
    set_facelets(
        pos.fur,
        orr.fur,
        &mut cube.r.uf,
        &mut cube.f.ur,
        &mut cube.u.fr,
    );
    set_facelets(
        pos.bur,
        orr.bur,
        &mut cube.r.ub,
        &mut cube.u.br,
        &mut cube.b.ur,
    );
    set_facelets(
        pos.bul,
        orr.bul,
        &mut cube.l.ub,
        &mut cube.b.ul,
        &mut cube.u.bl,
    );

    // FDL, FDR, BDR, BDL
    set_facelets(
        pos.fdl,
        orr.fdl,
        &mut cube.l.df,
        &mut cube.f.dl,
        &mut cube.d.fl,
    );
    set_facelets(
        pos.fdr,
        orr.fdr,
        &mut cube.r.df,
        &mut cube.d.fr,
        &mut cube.f.dr,
    );
    set_facelets(
        pos.bdr,
        orr.bdr,
        &mut cube.r.db,
        &mut cube.b.dr,
        &mut cube.d.br,
    );
    set_facelets(
        pos.bdl,
        orr.bdl,
        &mut cube.l.db,
        &mut cube.d.bl,
        &mut cube.b.dl,
    );
}

fn set_edge_facelets(cube: &mut Cube, pos: CubeEdgePositions, orr: EdgeOrientationState) {
    let l_color = cube.l.cc.clone();
    let r_color = cube.r.cc.clone();
    let u_color = cube.u.cc.clone();
    let d_color = cube.d.cc.clone();
    let f_color = cube.f.cc.clone();
    let b_color = cube.b.cc.clone();

    // TODO: these checks duplicated from edge_orientation_state
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

    let set_facelets_from_side =
        |pos: SideCubelet, orr: bool, side: &mut Facelet, non_side: &mut Facelet| {
            let [mut side_new, mut non_side_new] = get_edge_facelets(pos);

            // if the orientation of the facelets doesn't match the desired orientation, flip them

            if orr ^ lr_good(&side_new, &non_side_new) {
                std::mem::swap(&mut side_new, &mut non_side_new);
            }

            *side = side_new;
            *non_side = non_side_new;
        };

    let set_mid_facelets_from_ud =
        |pos: SideCubelet, orr: bool, ud: &mut Facelet, non_ud: &mut Facelet| {
            let [mut ud_new, mut non_ud_new] = get_edge_facelets(pos);

            // if the orientation of the facelets doesn't match the desired orientation, flip them

            if orr ^ ud_mid_good(&ud_new, &non_ud_new) {
                std::mem::swap(&mut ud_new, &mut non_ud_new);
            }

            *ud = ud_new;
            *non_ud = non_ud_new;
        };

    // left side first
    set_facelets_from_side(pos.ul, orr.ul, &mut cube.l.uc, &mut cube.u.lc);
    set_facelets_from_side(pos.bl, orr.bl, &mut cube.l.bc, &mut cube.b.lc);
    set_facelets_from_side(pos.dl, orr.dl, &mut cube.l.dc, &mut cube.d.lc);
    set_facelets_from_side(pos.fl, orr.fl, &mut cube.l.fc, &mut cube.f.lc);

    // then mid (orientation annoying here)
    set_mid_facelets_from_ud(pos.uf, orr.uf, &mut cube.u.fc, &mut cube.f.uc);
    set_mid_facelets_from_ud(pos.ub, orr.ub, &mut cube.u.bc, &mut cube.b.uc);
    set_mid_facelets_from_ud(pos.db, orr.db, &mut cube.d.bc, &mut cube.b.dc);
    set_mid_facelets_from_ud(pos.df, orr.df, &mut cube.d.fc, &mut cube.f.dc);

    // then right
    set_facelets_from_side(pos.ur, orr.ur, &mut cube.r.uc, &mut cube.u.rc);
    set_facelets_from_side(pos.fr, orr.fr, &mut cube.r.fc, &mut cube.f.rc);
    set_facelets_from_side(pos.dr, orr.dr, &mut cube.r.dc, &mut cube.d.rc);
    set_facelets_from_side(pos.br, orr.br, &mut cube.r.bc, &mut cube.b.rc);
}

pub fn make_cube(
    edge_pos: CubeEdgePositions,
    corner_pos: CubeCornerPositions,
    edge_or: EdgeOrientationState,
    corner_or: CornerOrientationState,
) -> Cube {
    // i really don't see a way out of how much this function sucks to write out
    let mut cube = Cube::make_solved(Facelet::Green, Facelet::Yellow);

    set_corner_facelets(&mut cube, corner_pos.clone(), corner_or.clone());
    set_edge_facelets(&mut cube, edge_pos.clone(), edge_or.clone());

    assert_eq!(
        CubeCornerPositions::from_cube(&cube),
        corner_pos,
        "Original corner position should be respected"
    );
    assert_eq!(
        CubeEdgePositions::from_cube(&cube),
        edge_pos,
        "Original edge position should be respected"
    );

    assert_eq!(
        CornerOrientationState::from_cube(&cube),
        corner_or,
        "Original corner orientation should be respected"
    );
    assert_eq!(
        EdgeOrientationState::from_cube(&cube),
        edge_or,
        "Original edge orientation should be respected"
    );

    cube
}

#[cfg(test)]
mod tests {
    use crate::corner_orientation_state::CornerOrientationState;
    use crate::edge_orientation_state::EdgeOrientationState;

    use super::*;

    /// strictly speaking this is a nondeterministic test but like ... whatever
    #[test]
    fn scramble_health_check() {
        const MAX_ATTEMPTS: usize = 100;

        for i in 0..MAX_ATTEMPTS {
            println!("Attempt {} of {}", i, MAX_ATTEMPTS);

            let cube = scramble_any();

            let pos = CubePositions::from_cube(&cube);
            assert!(pos.directly_solvable());

            let edge_or = EdgeOrientationState::from_cube(&cube);
            assert!(edge_or.is_solvable());

            let corner_or = CornerOrientationState::from_cube(&cube);
            assert!(corner_or.is_solvable());
        }
    }
}
