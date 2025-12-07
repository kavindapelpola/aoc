use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

const DEFAULT_CAPACITY: usize = 500_000;

// collection of items that can be searched on O(1) time using a hashmap
// and have items added and removed  at the start and end in O(1) time, and randomly in O(n) time
#[derive(Debug, Clone)]
pub struct SearchableOrderedList<T> {
    items: VecDeque<T>,
    keys: HashSet<T>,
}

impl<T: Hash + Eq + PartialEq + Clone> SearchableOrderedList<T> {
    pub fn new() -> Self {
        SearchableOrderedList {
            items: VecDeque::with_capacity(DEFAULT_CAPACITY),
            keys: HashSet::with_capacity(DEFAULT_CAPACITY),
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match self.items.pop_front() {
            Some(item) => {
                self.keys.remove(&item);
                Some(item)
            }
            None => None,
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match self.items.pop_back() {
            Some(item) => {
                self.keys.remove(&item);
                Some(item)
            }
            None => None,
        }
    }

    pub fn push_front(&mut self, item: T) {
        self.keys.insert(item.clone());
        self.items.push_front(item);
    }

    pub fn push_back(&mut self, item: T) {
        self.keys.insert(item.clone());
        self.items.push_back(item);
    }

    pub fn find_index(&self, item: &T) -> Option<usize> {
        self.items
            .iter()
            .enumerate()
            .find(|(_, i)| *i == item)
            .map(|(i, _)| i)
    }

    pub fn get_by_index(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    pub fn get_by_item(&self, item: &T) -> Option<&T> {
        match self.find_index(&item) {
            Some(index) => self.get_by_index(index),
            None => None,
        }
    }

    pub fn update_by_index(&mut self, index: usize, item: T) {
        self.items[index] = item;
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn items(&self) -> Vec<T> {
        self.items.iter().map(|i| i.clone()).collect()
    }

    // Contains returns true if the list contains the given item, searches using the hashmap which is O(1) vs O(n) for the vector
    pub fn contains(&self, item: &T) -> bool {
        self.keys.contains(item)
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.keys.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_incements_len() {
        let mut list = SearchableOrderedList::<i32>::new();
        assert_eq!(list.len(), 0);
        list.push_back(122);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn contains_returns_true_when_item_exists() {
        let mut list = SearchableOrderedList::<i32>::new();
        list.push_back(122);
        assert!(list.contains(&122));
    }

    #[test]
    fn contains_returns_false_when_item_doesnt_exists() {
        let mut list = SearchableOrderedList::<i32>::new();
        list.push_back(122);
        assert!(!list.contains(&123));
    }

    #[test]
    fn contains_returns_false_when_item_added_is_removed() {
        let mut list = SearchableOrderedList::<i32>::new();
        list.push_back(122);
        assert!(list.contains(&122));
        list.pop_back();
        assert!(!list.contains(&122));
    }

    #[test]
    fn find_returns_index_of_item_found() {
        let mut list = SearchableOrderedList::<i32>::new();
        list.push_back(122);
        list.push_back(123);
        assert_eq!(list.find_index(&122), Some(0));
        assert_eq!(list.find_index(&123), Some(1));
    }

    #[test]
    fn find_returns_none_if_item_node_found() {
        let mut list = SearchableOrderedList::<i32>::new();
        list.push_back(122);
        assert_eq!(list.find_index(&123), None);
    }
}
