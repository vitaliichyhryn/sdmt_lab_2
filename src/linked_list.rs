use std::{marker::PhantomData, ptr::NonNull};

#[derive(Debug)]
pub struct List {
    front: Link,
    back: Link,
    len: usize,
    marker: PhantomData<Box<Node>>,
}

#[derive(Debug)]
struct Node {
    front: Link,
    back: Link,
    elem: char,
}

type Link = Option<NonNull<Node>>;

impl List {
    /// Creates an empty list
    pub fn new() -> Self {
        List {
            front: None,
            back: None,
            len: 0,
            marker: PhantomData,
        }
    }

    /// Returns the length of the list
    pub fn length(&self) -> usize {
        todo!()
    }

    /// Appends `elem` to the back of the list
    pub fn append(&mut self, elem: char) {
        todo!()
    }

    /// Inserts `elem` at position `index` of the list
    /// # Errors
    /// Returns an error if `index > len`
    pub fn insert(&mut self, elem: char, index: usize) -> Result<(), String> {
        todo!()
    }

    /// Deletes element at position `index` of the list
    /// # Errors
    /// Returns an error if `index` is out of bounds
    pub fn delete(&mut self, index: usize) -> Result<char, String> {
        todo!()
    }

    /// Deletes all occurences of `elem` in the list
    pub fn delete_all(&mut self, elem: char) {
        todo!()
    }

    /// Returns the element at position `index` of the list
    /// # Errors
    /// Returns an error if `index` is out of bounds
    pub fn get(&self, index: usize) -> Result<char, String> {
        todo!()
    }

    /// Reverses the list
    pub fn reverse(&self) {
        todo!()
    }

    /// Returns the position of the first occurrence of `elem` in the list
    ///
    /// If there were no occurrences of `elem`, returns `-1`
    pub fn find_first(&self, elem: char) -> i64 {
        todo!()
    }

    /// Returns the position of the last occurrence of `elem` in the list
    ///
    /// If there were no occurrences of `elem`, returns `-1`
    pub fn find_last(&self, elem: char) -> i64 {
        todo!()
    }

    /// Removes all elements from the list
    pub fn clear(&mut self) {
        todo!()
    }

    /// Copies all elements from `other` to the end of the list
    pub fn extend(&mut self, other: &List) {
        todo!()
    }
}

impl Clone for List {
    /// Returns a copy of the list
    fn clone(&self) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
