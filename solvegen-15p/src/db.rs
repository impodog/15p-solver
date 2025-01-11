use crate::*;
use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use std::sync::LazyLock;

/// The number of sliders in a database
pub const DB_SIZE: usize = 4;
/// The number of databases required to store all sliders
pub const DB_NUMBER: usize = SQUARE_LENGTH_USIZE.div_ceil(DB_SIZE);

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Default)]
/// An entry key in [`Db`], mapping from each slider to their position
pub struct DbKey(pub [Value; DB_SIZE + 1]);

pub struct Db {
    map: HashMap<DbKey, usize>,
    /// Enabled on the last column to filter the space.
    /// The last value MUST be [`SQUARE_LENGTH`] - 1 when enabled
    ignore_last: bool,
    begin: DbKey,
}

impl Deref for Db {
    type Target = HashMap<DbKey, usize>;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl Db {
    fn calc_helper(mut self) -> Self {
        let mut queue: VecDeque<DbKey> = Default::default();
        queue.push_back(self.begin);
        self.map.insert(self.begin, 0);

        while let Some(key) = queue.pop_front() {
            let value = *self
                .map
                .get(&key)
                .expect("keys in the queue should be inserted in the map");
            let space_pos = key.0[0];
            for slide in SLIDES {
                if let Some(number) = slide.slide(space_pos) {
                    // Check for overlaps
                    let overlap = {
                        let mut overlap = None;
                        for (index, pos) in key.0.iter().enumerate().skip(1) {
                            if (!self.ignore_last || index != DB_SIZE) && *pos == number {
                                overlap = Some(index);
                            }
                        }
                        overlap
                    };

                    let mut next = key;
                    next.0[0] = number;
                    if let Some(overlap) = overlap {
                        next.0[overlap] = space_pos;
                    }

                    if let std::collections::hash_map::Entry::Vacant(e) = self.map.entry(next) {
                        e.insert(value + 1);
                        queue.push_back(next);
                    }
                }
            }
        }

        self
    }

    /// Calculates a new database based on given [`DbKey`]
    pub fn calc(begin: DbKey, ignore_last: bool) -> Self {
        Self {
            map: Default::default(),
            ignore_last,
            begin,
        }
        .calc_helper()
    }
}

pub struct DbList(Vec<Db>);

impl Deref for DbList {
    type Target = Vec<Db>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DbList {
    pub fn new() -> Self {
        let mut list = Vec::new();
        let mut key = DbKey([SQUARE_LENGTH - 1, 0, 1, 2, 3]);
        for number in 0..DB_NUMBER {
            list.push(Db::calc(key, number + 1 == DB_NUMBER));
            for index in 1..=DB_SIZE {
                key.0[index] += DB_SIZE as Value;
            }
        }
        Self(list)
    }
}

impl Default for DbList {
    fn default() -> Self {
        Self::new()
    }
}

pub static DB_LIST: LazyLock<DbList> = LazyLock::new(DbList::new);
