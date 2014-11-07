use std::cmp::{Equal, Ordering};
use std::collections::BinaryHeap;

#[deriving(Clone)]
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

#[deriving(Clone)]
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

    pub fn top<'a>(&'a self) -> Option<&'a T> {
        match self.heap.top() {
            None => None,
            Some(item) => Some(&item.item)
        }
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

    pub fn from_vec(cmp : fn (&T, &T) -> Ordering,
                    xs : Vec<T>) -> ComparatorHeap<T> {
        let mut vs : Vec<ComparatorHeapItem<T>> = Vec::new();
        for item in xs.into_iter() {
            vs.push(ComparatorHeapItem {
                compare : cmp,
                item : item
            });
        };

        ComparatorHeap {
            compare : cmp,
            heap : BinaryHeap::from_vec(vs)
        }
    }

    /// consumes the ComparatorHeap and returns the underlying
    /// vector in arbitrary order.
    pub fn into_vec(self) -> Vec<T> {
        self.heap.into_vec().
                into_iter().map(|item : ComparatorHeapItem<T>| -> T {
            item.item
        }).collect()
    }

    /// consumes the ComparatorHeap and returns a vector in sorted
    /// (ascending) order
    pub fn into_sorted_vec(self) -> Vec<T> {
        self.heap.into_sorted_vec().
                into_iter().map(|item : ComparatorHeapItem<T>| -> T {
            item.item
        }).collect()
    }
}
