use crate::*;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::sync::{LazyLock, RwLock};

#[derive(Debug, Default)]
pub struct Config {
    /// Configuration of heuristic weight, use [`None`] for most accurate results, or a value
    /// higher than 1.0 for fuzzy results
    pub weight: Option<f32>,
}

pub static CONFIG: LazyLock<RwLock<Config>> = LazyLock::new(Default::default);

pub struct Node {
    puzzle: Puzzle,
    dist: usize,
    value: usize,
    slide: Slide,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.value.cmp(&self.value))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.value.cmp(&self.value)
    }
}

#[derive(Default)]
pub struct Solution(pub Vec<Slide>);

enum RbfsResult {
    Solution(Solution),
    Update(usize),
}

impl Node {
    /// Creates an initial node for RBFS
    pub fn new(puzzle: Puzzle) -> Self {
        Self {
            puzzle,
            dist: 0,
            value: 0,
            slide: Default::default(),
        }
    }

    /// Gets the search weight value of the node
    fn weight(&self) -> usize {
        let heu = self.puzzle.heu();
        let heu = match CONFIG.read().unwrap().weight {
            Some(value) => (heu as f32 * value) as usize,
            None => heu,
        };
        heu + self.dist
    }

    fn rbfs_next(&self, slide: Slide) -> Option<Self> {
        let mut next = Self {
            puzzle: self.puzzle,
            dist: self.dist + 1,
            value: self.value,
            slide,
        };
        if next.puzzle.slide(slide) {
            let weight = next.weight();
            next.value = weight.max(next.value);
            Some(next)
        } else {
            None
        }
    }

    fn rbfs_helper(&mut self, limit: usize) -> RbfsResult {
        if self.puzzle.is_solved() {
            return RbfsResult::Solution(Solution::default());
        }

        let mut succ = InsertSet::new();
        for slide in SLIDES {
            if let Some(next) = self.rbfs_next(slide) {
                succ.insert(next);
            }
        }
        if succ.is_empty() {
            return RbfsResult::Update(usize::MAX);
        }
        loop {
            let mut best = succ.pop_back().expect("successors should not be empty");
            if best.value > limit {
                return RbfsResult::Update(best.value);
            }
            let next_limit = if let Some(alternative) = succ.back() {
                alternative.value.min(limit)
            } else {
                limit
            };
            let result = best.rbfs_helper(next_limit);
            match result {
                RbfsResult::Update(value) => {
                    best.value = value;
                    succ.insert(best);
                }
                RbfsResult::Solution(mut solution) => {
                    solution.0.push(best.slide);
                    return RbfsResult::Solution(solution);
                }
            }
        }
    }

    /// Computes the solution using RBFS method
    pub fn rbfs(mut self) -> Option<Solution> {
        match self.rbfs_helper(usize::MAX) {
            RbfsResult::Solution(mut solution) => {
                solution.0.reverse();
                Some(solution)
            }
            _ => None,
        }
    }
}
