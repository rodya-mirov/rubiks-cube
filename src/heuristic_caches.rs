use crate::moves::{Amt, ApplyMove, CanMove, Dir, FullMove};
use ahash::{HashMap, HashSet};
use std::collections::VecDeque;
use std::hash::Hash;

const ALL_AMTS: [Amt; 3] = [Amt::One, Amt::Two, Amt::Rev];

pub struct HeuristicCache<StateType: Hash> {
    known_costs: HashMap<StateType, usize>,
}

impl<StateType> HeuristicCache<StateType>
where
    StateType: Hash + Eq + Clone + CanMove,
{
    pub fn from_goal(goal_state: StateType, free_dirs: &[Dir], half_dirs: &[Dir]) -> Self {
        let mut goal_states = HashSet::default();
        goal_states.insert(goal_state);
        Self::from_set(&goal_states, free_dirs, half_dirs)
    }

    pub fn from_set(
        goal_states: &HashSet<StateType>,
        free_dirs: &[Dir],
        half_dirs: &[Dir],
    ) -> Self {
        let mut known_costs = HashMap::default();

        let mut to_process: VecDeque<(StateType, usize)> = VecDeque::new();

        for c in goal_states {
            to_process.push_back((c.clone(), 0));
        }

        while let Some((pos, cost)) = to_process.pop_front() {
            let existing = known_costs.get(&pos);
            // note that by use of the VecDeque as a queue, we guarantee that we generate everything
            // in the most efficient manner possible, so if we've seen it before, this is not an
            // improvement on the previous
            if existing.is_some() {
                continue;
            }

            known_costs.insert(pos.clone(), cost);

            for dir in free_dirs.iter().copied() {
                for amt in ALL_AMTS {
                    let fm = FullMove { dir, amt };
                    let next = pos.clone().apply(fm);
                    let next_cost = cost + 1;
                    to_process.push_back((next, next_cost));
                }
            }
            for dir in half_dirs.iter().copied() {
                let amt = Amt::Two;
                let fm = FullMove { dir, amt };
                let next = pos.clone().apply(fm);
                let next_cost = cost + 1;
                to_process.push_back((next, next_cost));
            }
        }

        Self { known_costs }
    }
}

pub trait Heuristic<StateType> {
    fn evaluate(&self, state: &StateType) -> usize;
}

impl<StateType: Hash + Eq + PartialEq> Heuristic<StateType> for HeuristicCache<StateType> {
    fn evaluate(&self, state: &StateType) -> usize {
        if let Some(&cost) = self.known_costs.get(&state) {
            return cost;
        }

        panic!("Should have covered everything really nicely");
    }
}
