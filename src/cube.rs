pub trait FaceletKind {
    fn matches(&self, other: &Self) -> bool;
}

pub trait FaceKind {
    fn is_solved(&self) -> bool;
}

/// Fully known facelet
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum Facelet {
    Yellow,
    White,
    Green,
    Blue,
    Red,
    Orange,
}

// DO NOT reorder this array; it must be kept in sync with CornerCubelet::to_index
pub const CORNERS: [[Facelet; 3]; 8] = [
    // top corners first; FUL, FUR, BUR, BUL
    // given from top, clockwise
    [Facelet::Yellow, Facelet::Green, Facelet::Red],
    [Facelet::Yellow, Facelet::Orange, Facelet::Green],
    [Facelet::Yellow, Facelet::Blue, Facelet::Orange],
    [Facelet::Yellow, Facelet::Red, Facelet::Blue],
    // then bottom corners; FDL, FDR, BDR, BDL
    // given from bottom, clockwise
    [Facelet::White, Facelet::Red, Facelet::Green],
    [Facelet::White, Facelet::Green, Facelet::Orange],
    [Facelet::White, Facelet::Orange, Facelet::Blue],
    [Facelet::White, Facelet::Blue, Facelet::Red],
];

// DO NOT reorder this array; it must be kept in sync with SideCubelet::to_index
pub const EDGES: [[Facelet; 2]; 12] = [
    // left first; UL, FL, DL, BL
    // given from red (correctly oriented, if you put in from the left)
    [Facelet::Red, Facelet::Yellow],
    [Facelet::Red, Facelet::Green],
    [Facelet::Red, Facelet::White],
    [Facelet::Red, Facelet::Blue],
    // then the middle slice: FU, FD, BD, BU
    // given from front/back (green/blue)
    [Facelet::Green, Facelet::Yellow],
    [Facelet::Green, Facelet::White],
    [Facelet::Blue, Facelet::White],
    [Facelet::Blue, Facelet::Yellow],
    // then right face: UR, BR, DR, FR
    // given from orange (correctly oriented, if you put in from the right)
    [Facelet::Orange, Facelet::Yellow],
    [Facelet::Orange, Facelet::Blue],
    [Facelet::Orange, Facelet::White],
    [Facelet::Orange, Facelet::Green],
];

pub fn get_third_corner(front: Facelet, top: Facelet) -> Facelet {
    for corner in CORNERS {
        for i in 0..3 {
            if corner[i] == front && corner[(i + 1) % 3] == top {
                return corner[(i + 2) % 3].clone();
            }
        }
    }

    panic!(
        "There is no corner with front {:?} and top {:?}",
        front, top
    );
}

impl FaceletKind for Facelet {
    fn matches(&self, other: &Self) -> bool {
        self == other
    }
}

/// Facelet which is allowed to have the state "unknown." Used for stepwise solving a cube,
/// e.g. white cross, F2L, where you don't care what happens to the irrelevant facelets.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AmbigFacelet {
    Yellow,
    White,
    Green,
    Blue,
    Red,
    Orange,
    Unknown,
}

impl FaceletKind for AmbigFacelet {
    fn matches(&self, other: &Self) -> bool {
        self == &AmbigFacelet::Unknown || other == &AmbigFacelet::Unknown || self == other
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct LRFace<FaceletType> {
    // Field precedence order: UD / FB / LR
    // "C" is used to denote center; so corners have two coordinates,
    // edges have one coordinate then center, and true center is just cc
    pub ub: FaceletType,
    pub uc: FaceletType,
    pub uf: FaceletType,
    pub bc: FaceletType,
    pub cc: FaceletType,
    pub fc: FaceletType,
    pub db: FaceletType,
    pub dc: FaceletType,
    pub df: FaceletType,
}

impl<FaceletType: FaceletKind + Clone> LRFace<FaceletType> {
    pub fn make_solved(f: FaceletType) -> Self {
        Self {
            ub: f.clone(),
            uc: f.clone(),
            uf: f.clone(),
            bc: f.clone(),
            cc: f.clone(),
            fc: f.clone(),
            db: f.clone(),
            dc: f.clone(),
            df: f,
        }
    }
}

impl<FaceletType: FaceletKind> FaceKind for LRFace<FaceletType> {
    fn is_solved(&self) -> bool {
        self.cc.matches(&self.ub)
            && self.cc.matches(&self.uc)
            && self.cc.matches(&self.uf)
            && self.cc.matches(&self.bc)
            && self.cc.matches(&self.fc)
            && self.cc.matches(&self.db)
            && self.cc.matches(&self.dc)
            && self.cc.matches(&self.df)
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct UDFace<FaceletType> {
    // Field precedence order: UD / FB / LR
    // "C" is used to denote center; so corners have two coordinates,
    // edges have one coordinate then center, and true center is just cc
    pub bl: FaceletType,
    pub bc: FaceletType,
    pub br: FaceletType,
    pub lc: FaceletType,
    pub cc: FaceletType,
    pub rc: FaceletType,
    pub fl: FaceletType,
    pub fc: FaceletType,
    pub fr: FaceletType,
}

impl<FaceletType: FaceletKind + Clone> UDFace<FaceletType> {
    pub fn make_solved(f: FaceletType) -> Self {
        Self {
            bl: f.clone(),
            bc: f.clone(),
            br: f.clone(),
            lc: f.clone(),
            cc: f.clone(),
            rc: f.clone(),
            fl: f.clone(),
            fc: f.clone(),
            fr: f,
        }
    }
}

impl<FaceletType: FaceletKind> FaceKind for UDFace<FaceletType> {
    fn is_solved(&self) -> bool {
        self.cc.matches(&self.bl)
            && self.cc.matches(&self.bc)
            && self.cc.matches(&self.br)
            && self.cc.matches(&self.lc)
            && self.cc.matches(&self.rc)
            && self.cc.matches(&self.fl)
            && self.cc.matches(&self.fc)
            && self.cc.matches(&self.fr)
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct FBFace<FaceletType> {
    pub ul: FaceletType,
    pub uc: FaceletType,
    pub ur: FaceletType,
    pub lc: FaceletType,
    pub cc: FaceletType,
    pub rc: FaceletType,
    pub dl: FaceletType,
    pub dc: FaceletType,
    pub dr: FaceletType,
}

impl<FaceletType: FaceletKind + Clone> FBFace<FaceletType> {
    pub fn make_solved(f: FaceletType) -> Self {
        Self {
            ul: f.clone(),
            uc: f.clone(),
            lc: f.clone(),
            cc: f.clone(),
            rc: f.clone(),
            dl: f.clone(),
            dc: f.clone(),
            ur: f.clone(),
            dr: f,
        }
    }
}

impl<FaceletType: FaceletKind> FaceKind for FBFace<FaceletType> {
    fn is_solved(&self) -> bool {
        self.cc.matches(&self.ul)
            && self.cc.matches(&self.uc)
            && self.cc.matches(&self.ur)
            && self.cc.matches(&self.lc)
            && self.cc.matches(&self.rc)
            && self.cc.matches(&self.dl)
            && self.cc.matches(&self.dc)
            && self.cc.matches(&self.dr)
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Cube<FaceletType = Facelet> {
    pub u: UDFace<FaceletType>,
    pub d: UDFace<FaceletType>,
    pub l: LRFace<FaceletType>,
    pub r: LRFace<FaceletType>,
    pub f: FBFace<FaceletType>,
    pub b: FBFace<FaceletType>,
}

impl Cube<Facelet> {
    pub fn make_solved(front: Facelet, top: Facelet) -> Self {
        let right = get_third_corner(front.clone(), top.clone());
        let down = get_third_corner(front.clone(), right.clone());
        let left = get_third_corner(front.clone(), down.clone());
        let back = get_third_corner(right.clone(), top.clone());

        Self {
            u: UDFace::make_solved(top),
            d: UDFace::make_solved(down),
            l: LRFace::make_solved(left),
            r: LRFace::make_solved(right),
            f: FBFace::make_solved(front),
            b: FBFace::make_solved(back),
        }
    }
}

impl<F: FaceletKind> Cube<F> {
    pub fn is_solved(&self) -> bool {
        self.u.is_solved()
            && self.d.is_solved()
            && self.f.is_solved()
            && self.b.is_solved()
            && self.l.is_solved()
            && self.r.is_solved()
    }
}
