/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default + Eq + Clone,
{
    count: usize,
    items: Vec<T>,
    itnum: usize,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Eq + Clone,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            itnum: 0,
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.add_i(1, value.clone());
        if self.itnum == 0 || (self.comparator)(&value, &self.items[self.itnum]) {
            self.itnum = self.smallest_child_idx(1);
        }
        self.count += 1;
    }

    pub fn add_i(&mut self, idx: usize, value: T) {
        if idx >= self.items.len() {
            self.items.resize_with(idx + 1, T::default);
        }
        if self.items[idx] == T::default() {
            self.items[idx] = value;
        }
        else if (self.comparator)(&value, &self.items[idx]) {
            self.add_i(self.left_child_idx(idx), value);
        }
        else {
            self.add_i(self.right_child_idx(idx), value);
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let mut cur = idx;
        loop {
            let next = self.left_child_idx(cur);
            if !self.idx_hasvalue(next) {
                break;
            }
            cur = next;
        }
        cur
    }

    fn idx_hasvalue(&self, idx: usize) -> bool {
        idx < self.items.len() && self.items[idx] != T::default()
    }

    fn idx_next(&self, idx: usize, flag: bool) -> usize {
        if !flag && self.idx_hasvalue(self.right_child_idx(idx)) {
            self.smallest_child_idx(self.right_child_idx(idx))
        }
        else if self.left_child_idx(self.parent_idx(idx)) == idx {
            self.parent_idx(idx)
        }
        else {
            self.idx_next(idx, true)
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + Clone,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Eq + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.itnum == 0 { return None; }
		let ret = self.items[self.itnum].clone();
        self.itnum = self.idx_next(self.itnum, false);
        Some(ret)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}