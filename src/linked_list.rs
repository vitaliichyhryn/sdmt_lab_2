use std::{fmt::Debug, marker::PhantomData, ptr::NonNull};

pub struct List {
    front: Link,
    back: Link,
    len: usize,
    marker: PhantomData<Box<Node>>,
}

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
        self.len
    }

    /// Appends `elem` to the back of the list
    pub fn append(&mut self, elem: char) {
        unsafe {
            let new_back = Box::new(Node {
                front: None,
                back: None,
                elem,
            });
            let new_back = NonNull::new_unchecked(Box::into_raw(new_back));

            if let Some(old_back) = self.back {
                (*old_back.as_ptr()).back = Some(new_back);
                (*new_back.as_ptr()).front = Some(old_back);
            } else {
                self.front = Some(new_back);
            }

            self.back = Some(new_back);
            self.len += 1;
        }
    }

    /// Inserts `elem` at position `index` of the list
    /// # Errors
    /// Returns an error if `index > len`
    pub fn insert(&mut self, elem: char, index: usize) -> Result<(), String> {
        unsafe {
            if index > self.len {
                return Err("Index out of bounds".to_owned());
            }

            let new_node = Box::new(Node {
                front: None,
                back: None,
                elem,
            });
            let new_node = NonNull::new_unchecked(Box::into_raw(new_node));

            let (front_node, back_node) = match index {
                0 => (None, self.front),
                _ if index == self.len => (self.back, None),
                index => {
                    let mut front_node = self.front.unwrap();
                    for _ in 0..(index - 1) {
                        front_node = (*front_node.as_ptr()).back.unwrap();
                    }
                    let back_node = (*front_node.as_ptr()).back.unwrap();
                    (Some(front_node), Some(back_node))
                }
            };

            (*new_node.as_ptr()).front = front_node;
            (*new_node.as_ptr()).back = back_node;

            if let Some(front_node) = front_node {
                (*front_node.as_ptr()).back = Some(new_node);
            } else {
                self.front = Some(new_node);
            }
            if let Some(back_node) = back_node {
                (*back_node.as_ptr()).front = Some(new_node);
            } else {
                self.back = Some(new_node);
            }

            self.len += 1;
            Ok(())
        }
    }

    /// Deletes element at position `index` of the list
    /// # Errors
    /// Returns an error if `index` is out of bounds
    pub fn delete(&mut self, index: usize) -> Result<char, String> {
        unsafe {
            if index >= self.len {
                return Err("Index out of bounds".to_owned());
            }

            let target_node = match index {
                0 => self.front.unwrap(),
                _ if index == self.len => self.back.unwrap(),
                index => {
                    let mut target_node = self.front.unwrap();
                    for _ in 0..index {
                        target_node = (*target_node.as_ptr()).back.unwrap();
                    }
                    target_node
                }
            };

            let front_node = (*target_node.as_ptr()).front;
            let back_node = (*target_node.as_ptr()).back;

            if let Some(front_node) = front_node {
                (*front_node.as_ptr()).back = back_node;
            } else {
                self.front = back_node;
            }
            if let Some(back_node) = back_node {
                (*back_node.as_ptr()).front = front_node;
            } else {
                self.back = front_node;
            }

            let target_node = Box::from_raw(target_node.as_ptr());
            self.len -= 1;
            Ok(target_node.elem)
        }
    }

    /// Deletes all occurences of `elem` in the list
    pub fn delete_all(&mut self, elem: char) {
        loop {
            let index = self.find_first(elem);
            if index == -1 {
                break;
            }
            let _ = self.delete(index as usize);
        }
    }

    /// Returns the element at position `index` of the list
    /// # Errors
    /// Returns an error if `index` is out of bounds
    pub fn get(&self, index: usize) -> Result<char, String> {
        unsafe {
            if index >= self.len {
                return Err("Index out of bounds".to_owned());
            }

            match index {
                0 => Ok((*self.front.unwrap().as_ptr()).elem),
                _ if index == self.len => Ok((*self.back.unwrap().as_ptr()).elem),
                index => {
                    let mut target_node = self.front.unwrap();
                    for _ in 0..index {
                        target_node = (*target_node.as_ptr()).back.unwrap();
                    }
                    Ok((*target_node.as_ptr()).elem)
                }
            }
        }
    }

    /// Reverses the list
    pub fn reverse(&mut self) {
        unsafe {
            std::mem::swap(&mut self.front, &mut self.back);
            let mut current_node = self.front;
            while let Some(node) = current_node {
                std::mem::swap(&mut (*node.as_ptr()).front, &mut (*node.as_ptr()).back);
                current_node = (*node.as_ptr()).back;
            }
        }
    }

    /// Returns the position of the first occurrence of `elem` in the list
    ///
    /// If there were no occurrences of `elem`, returns `-1`
    pub fn find_first(&self, elem: char) -> i64 {
        unsafe {
            let mut current_node = self.front;
            let mut index = 0;

            while let Some(node) = current_node {
                if (*node.as_ptr()).elem == elem {
                    return index;
                }
                current_node = (*node.as_ptr()).back;
                index += 1
            }
            -1
        }
    }

    /// Returns the position of the last occurrence of `elem` in the list
    ///
    /// If there were no occurrences of `elem`, returns `-1`
    pub fn find_last(&self, elem: char) -> i64 {
        unsafe {
            let mut current_node = self.back;
            let mut index = self.len as i64 - 1;

            while let Some(node) = current_node {
                if (*node.as_ptr()).elem == elem {
                    return index;
                }
                current_node = (*node.as_ptr()).front;
                index -= 1
            }
            -1
        }
    }

    /// Removes all elements from the list
    pub fn clear(&mut self) {
        while self.len != 0 {
            let _ = self.delete(0);
        }
    }

    /// Copies all elements from `other` to the end of the list
    pub fn extend(&mut self, other: &List) {
        for index in 0..other.len {
            self.append(other.get(index).unwrap());
        }
    }
}

impl Clone for List {
    /// Returns a copy of the list
    fn clone(&self) -> Self {
        let mut copy = Self::new();
        for index in 0..self.len {
            copy.append(self.get(index).unwrap())
        }
        copy
    }
}

impl Drop for List {
    fn drop(&mut self) {
        self.clear()
    }
}

impl Debug for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            let mut elems = Vec::new();

            let mut current_node = self.front;
            while let Some(node) = current_node {
                elems.push((*node.as_ptr()).elem);
                current_node = (*node.as_ptr()).back;
            }

            f.debug_list().entries(&elems).finish()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let list = List::new();
        assert_eq!(list.length(), 0);
    }

    #[test]
    fn test_length() {
        let mut list = List::new();
        assert_eq!(list.length(), 0);

        list.append('a');
        assert_eq!(list.length(), 1);

        list.append('b');
        assert_eq!(list.length(), 2);

        let _ = list.delete(0);
        assert_eq!(list.length(), 1);

        let _ = list.delete(0);
        assert_eq!(list.length(), 0);
    }

    #[test]
    fn test_append() {
        let mut list = List::new();

        list.append('a');
        assert_eq!(list.length(), 1);
        assert_eq!(list.get(0).unwrap(), 'a');

        list.append('b');
        assert_eq!(list.length(), 2);
        assert_eq!(list.get(0).unwrap(), 'a');
        assert_eq!(list.get(1).unwrap(), 'b');

        list.append('c');
        assert_eq!(list.length(), 3);
        assert_eq!(list.get(2).unwrap(), 'c');
    }

    #[test]
    fn test_insert_valid() {
        let mut list = List::new();

        // Insert into empty list
        assert!(list.insert('a', 0).is_ok());
        assert_eq!(list.length(), 1);
        assert_eq!(list.get(0).unwrap(), 'a');

        // Insert at beginning
        assert!(list.insert('b', 0).is_ok());
        assert_eq!(list.length(), 2);
        assert_eq!(list.get(0).unwrap(), 'b');
        assert_eq!(list.get(1).unwrap(), 'a');

        // Insert at end
        assert!(list.insert('c', 2).is_ok());
        assert_eq!(list.length(), 3);
        assert_eq!(list.get(2).unwrap(), 'c');

        // Insert in middle
        assert!(list.insert('d', 1).is_ok());
        assert_eq!(list.length(), 4);
        assert_eq!(list.get(1).unwrap(), 'd');
        assert_eq!(list.get(2).unwrap(), 'a');
    }

    #[test]
    fn test_insert_invalid() {
        let mut list = List::new();

        // Insert at invalid index in empty list
        assert!(list.insert('a', 1).is_err());

        list.append('a');
        list.append('b');

        // Insert at index > length
        assert!(list.insert('c', 3).is_err());
        assert_eq!(list.length(), 2); // List should be unchanged
    }

    #[test]
    fn test_delete_valid() {
        let mut list = List::new();
        list.append('a');
        list.append('b');
        list.append('c');

        // Delete from middle
        assert_eq!(list.delete(1).unwrap(), 'b');
        assert_eq!(list.length(), 2);
        assert_eq!(list.get(0).unwrap(), 'a');
        assert_eq!(list.get(1).unwrap(), 'c');

        // Delete from beginning
        assert_eq!(list.delete(0).unwrap(), 'a');
        assert_eq!(list.length(), 1);
        assert_eq!(list.get(0).unwrap(), 'c');

        // Delete last element
        assert_eq!(list.delete(0).unwrap(), 'c');
        assert_eq!(list.length(), 0);
    }

    #[test]
    fn test_delete_invalid() {
        let mut list = List::new();

        // Delete from empty list
        assert!(list.delete(0).is_err());

        list.append('a');
        list.append('b');

        // Delete at invalid index
        assert!(list.delete(2).is_err());
        assert!(list.delete(10).is_err());
        assert_eq!(list.length(), 2); // List should be unchanged
    }

    #[test]
    fn test_delete_all() {
        let mut list = List::new();

        // Delete from empty list
        list.delete_all('a');
        assert_eq!(list.length(), 0);

        // Add some elements with duplicates
        list.append('a');
        list.append('b');
        list.append('a');
        list.append('c');
        list.append('a');
        assert_eq!(list.length(), 5);

        // Delete all occurrences of 'a'
        list.delete_all('a');
        assert_eq!(list.length(), 2);
        assert_eq!(list.get(0).unwrap(), 'b');
        assert_eq!(list.get(1).unwrap(), 'c');

        // Delete non-existent element
        list.delete_all('x');
        assert_eq!(list.length(), 2);

        // Delete all remaining elements
        list.delete_all('b');
        list.delete_all('c');
        assert_eq!(list.length(), 0);
    }

    #[test]
    fn test_get_valid() {
        let mut list = List::new();
        list.append('a');
        list.append('b');
        list.append('c');

        assert_eq!(list.get(0).unwrap(), 'a');
        assert_eq!(list.get(1).unwrap(), 'b');
        assert_eq!(list.get(2).unwrap(), 'c');
    }

    #[test]
    fn test_get_invalid() {
        let mut list = List::new();

        // Get from empty list
        assert!(list.get(0).is_err());

        list.append('a');
        list.append('b');

        // Get at invalid indices
        assert!(list.get(2).is_err());
        assert!(list.get(10).is_err());
    }

    #[test]
    fn test_reverse() {
        // Test with empty list
        let mut list = List::new();
        list.reverse();
        assert_eq!(list.length(), 0);

        // Test with single element
        list.append('a');
        list.reverse();
        assert_eq!(list.length(), 1);
        assert_eq!(list.get(0).unwrap(), 'a');

        // Test with multiple elements
        list.append('b');
        list.append('c');
        list.append('d');
        list.reverse();

        assert_eq!(list.get(0).unwrap(), 'd');
        assert_eq!(list.get(1).unwrap(), 'c');
        assert_eq!(list.get(2).unwrap(), 'b');
        assert_eq!(list.get(3).unwrap(), 'a');
    }

    #[test]
    fn test_find_first() {
        let mut list = List::new();

        // Find in empty list
        assert_eq!(list.find_first('a'), -1);

        list.append('a');
        list.append('b');
        list.append('a');
        list.append('c');
        list.append('a');

        // Find existing elements
        assert_eq!(list.find_first('a'), 0);
        assert_eq!(list.find_first('b'), 1);
        assert_eq!(list.find_first('c'), 3);

        // Find non-existent element
        assert_eq!(list.find_first('x'), -1);
    }

    #[test]
    fn test_find_last() {
        let mut list = List::new();

        // Find in empty list
        assert_eq!(list.find_last('a'), -1);

        list.append('a');
        list.append('b');
        list.append('a');
        list.append('c');
        list.append('a');

        // Find existing elements
        assert_eq!(list.find_last('a'), 4);
        assert_eq!(list.find_last('b'), 1);
        assert_eq!(list.find_last('c'), 3);

        // Find non-existent element
        assert_eq!(list.find_last('x'), -1);
    }

    #[test]
    fn test_clear() {
        let mut list = List::new();

        // Clear empty list
        list.clear();
        assert_eq!(list.length(), 0);

        // Clear non-empty list
        list.append('a');
        list.append('b');
        list.append('c');
        assert_eq!(list.length(), 3);

        list.clear();
        assert_eq!(list.length(), 0);

        // Verify we can still use the list after clearing
        list.append('x');
        assert_eq!(list.length(), 1);
        assert_eq!(list.get(0).unwrap(), 'x');
    }

    #[test]
    fn test_extend() {
        let mut list1 = List::new();
        let mut list2 = List::new();

        // Extend empty list with empty list
        list1.extend(&list2);
        assert_eq!(list1.length(), 0);

        // Extend empty list with non-empty list
        list2.append('a');
        list2.append('b');
        list1.extend(&list2);
        assert_eq!(list1.length(), 2);
        assert_eq!(list1.get(0).unwrap(), 'a');
        assert_eq!(list1.get(1).unwrap(), 'b');

        // Extend non-empty list with non-empty list
        let mut list3 = List::new();
        list3.append('c');
        list3.append('d');
        list1.extend(&list3);
        assert_eq!(list1.length(), 4);
        assert_eq!(list1.get(2).unwrap(), 'c');
        assert_eq!(list1.get(3).unwrap(), 'd');

        // Verify original lists are unchanged
        assert_eq!(list2.length(), 2);
        assert_eq!(list3.length(), 2);
    }

    #[test]
    fn test_clone() {
        let mut original = List::new();

        // Clone empty list
        let empty_clone = original.clone();
        assert_eq!(empty_clone.length(), 0);

        // Clone non-empty list
        original.append('a');
        original.append('b');
        original.append('c');

        let clone = original.clone();
        assert_eq!(clone.length(), 3);
        assert_eq!(clone.get(0).unwrap(), 'a');
        assert_eq!(clone.get(1).unwrap(), 'b');
        assert_eq!(clone.get(2).unwrap(), 'c');

        // Verify independence - modifying original doesn't affect clone
        original.append('d');
        assert_eq!(original.length(), 4);
        assert_eq!(clone.length(), 3);
    }
}
