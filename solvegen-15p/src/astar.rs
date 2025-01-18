use crate::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::rc::Rc;

pub type AstarNode = Node<Rc<SolutionTree>>;

#[derive(Eq)]
struct NodeWrap(AstarNode);

impl PartialEq for NodeWrap {
    fn eq(&self, other: &Self) -> bool {
        self.0.weight() == other.0.weight()
    }
}

impl PartialOrd for NodeWrap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NodeWrap {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.weight().cmp(&other.0.weight()).reverse()
    }
}

impl AstarNode {
    fn astar_next(&self, slide: Slide) -> Option<Self> {
        let mut next = Self {
            puzzle: self.puzzle,
            dist: self.dist + 1,
            data: self.data.sub_tree(slide),
        };
        if next.puzzle.slide(slide) {
            Some(next)
        } else {
            None
        }
    }

    fn prev_slide(&self) -> Option<Slide> {
        self.data.prev()
    }

    pub fn astar(self) -> Option<Solution> {
        if self.puzzle.is_solved() {
            return Some(Solution::default());
        }

        let mut heap = BinaryHeap::from_iter([NodeWrap(self)]);
        loop {
            let node = heap.pop().expect("heap should not be empty").0;

            for slide in SLIDES {
                if node.prev_slide().is_none_or(|value| value != slide.opp()) {
                    if let Some(next) = node.astar_next(slide) {
                        if next.puzzle.is_solved() {
                            return Some(next.data.get());
                        }
                        heap.push(NodeWrap(next));
                    }
                }
            }
        }
    }
}
