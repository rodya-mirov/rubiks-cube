//! Set of functionality corresponding to a Thistlethwaite algorithm solution
//!
//! See for instance: https://www.jaapsch.net/puzzles/thistle.htm
//!
//! Basically this goes as follows:
//!     G0 is the set of all configurations of the cube reachable from <L, R, F, B, U, D>;
//!         that is, all solveable configurations of the cube
//!     G1 is the set of all configurations of the cube reachable from <L, R, F, B, U2, D2>;
//!         that is, where U and D cannot be used singly
//!     G2 is the set of all configurations of the cube reachable from <L, R, F2, B2, U2, D2>;
//!         that is, where only L and R can be used singly
//!     G3 is the set of all configurations of the cube reachable from <L2, R2, F2, B2, U2, D2>;
//!         that is, where only double moves are allowed
//!     G4 is just a solved cube (set of 1 configuration)
//!
//! What you do is, given a configuration, move it into G1 as quickly as possible; then into G2;
//! and so on, until it is solved. Because the coset spaces are relatively small, these problems
//! are individually manageable, and glue together into a not-terrible solution for the cube.
//!
//! Each group can (to some extent) be described "usefully" and then the solution from Gi to Gi+1
//! can be described in those terms.
//!
//! G1 -- an edge piece is "good" if, when it is moved into position using only LRBF moves,
//!         it gets there in the correct orientation. Otherwise it is bad.
//!
//!         Then: G1 is the set of all cube configurations where all the edges are "good."
//!
//!         This also suggests a strategy for moving from G0 to G1 -- a U or D rotation flips the
//!         "goodness" of all affected edges cubies, so we can compute the "goodness" state of a
//!         cube configuration, solve that, then apply the resulting set of moves to the original
//!         cube in order to get it into G1.
//!
//! G2 -- a corner piece is "good" if its "side" facelet is on a side. E.g. if your L and R faces
//!         are red and orange, then every corner cubelet has exactly one facelet which is either
//!         red or orange. Then that facelet can be _on_ a side, that is, on the L or R face, and if
//!         so, it is "good."
//!
//!         Then: G2 is the set of all cube configurations where all the corners are "good"
//!         and where every center edge piece (that is, FU, FD, BU, BD) is in the middle slice.
//!
//!         For this, note that L, R, F2, B2, U2, and D2 moves do not affect any of of the
//!         invariants. For the corner orientation, we should think of the orientation as a number
//!         mod 3 (where 0 is "good," 1 is "cw rotated," and 2 is "ccw rotated"); then F
//!         subtracts one from FUR and FDL (then moves them), while it adds one to FUL and FDR
//!         (then moves them) where all addition / subtraction is mod three. Likewise B
//!         subtracts one from BUR and BDL (then moves them), while it adds one to BUL and BDR
//!         (then moves them).
//!
//!         Thus the computed state is then twofold; a 2x2x2 cube of orientations (numbers mod 3)
//!         and twelve edge pieces which are bools (is middle slice / is not middle slice) which is
//!         solved when all the orientations are zero and the bools are in the right spot.
//!
//! G3 -- this is the trickiest to state clearly what it even is; the source material is unclear
//!         and lots of "plain English summaries" are inconsistent with each other. Doing my best.
//!
//!         What we want to ensure is that, (1) for the edge pieces, the L and R faces are in their
//!         correct slices, (2) the corners are in their correct tetrads, (3) the parity of the edge
//!         permutation is even (which ensures it for the corners as well), and (4) the total twist
//!         of each tetrad is fixed.
//!
//!         Going line by line, this means:
//!         1. We already know the edge pieces are correctly oriented (since we have gotten into G1)
//!             so on the side (say, red and orange) everything is already red or orange. This
//!             calls for getting them completely correct, at least for the edge pieces.
//!         2. A tetrad is a collection of four corner positions which are non-adjacent. Given a
//!             corner cubelet it's well-defined whether it's in the correct tetrad (the goal
//!             position is the correct tetrad; everything adjacent to that is not; everything
//!             adjacent to those _is_, and so on until you're done).
//!         3. and 4. The last bit is just making sure the permutation at the end is solvable in G3,
//!             which is not very easy to describe. It turns out a lot of people have trouble with
//!             this and so we'll come back to it ... someday.
//!
//!         Nevertheless, recalling that G3 is generated by <L, R, F2, B2, U2, D2>, we can describe
//!         the state in those terms. We can describe the state fourfold, according to the four
//!         invariants.
//!
//!         TODO: ugh I still don't understand G3 well enough to explain it

pub use g0g1::{solve_to_g1, G0toG1Cache};
pub use g1g2::{solve_to_g2, G1toG2Cache};
pub use g2g3::{solve_to_g3, G2toG3Cache};
pub use g3g4::{solve_to_g4, G3toG4Cache};

mod g0g1;
mod g1g2;
mod g2g3;
mod g3g4;

pub struct ThistlethwaiteCaches {
    pub g0g1cache: G0toG1Cache,
    pub g1g2cache: G1toG2Cache,
    pub g2g3cache: G2toG3Cache,
    pub g3g4cache: G3toG4Cache,
}

impl ThistlethwaiteCaches {
    pub fn initialize() -> Self {
        Self {
            g0g1cache: G0toG1Cache::initialize(),
            g1g2cache: G1toG2Cache::initialize(),
            g2g3cache: G2toG3Cache::initialize(),
            g3g4cache: G3toG4Cache::initialize(),
        }
    }
}
