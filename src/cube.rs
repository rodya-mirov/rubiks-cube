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

impl Facelet {
    pub fn is_opposite(&self, other: Self) -> bool {
        match self {
            Facelet::Yellow => other == Facelet::White,
            Facelet::White => other == Facelet::Yellow,
            Facelet::Green => other == Facelet::Blue,
            Facelet::Blue => other == Facelet::Green,
            Facelet::Red => other == Facelet::Orange,
            Facelet::Orange => other == Facelet::Red,
        }
    }
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
    pub fn make_solved() -> Self {
        Self {
            u: UDFace::make_solved(Facelet::White),
            d: UDFace::make_solved(Facelet::Yellow),
            l: LRFace::make_solved(Facelet::Orange),
            r: LRFace::make_solved(Facelet::Red),
            f: FBFace::make_solved(Facelet::Green),
            b: FBFace::make_solved(Facelet::Blue),
        }
    }
}

impl <F: FaceletKind> Cube<F> {
    pub fn is_solved(&self) -> bool {
        self.u.is_solved() && self.d.is_solved() && self.f.is_solved() && self.b.is_solved() && self.l.is_solved() && self.r.is_solved()
    }
}
