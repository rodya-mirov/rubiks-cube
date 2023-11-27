use std::marker::PhantomData;

use crate::cube::Cube;
use crate::moves::{Amt, ApplyMove, CanMove, Dir, FullMove};

const ALL_AMTS: [Amt; 3] = [Amt::One, Amt::Two, Amt::Rev];

fn can_follow(last: Option<Dir>, next: Dir) -> bool {
    if last.is_none() {
        return true;
    }

    let last = last.unwrap();

    // can't repeat a direction, and if two directions commute, have to pick an order
    // so with no significance -- B before F, L before R, D before U
    if last == next {
        false
    } else if last == Dir::F && next == Dir::B {
        false
    } else if last == Dir::R && next == Dir::L {
        false
    } else if last == Dir::U && next == Dir::D {
        false
    } else {
        true
    }
}

pub fn solve<
    StateType: CanMove + Clone,
    ToState: FnOnce(&Cube) -> StateType,
    IsSolved: Fn(&StateType) -> bool,
>(
    cube: &Cube,
    free_dirs: &[Dir],
    half_move_dirs: &[Dir],
    to_state: ToState,
    is_solved: IsSolved,
    max_fuel: usize,
) -> Vec<FullMove> {
    let start_state = to_state(cube);

    struct IdaState<'a, StateType: CanMove, IsSolved: Fn(&StateType) -> bool> {
        free_dirs: &'a [Dir],
        half_move_dirs: &'a [Dir],
        st: PhantomData<StateType>,
        is_solved: IsSolved,
    }

    impl<'a, StateType: CanMove + Clone, IsSolved: Fn(&StateType) -> bool>
        IdaState<'a, StateType, IsSolved>
    {
        fn ida(&self, cube: &StateType, running: &mut Vec<FullMove>, max_depth: usize) -> bool {
            if (self.is_solved)(cube) {
                return true;
            } else if running.len() >= max_depth {
                return false;
            }

            // todo: the insides of these two loops are really similar
            for dir in self.half_move_dirs.iter().copied() {
                if !can_follow(running.last().map(|fm| fm.dir), dir) {
                    continue;
                }

                let amt = Amt::Two;

                let fm = FullMove { amt, dir };

                let next = cube.clone().apply(fm);

                running.push(fm);

                let found_solution = self.ida(&next, running, max_depth);

                if found_solution {
                    return true;
                }

                running.pop();
            }

            for dir in self.free_dirs.iter().copied() {
                if !can_follow(running.last().map(|fm| fm.dir), dir) {
                    continue;
                }

                for amt in ALL_AMTS.iter().copied() {
                    let fm = FullMove { amt, dir };
                    let next = cube.clone().apply(fm);

                    running.push(fm);

                    let found_solution = self.ida(&next, running, max_depth);

                    if found_solution {
                        return true;
                    }

                    running.pop();
                }
            }

            false
        }
    }

    let ida_state = IdaState {
        free_dirs,
        half_move_dirs,
        st: Default::default(),
        is_solved,
    };

    for fuel in 0..=max_fuel {
        let mut running = Vec::new();
        let solved = ida_state.ida(&start_state, &mut running, fuel);

        if solved {
            return running;
        }
    }

    panic!("Couldn't solve it I guess lol")
}
