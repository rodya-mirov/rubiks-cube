use std::fmt::{Display, Formatter};

use crate::cube::{Cube, FBFace, LRFace, UDFace};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Dir {
    R,
    L,
    D,
    U,
    F,
    B,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Amt {
    One,
    Two,
    Rev,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct FullMove {
    pub dir: Dir,
    pub amt: Amt,
}

pub const ALL_DIRS: [Dir; 6] = [Dir::U, Dir::D, Dir::B, Dir::F, Dir::L, Dir::R];
pub const ALL_AMTS: [Amt; 3] = [Amt::One, Amt::Two, Amt::Rev];

/// Parses an input as a whitespace-separated list of moves. Panics on bad input because this is
/// not a production application.
pub fn parse_many(input: &str) -> Vec<FullMove> {
    input
        .split_ascii_whitespace()
        .map(|tok| {
            FullMove::try_from(tok)
                .map_err(|e| panic!("Bad input: {}", e))
                .unwrap()
        })
        .collect()
}

impl Display for FullMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.dir)?;

        match self.amt {
            Amt::One => {}
            Amt::Two => write!(f, "2")?,
            Amt::Rev => write!(f, "'")?,
        }

        Ok(())
    }
}

pub fn invert(fms: &[FullMove]) -> Vec<FullMove> {
    fn inv_fm(fm: FullMove) -> FullMove {
        let rev_amt: Amt = match fm.amt {
            Amt::Two => Amt::Two,
            Amt::One => Amt::Rev,
            Amt::Rev => Amt::One,
        };

        FullMove {
            dir: fm.dir,
            amt: rev_amt,
        }
    }

    let mut out = Vec::with_capacity(fms.len());

    for fm in fms.iter().copied().rev() {
        out.push(inv_fm(fm));
    }

    out
}

pub fn to_nice_str(fms: &[FullMove]) -> String {
    let mut moves = fms.iter().copied();

    let mut out = String::new();

    if let Some(next) = moves.next() {
        out.push_str(&format!("{next}"));
    }

    for next in moves {
        out.push(' ');
        out.push_str(&format!("{next}"));
    }

    out
}

impl<'a> TryFrom<&'a str> for FullMove {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        use Amt::*;
        use Dir::*;

        match value {
            "R" => Ok(FullMove { dir: R, amt: One }),
            "R2" => Ok(FullMove { dir: R, amt: Two }),
            "R'" => Ok(FullMove { dir: R, amt: Rev }),

            "L" => Ok(FullMove { dir: L, amt: One }),
            "L2" => Ok(FullMove { dir: L, amt: Two }),
            "L'" => Ok(FullMove { dir: L, amt: Rev }),

            "U" => Ok(FullMove { dir: U, amt: One }),
            "U2" => Ok(FullMove { dir: U, amt: Two }),
            "U'" => Ok(FullMove { dir: U, amt: Rev }),

            "D" => Ok(FullMove { dir: D, amt: One }),
            "D2" => Ok(FullMove { dir: D, amt: Two }),
            "D'" => Ok(FullMove { dir: D, amt: Rev }),

            "F" => Ok(FullMove { dir: F, amt: One }),
            "F2" => Ok(FullMove { dir: F, amt: Two }),
            "F'" => Ok(FullMove { dir: F, amt: Rev }),

            "B" => Ok(FullMove { dir: B, amt: One }),
            "B2" => Ok(FullMove { dir: B, amt: Two }),
            "B'" => Ok(FullMove { dir: B, amt: Rev }),

            other => Err(other),
        }
    }
}

pub trait ApplyMove: Sized {
    fn apply_many(self, moves: &[FullMove]) -> Self {
        let mut out = self;
        for m in moves {
            out = out.apply(*m);
        }
        out
    }

    fn apply(self, m: FullMove) -> Self;
}

impl<T> ApplyMove for T
where
    T: CanMove + Sized,
{
    fn apply(self, m: FullMove) -> Self {
        {
            let FullMove { dir, amt } = m;
            match dir {
                Dir::R => match amt {
                    Amt::One => self.r(),
                    Amt::Two => self.r().r(),
                    Amt::Rev => self.r().r().r(),
                },
                Dir::L => match amt {
                    Amt::One => self.l(),
                    Amt::Two => self.l().l(),
                    Amt::Rev => self.l().l().l(),
                },
                Dir::D => match amt {
                    Amt::One => self.d(),
                    Amt::Two => self.d_two(),
                    Amt::Rev => self.d().d().d(),
                },
                Dir::U => match amt {
                    Amt::One => self.u(),
                    Amt::Two => self.u_two(),
                    Amt::Rev => self.u().u().u(),
                },
                Dir::F => match amt {
                    Amt::One => self.f(),
                    Amt::Two => self.f().f(),
                    Amt::Rev => self.f().f().f(),
                },
                Dir::B => match amt {
                    Amt::One => self.b(),
                    Amt::Two => self.b().b(),
                    Amt::Rev => self.b().b().b(),
                },
            }
        }
    }
}

impl<A: CanMove, B: CanMove> CanMove for (A, B) {
    fn r(self) -> Self {
        (self.0.r(), self.1.r())
    }

    fn l(self) -> Self {
        (self.0.l(), self.1.l())
    }

    fn u(self) -> Self {
        (self.0.u(), self.1.u())
    }

    fn u_two(self) -> Self {
        (self.0.u_two(), self.1.u_two())
    }

    fn d(self) -> Self {
        (self.0.d(), self.1.d())
    }

    fn d_two(self) -> Self {
        (self.0.d_two(), self.1.d_two())
    }

    fn b(self) -> Self {
        (self.0.b(), self.1.b())
    }

    fn f(self) -> Self {
        (self.0.f(), self.1.f())
    }
}

pub trait CanMove: Sized {
    fn r(self) -> Self;

    // TODO: we may want to specialize this implementation with r_two, r_rev, and so on
    //          so they can be specifically overridden

    fn l(self) -> Self;

    fn u(self) -> Self;

    fn u_two(self) -> Self {
        self.u().u()
    }

    fn d(self) -> Self;

    fn d_two(self) -> Self {
        self.d().d()
    }

    fn b(self) -> Self;

    fn f(self) -> Self;
}

impl<F> CanMove for Cube<F> {
    #[inline(always)]
    fn r(self) -> Self {
        let Self { u, d, l, r, f, b } = self;

        Self {
            l,
            r: LRFace {
                // rotate corners
                ub: r.uf,
                uf: r.df,
                df: r.db,
                db: r.ub,
                // rotate edges
                uc: r.fc,
                fc: r.dc,
                dc: r.bc,
                bc: r.uc,
                // the center abides
                cc: r.cc,
            },
            d: UDFace {
                fr: b.dr,
                rc: b.rc,
                br: b.ur,
                ..d
            },
            u: UDFace {
                fr: f.dr,
                rc: f.rc,
                br: f.ur,
                ..u
            },
            f: FBFace {
                ur: d.fr,
                rc: d.rc,
                dr: d.br,
                ..f
            },
            b: FBFace {
                ur: u.fr,
                rc: u.rc,
                dr: u.br,
                ..b
            },
        }
    }

    #[inline(always)]
    fn l(self) -> Self {
        let Self { u, d, l, r, f, b } = self;

        Self {
            r,
            l: LRFace {
                // rotate the corners
                uf: l.ub,
                ub: l.db,
                db: l.df,
                df: l.uf,
                // rotate the edges
                fc: l.uc,
                uc: l.bc,
                bc: l.dc,
                dc: l.fc,
                // the center abides
                cc: l.cc,
            },
            u: UDFace {
                fl: b.ul,
                lc: b.lc,
                bl: b.dl,
                ..u
            },
            d: UDFace {
                fl: f.ul,
                lc: f.lc,
                bl: f.dl,
                ..d
            },
            f: FBFace {
                ul: u.bl,
                lc: u.lc,
                dl: u.fl,
                ..f
            },
            b: FBFace {
                ul: d.bl,
                lc: d.lc,
                dl: d.fl,
                ..b
            },
        }
    }

    #[inline(always)]
    fn u(self) -> Self {
        let Self { u, d, b, f, r, l } = self;

        Self {
            d,
            u: UDFace {
                // rotate corners
                fr: u.br,
                br: u.bl,
                bl: u.fl,
                fl: u.fr,
                // rotate edges
                fc: u.rc,
                rc: u.bc,
                bc: u.lc,
                lc: u.fc,
                // center abides
                cc: u.cc,
            },
            r: LRFace {
                uf: b.ur,
                uc: b.uc,
                ub: b.ul,
                ..r
            },
            l: LRFace {
                uf: f.ur,
                uc: f.uc,
                ub: f.ul,
                ..l
            },
            f: FBFace {
                ul: r.uf,
                uc: r.uc,
                ur: r.ub,
                ..f
            },
            b: FBFace {
                ur: l.ub,
                uc: l.uc,
                ul: l.uf,
                ..b
            },
        }
    }

    #[inline(always)]
    fn d(self) -> Self {
        let Self { u, d, f, b, r, l } = self;

        Self {
            u,
            d: UDFace {
                // rotate corners
                fr: d.fl,
                fl: d.bl,
                bl: d.br,
                br: d.fr,
                // rotate edges
                fc: d.lc,
                lc: d.bc,
                bc: d.rc,
                rc: d.fc,
                // center abides
                cc: d.cc,
            },
            r: LRFace {
                db: f.dr,
                dc: f.dc,
                df: f.dl,
                ..r
            },
            l: LRFace {
                df: b.dl,
                dc: b.dc,
                db: b.dr,
                ..l
            },
            b: FBFace {
                dl: r.db,
                dc: r.dc,
                dr: r.df,
                ..b
            },
            f: FBFace {
                dl: l.db,
                dc: l.dc,
                dr: l.df,
                ..f
            },
        }
    }

    #[inline(always)]
    fn b(self) -> Self {
        let Self { u, d, b, f, l, r } = self;
        Self {
            f,
            b: FBFace {
                // rotate edges
                ul: b.ur,
                ur: b.dr,
                dr: b.dl,
                dl: b.ul,
                // rotate corners
                uc: b.rc,
                rc: b.dc,
                dc: b.lc,
                lc: b.uc,
                // center abides
                cc: b.cc,
            },
            r: LRFace {
                ub: d.br,
                bc: d.bc,
                db: d.bl,
                ..r
            },
            l: LRFace {
                ub: u.br,
                bc: u.bc,
                db: u.bl,
                ..l
            },
            u: UDFace {
                bl: r.ub,
                bc: r.bc,
                br: r.db,
                ..u
            },
            d: UDFace {
                bl: l.ub,
                bc: l.bc,
                br: l.db,
                ..d
            },
        }
    }

    #[inline(always)]
    fn f(self) -> Self {
        let Self { u, d, b, f, l, r } = self;

        Self {
            b,
            f: FBFace {
                // rotate corners
                ul: f.dl,
                dl: f.dr,
                dr: f.ur,
                ur: f.ul,
                // rotate edges
                uc: f.lc,
                lc: f.dc,
                dc: f.rc,
                rc: f.uc,
                // center abides
                cc: f.cc,
            },
            r: LRFace {
                uf: u.fl,
                fc: u.fc,
                df: u.fr,
                ..r
            },
            l: LRFace {
                df: d.fr,
                fc: d.fc,
                uf: d.fl,
                ..l
            },
            u: UDFace {
                fl: l.df,
                fc: l.fc,
                fr: l.uf,
                ..u
            },
            d: UDFace {
                fl: r.df,
                fc: r.fc,
                fr: r.uf,
                ..d
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cube::Facelet;

    use super::*;

    fn solved() -> Cube {
        return Cube::make_solved(Facelet::White, Facelet::Blue);
    }

    #[test]
    fn simple_equality() {
        assert_eq!(solved(), solved())
    }

    #[test]
    fn simple_circuit_r() {
        assert_eq!(solved(), solved().r().r().r().r())
    }

    #[test]
    fn simple_circuit_l() {
        assert_eq!(solved(), solved().l().l().l().l())
    }

    #[test]
    fn simple_circuit_rl() {
        // random set of R and L, total of four each
        assert_eq!(solved(), solved().r().l().l().r().l().r().r().l())
    }

    #[test]
    fn simple_circuit_u() {
        assert_eq!(solved(), solved().u().u().u().u())
    }

    #[test]
    fn simple_circuit_d() {
        assert_eq!(solved(), solved().d().d().d().d())
    }

    #[test]
    fn simple_circuit_ud() {
        assert_eq!(solved(), solved().u().d().d().u().u().u().d().d())
    }

    #[test]
    fn simple_circuit_b() {
        assert_eq!(solved(), solved().b().b().b().b())
    }

    #[test]
    fn simple_circuit_f() {
        assert_eq!(solved(), solved().f().f().f().f())
    }

    #[test]
    fn simple_circuit_fb() {
        assert_eq!(solved(), solved().f().b().b().f().b().f().f().b())
    }

    fn moves_unsolved(input: &str) {
        let moves = parse_many(input);
        let actual = Cube::make_solved(Facelet::Orange, Facelet::White).apply_many(&moves);
        assert!(!actual.is_solved())
    }

    fn moves_solved(input: &str) {
        let moves = parse_many(input);
        let actual = Cube::make_solved(Facelet::Yellow, Facelet::Red).apply_many(&moves);
        assert!(actual.is_solved());
    }

    #[test]
    fn sanity_checks() {
        moves_unsolved("R2");
        moves_unsolved("L2 R2");
        moves_unsolved("R2 F");
        moves_unsolved("R2 B U D");

        moves_solved("R2 L R' L' R' F B F' B2 F2 B F2");
    }
}
