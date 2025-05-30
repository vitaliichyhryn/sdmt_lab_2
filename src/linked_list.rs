use std::fmt::Debug;

pub struct List {
    vec: Vec<char>,
}

impl List {
    /// Creates an empty list
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }

    /// Returns the length of the list
    pub fn length(&self) -> usize {
        self.vec.len()
    }

    /// Appends `elem` to the back of the list
    pub fn append(&mut self, elem: char) {
        self.vec.push(elem);
    }

    /// Inserts `elem` at position `index` of the list
    /// # Errors
    /// Returns an error if `index > len`
    pub fn insert(&mut self, elem: char, index: usize) -> Result<(), String> {
        if index > self.length() {
            return Err("Index out of bounds".to_string());
        }
        self.vec.insert(index, elem);
        Ok(())
    }

    /// Deletes element at position `index` of the list
    /// # Errors
    /// Returns an error if `index` is out of bounds
    pub fn delete(&mut self, index: usize) -> Result<char, String> {
        if index >= self.length() {
            return Err("Index out of bounds".to_string());
        }
        Ok(self.vec.remove(index))
    }

    /// Deletes all occurences of `elem` in the list
    pub fn delete_all(&mut self, elem: char) {
        self.vec.retain(|&x| x != elem);
    }

    /// Returns the element at position `index` of the list
    /// # Errors
    /// Returns an error if `index` is out of bounds
    pub fn get(&self, index: usize) -> Result<char, String> {
        self.vec
            .get(index)
            .copied()
            .ok_or("Index out of bounds".to_string())
    }

    /// Reverses the list
    pub fn reverse(&mut self) {
        self.vec.reverse();
    }

    /// Returns the position of the first occurrence of `elem` in the list
    ///
    /// If there were no occurrences of `elem`, returns `-1`
    pub fn find_first(&self, elem: char) -> i64 {
        match self.vec.iter().position(|&x| x == elem) {
            None => -1,
            Some(index) => index as i64,
        }
    }

    /// Returns the position of the last occurrence of `elem` in the list
    ///
    /// If there were no occurrences of `elem`, returns `-1`
    pub fn find_last(&self, elem: char) -> i64 {
        match self.vec.iter().rposition(|&x| x == elem) {
            None => -1,
            Some(index) => index as i64,
        }
    }

    /// Removes all elements from the list
    pub fn clear(&mut self) {
        self.vec.clear();
    }

    /// Copies all elements from `other` to the end of the list
    pub fn extend(&mut self, other: &List) {
        self.vec.extend(&other.vec);
    }
}

impl Clone for List {
    /// Returns a copy of the list
    fn clone(&self) -> Self {
        Self {
            vec: self.vec.clone(),
        }
    }
}

impl Debug for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.vec.iter()).finish()
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
