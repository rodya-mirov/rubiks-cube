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
    IsSolved: Fn(&StateType) -> bool,
    CostHeuristic: FnMut(&StateType) -> usize,
>(
    start_state: StateType,
    free_dirs: &[Dir],
    half_move_dirs: &[Dir],
    is_solved: IsSolved,
    cost_heuristic: CostHeuristic,
    max_fuel: usize,
) -> Vec<FullMove> {
    struct IdaState<'a, IsSolved, CostHeuristic> {
        free_dirs: &'a [Dir],
        half_move_dirs: &'a [Dir],
        is_solved: IsSolved,
        cost_heuristic: CostHeuristic,
    }

    // TODO perf: strictly speaking we are able to increment the max_depth a little faster
    //      if we jump to "lowest cost of a pruned node" instead of just going up by one, but it's
    //      code complexity I don't yet want to deal with

    fn ida<
        'a,
        StateType: CanMove + Clone,
        IsSolved: Fn(&StateType) -> bool,
        CostHeuristic: FnMut(&StateType) -> usize,
    >(
        ida_state: &mut IdaState<'a, IsSolved, CostHeuristic>,
        cube: &StateType,
        running: &mut Vec<FullMove>,
        max_depth: usize,
    ) -> bool {
        if (ida_state.is_solved)(cube) {
            return true;
        } else if running.len() + (ida_state.cost_heuristic)(cube) >= max_depth {
            return false;
        }

        // todo: the insides of these two loops are really similar
        for dir in ida_state.half_move_dirs.iter().copied() {
            if !can_follow(running.last().map(|fm| fm.dir), dir) {
                continue;
            }

            let amt = Amt::Two;

            let fm = FullMove { amt, dir };

            let next = cube.clone().apply(fm);

            running.push(fm);

            let found_solution = ida(ida_state, &next, running, max_depth);

            if found_solution {
                return true;
            }

            running.pop();
        }

        for dir in ida_state.free_dirs.iter().copied() {
            if !can_follow(running.last().map(|fm| fm.dir), dir) {
                continue;
            }

            for amt in ALL_AMTS.iter().copied() {
                let fm = FullMove { amt, dir };
                let next = cube.clone().apply(fm);

                running.push(fm);

                let found_solution = ida(ida_state, &next, running, max_depth);

                if found_solution {
                    return true;
                }

                running.pop();
            }
        }

        false
    }

    // needs to be mutable because the cost heuristic is stateful
    let mut ida_state = IdaState {
        free_dirs,
        half_move_dirs,
        is_solved,
        cost_heuristic,
    };

    for fuel in 0..=max_fuel {
        let mut running = Vec::new();
        let solved = ida(&mut ida_state, &start_state, &mut running, fuel);

        if solved {
            return running;
        }
    }

    panic!("Couldn't solve it I guess lol")
}
