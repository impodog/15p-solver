use smallvec::SmallVec;
use std::ops::Deref;

/// A sorted set maintained by insertion sort, with small values on front, big values on back
#[derive(Default)]
pub struct InsertSet<T> {
    list: SmallVec<[T; 4]>,
}

impl<T> Deref for InsertSet<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.list.as_ref()
    }
}

impl<T> InsertSet<T>
where
    T: Ord,
{
    /// Creates an empty set
    pub fn new() -> Self {
        Self {
            list: SmallVec::new(),
        }
    }

    /// Inserts the element to a sorted position
    pub fn insert(&mut self, elem: T) {
        let mut index = self.list.len();
        self.list.push(elem);
        while index > 0 && self.list[index] < self.list[index - 1] {
            self.list.swap(index - 1, index);
            index -= 1;
        }
    }

    /// Pops the smallest element
    pub fn pop_front(&mut self) -> Option<T> {
        if self.list.is_empty() {
            None
        } else {
            Some(self.list.remove(0))
        }
    }

    /// Pops the biggest element
    pub fn pop_back(&mut self) -> Option<T> {
        self.list.pop()
    }

    pub fn front(&mut self) -> Option<&T> {
        self.list.first()
    }

    pub fn back(&mut self) -> Option<&T> {
        self.list.last()
    }
}
