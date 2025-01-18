use crate::*;
use std::cmp::Ordering;

enum RbfsResult {
    Solution(Solution),
    Update(usize),
}

/// Additional information stored in a [`Node`] for RBFS
#[derive(Default)]
pub struct RbfsAddition {
    /// The updated weight
    value: usize,
    /// Previous slide action of the current node
    slide: Slide,
}

pub type RbfsNode = Node<RbfsAddition>;

#[derive(Eq)]
struct NodeWrap(RbfsNode);

impl PartialEq for NodeWrap {
    fn eq(&self, other: &Self) -> bool {
        self.0.data.value == other.0.data.value
    }
}

impl PartialOrd for NodeWrap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NodeWrap {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.data.value.cmp(&other.0.data.value).reverse()
    }
}

impl RbfsNode {
    fn rbfs_next(&self, slide: Slide) -> Option<Self> {
        let mut next = Self {
            puzzle: self.puzzle,
            dist: self.dist + 1,
            data: RbfsAddition { value: 0, slide },
        };
        if next.puzzle.slide(slide) {
            let weight = next.weight();
            next.data.value = weight.max(self.data.value);
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
                succ.insert(NodeWrap(next));
            }
        }
        if succ.is_empty() {
            return RbfsResult::Update(usize::MAX);
        }
        loop {
            let mut best = succ.pop_back().expect("successors should not be empty").0;
            if best.data.value > limit {
                return RbfsResult::Update(best.data.value);
            }
            let next_limit = if let Some(NodeWrap(alternative)) = succ.back() {
                alternative.data.value.min(limit)
            } else {
                limit
            };
            let result = best.rbfs_helper(next_limit);
            match result {
                RbfsResult::Update(value) => {
                    best.data.value = value;
                    succ.insert(NodeWrap(best));
                }
                RbfsResult::Solution(mut solution) => {
                    solution.0.push(best.data.slide);
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
