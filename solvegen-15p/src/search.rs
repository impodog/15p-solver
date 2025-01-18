use crate::*;
use std::cmp::Ordering;
use std::rc::Rc;
use std::sync::{LazyLock, RwLock};

#[derive(Debug, Default)]
pub struct Config {
    /// Configuration of heuristic weight, use [`None`] for most accurate results, or a value
    /// higher than 1.0 for fuzzy results
    pub weight: Option<f32>,
}

pub static CONFIG: LazyLock<RwLock<Config>> = LazyLock::new(Default::default);

#[derive(Default)]
pub struct Solution(pub Vec<Slide>);

#[derive(Default)]
pub struct SolutionTree(Option<(Rc<SolutionTree>, Slide)>);

impl SolutionTree {
    pub fn new() -> Rc<Self> {
        Rc::new(Self::default())
    }

    pub fn sub_tree(self: &Rc<Self>, slide: Slide) -> Rc<Self> {
        Rc::new(Self(Some((self.clone(), slide))))
    }

    pub fn prev(&self) -> Option<Slide> {
        self.0.as_ref().map(|(_, slide)| *slide)
    }

    pub fn get(&self) -> Solution {
        let mut list = Vec::new();
        let mut this = self;
        while let Some((tree, slide)) = &this.0 {
            list.push(*slide);
            this = tree.as_ref();
        }
        list.reverse();
        Solution(list)
    }
}

pub struct Node<T> {
    pub(crate) puzzle: Puzzle,
    pub(crate) dist: usize,
    pub data: T,
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.weight() == other.weight()
    }
}

impl<T> Eq for Node<T> {}

impl<T> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.weight().cmp(&self.weight()))
    }
}

impl<T> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight().cmp(&self.weight())
    }
}

impl<T> Node<T> {
    /// Creates an initial node
    pub fn new(puzzle: Puzzle, data: T) -> Self {
        Self {
            puzzle,
            dist: 0,
            data,
        }
    }

    /// Gets the search weight value of the node
    pub(crate) fn weight(&self) -> usize {
        let heu = self.puzzle.heu();
        let heu = match CONFIG.read().unwrap().weight {
            Some(value) => (heu as f32 * value) as usize,
            None => heu,
        };
        heu + self.dist
    }
}
