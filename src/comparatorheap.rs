use std::cmp::{Equal, Ordering};
use std::collections::BinaryHeap;

struct ComparatorHeapItem<T> {
    compare : fn (&T, &T) -> Ordering,
    item : T
}

impl<T> Eq for ComparatorHeapItem<T> {}
impl<T> PartialEq for ComparatorHeapItem<T> {
    fn eq(&self, other : &ComparatorHeapItem<T>) -> bool {
        match self.cmp(other) {
            Equal => true,
            _ => false
        }
    }
}

impl<T> PartialOrd for ComparatorHeapItem<T> {
    fn partial_cmp(&self, other : &ComparatorHeapItem<T>) -> Option<Ordering> {
        Some((self.compare)(&self.item, &other.item))
    }
}
impl<T> Ord for ComparatorHeapItem<T> {
    fn cmp(&self, other : &ComparatorHeapItem<T>) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl<T : Clone> Clone for ComparatorHeapItem<T> {
    fn clone(&self) -> ComparatorHeapItem<T> {
        ComparatorHeapItem {
            compare : self.compare,
            item : self.item.clone()
        }
    }
}

pub struct ComparatorHeap<T> {
    compare : fn (&T, &T) -> Ordering,
    heap : BinaryHeap<ComparatorHeapItem<T>>
}

impl<T> ComparatorHeap<T> {
    pub fn new(cmp : fn (&T, &T) -> Ordering) -> ComparatorHeap<T> {
        ComparatorHeap {
            compare : cmp,
            heap : BinaryHeap::new()
        }
    }

    pub fn with_capacity(cmp : fn (&T, &T) -> Ordering, capacity : uint) -> ComparatorHeap<T> {
        ComparatorHeap {
            compare : cmp,
            heap : BinaryHeap::with_capacity(capacity)
        }
    }

    pub fn capacity(&self) -> uint {
        self.heap.capacity()
    }

    pub fn reserve_exact(&mut self, n : uint) {
        self.heap.reserve_exact(n)
    }

    pub fn reserve(&mut self, n : uint) {
        self.heap.reserve(n)
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.heap.pop() {
            None => None,
            Some(item) => Some(item.item)
        }
    }

    pub fn push(&mut self, item : T) {
        self.heap.push(ComparatorHeapItem {
            compare : self.compare,
            item : item
        })
    }

    /// pushes an item onto a queue and then pops the greatest item
    /// off in an optimized fashion
    pub fn push_pop(&mut self, item : T) -> T {
        self.heap.push_pop(ComparatorHeapItem {
            compare : self.compare,
            item : item
        }).item
    }

    /// pops the greatest item off a queue then pushes an item
    /// onto the queue in an optimized fashion
    pub fn replace(&mut self, item : T) -> Option<T> {
        match self.heap.replace(ComparatorHeapItem {
            compare : self.compare,
            item : item
        }) {
            None => None,
            Some(item) => Some(item.item)
        }
    }

    pub fn len(&self) -> uint {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn clear(&mut self) {
        self.heap.clear()
    }
}

impl<T : Clone> Clone for ComparatorHeap<T> {
    fn clone(&self) -> ComparatorHeap<T>{
        ComparatorHeap {
            compare : self.compare,
            heap : self.heap.clone()
        }
    }
}
