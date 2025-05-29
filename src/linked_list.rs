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
    fn new() -> Self {
        List {
            front: None,
            back: None,
            len: 0,
            marker: PhantomData,
        }
    }

    fn length(&self) -> usize {
        todo!()
    }

    fn append(&mut self, elem: char) {
        todo!()
    }

    fn insert(&mut self, elem: char, index: usize) -> Result<(), String> {
        todo!()
    }

    fn delete(&mut self, index: usize) -> Result<char, String> {
        todo!()
    }

    fn delete_all(&mut self, elem: char) {
        todo!()
    }

    fn get(&self, index: usize) -> Result<char, String> {
        todo!()
    }

    fn reverse(&self) {
        todo!()
    }

    fn find_first(&self, elem: char) -> i64 {
        todo!()
    }

    fn find_last(&self, elem: char) -> i64 {
        todo!()
    }

    fn clear(&mut self) {
        todo!()
    }

    fn extend(&mut self, other: &List) {
        todo!()
    }
}

impl Clone for List {
    fn clone(&self) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
