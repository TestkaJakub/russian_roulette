use std::fmt::Debug;

pub struct UVec<T> {
    items: Vec<T>,
}

impl<T: PartialEq> UVec<T> {
    /// Creates a new, empty `UVec`.
    pub fn new() -> Self {
        UVec { items: Vec::new() }
    }

    /// Adds an element to the `UVec`, ensuring no duplicates.
    pub fn add(&mut self, item: T) {
        if !self.items.contains(&item) {
            self.items.push(item);
        }
    }

    /// Removes an element by value, if it exists.
    pub fn remove(&mut self, item: &T) {
        if let Some(pos) = self.items.iter().position(|x| x == item) {
            self.items.remove(pos);
        }
    }

    /// Returns a reference to the inner vector.
    pub fn as_vec(&self) -> &Vec<T> {
        &self.items
    }

    /// Returns a mutable reference to the inner vector.
    pub fn as_mut_vec(&mut self) -> &mut Vec<T> {
        &mut self.items
    }
}

// Optional: Implement Debug for easier printing.
impl<T: Debug> Debug for UVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(&self.items).finish()
    }
}

// Optional: Implement IntoIterator for easy iteration.
impl<T> IntoIterator for UVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<T> UVec<T> {
    pub fn get_items(&self) -> &Vec<T> {
        &self.items
    }
}

impl<T> UVec<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }
}

impl<T> UVec<T> {
    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<T> UVec<T> {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.items.iter_mut()
    }
}

use std::ops::{Index, IndexMut};

impl<T> Index<usize> for UVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl<T> IndexMut<usize> for UVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[index]
    }
}