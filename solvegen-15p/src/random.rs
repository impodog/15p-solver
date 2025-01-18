use rand::Rng;

use crate::*;

static MAX_RANDOM_TIMES: usize = 16;

impl Puzzle {
    /// Generates a random puzzle with approximate steps required to solve
    ///
    /// The steps given is an accurate upper bound of least steps needed to solve
    pub fn new_random_approx(steps: usize) -> Self {
        let mut puzzle = Self::new_solved();
        for _ in 0..steps {
            let mut times = 0;
            loop {
                let slide = SLIDES[rand::thread_rng().gen_range(0..4)];
                let mut next = puzzle;
                next.slide(slide);
                if next.heu() > puzzle.heu() {
                    puzzle = next;
                    break;
                } else {
                    times += 1;
                    if times == MAX_RANDOM_TIMES {
                        puzzle = next;
                        break;
                    }
                }
            }
        }
        puzzle
    }
}
