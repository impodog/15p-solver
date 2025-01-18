/// The solver uses this type to save some memory for small numbers
pub type Value = u8;
pub const LENGTH: Value = 4;
pub const SQUARE_LENGTH: Value = LENGTH * LENGTH;
pub const SQUARE_LENGTH_USIZE: usize = LENGTH as usize * LENGTH as usize;

#[derive(Debug, Clone, Copy)]
pub struct Puzzle {
    /// Map from position to slider number
    slider: [Value; SQUARE_LENGTH_USIZE],
    /// Map from slider number to position
    pos: [Value; SQUARE_LENGTH_USIZE],
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
/// Represents a sliding actions, from the space to a target nearby
pub enum Slide {
    #[default]
    Left,
    Right,
    Up,
    Down,
}

pub const SLIDES: [Slide; 4] = [Slide::Left, Slide::Right, Slide::Up, Slide::Down];

impl Slide {
    /// Returns the opposite sliding direction
    pub fn opp(self) -> Slide {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }

    /// Returns the slid position of a slider, if any
    pub fn slide(self, pos: Value) -> Option<Value> {
        match self {
            Slide::Left => {
                if pos % LENGTH == 3 {
                    None
                } else {
                    Some(pos + 1)
                }
            }
            Slide::Right => {
                if pos % LENGTH == 0 {
                    None
                } else {
                    Some(pos - 1)
                }
            }
            Slide::Up => {
                if pos >= SQUARE_LENGTH - LENGTH {
                    None
                } else {
                    Some(pos + LENGTH)
                }
            }
            Slide::Down => {
                if pos < LENGTH {
                    None
                } else {
                    Some(pos - LENGTH)
                }
            }
        }
    }
}

impl Puzzle {
    /// Returns the solved state slider at the position
    pub(crate) fn initial_slider(pos: usize) -> usize {
        if pos + 1 == SQUARE_LENGTH_USIZE {
            0
        } else {
            pos + 1
        }
    }

    /// Returns a puzzle at its solved state
    pub fn new_solved() -> Self {
        Self::from_slider([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0])
    }

    /// Creates a puzzle status with given map from position to slider,
    /// panics if the puzzle is invalid
    pub fn from_slider(slider: [Value; SQUARE_LENGTH_USIZE]) -> Self {
        let mut pos = [Value::default(); SQUARE_LENGTH_USIZE];
        for (slider_index, index) in slider.iter().copied().enumerate() {
            let index = index as usize;
            pos[index] = slider_index as Value;
        }
        Self { slider, pos }
    }

    /// Returns true if both positions are in bounds
    pub fn swap(&mut self, lhs: Value, rhs: Value) -> bool {
        let lhs = lhs as usize;
        let rhs = rhs as usize;
        if lhs < self.slider.len() && rhs < self.slider.len() {
            self.slider.swap(lhs, rhs);
            let lhs = self.slider[lhs] as usize;
            let rhs = self.slider[rhs] as usize;
            self.pos.swap(lhs, rhs);
            true
        } else {
            false
        }
    }

    /// Returns true if the sliding action is legal and done
    pub fn slide(&mut self, slide: Slide) -> bool {
        let space_pos = self.pos[0];
        if let Some(target_pos) = slide.slide(space_pos) {
            self.swap(space_pos, target_pos)
        } else {
            false
        }
    }

    /// Get position of slider, if any
    pub fn get_pos(&self, slider: Value) -> Option<&Value> {
        self.slider.get(slider as usize)
    }

    /// Get the slider on the position, if any
    pub fn get_slider(&self, pos: Value) -> Option<&Value> {
        self.pos.get(pos as usize)
    }

    /// Returns the map from slider to position
    pub fn slider_to_pos(&self) -> &[Value; SQUARE_LENGTH_USIZE] {
        &self.pos
    }

    /// Returns the map from position to slider
    pub fn pos_to_slider(&self) -> &[Value; SQUARE_LENGTH_USIZE] {
        &self.slider
    }

    /// Test if the puzzle is solved state
    pub fn is_solved(&self) -> bool {
        for pos in 0..SQUARE_LENGTH_USIZE {
            if self.slider[pos] as usize != Puzzle::initial_slider(pos) {
                return false;
            }
        }
        true
    }
}
