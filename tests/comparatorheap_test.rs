extern crate comparatorheap;
use comparatorheap::ComparatorHeap;

struct UintWrap {
    val : uint,
    #[allow(dead_code)]
    unord : u8 /* ensure that this type is not ordered */
}

fn wrap(val : uint) -> UintWrap {
    UintWrap {
        val : val,
        unord : 0
    }
}

fn uintwrap_max(a : &UintWrap, b : &UintWrap) -> Ordering {
    a.val.cmp(&b.val)
}
fn uintwrap_min(a : &UintWrap, b : &UintWrap) -> Ordering {
    uintwrap_max(a, b).reverse()
}

/* call this on a UintWrap, will not compile if not ordered */
#[allow(dead_code)]
fn require_ord<T: Ord>(_ord : T) {}

#[test]
/// only need to test that items come out of the heap in proper order
/// since the heap functionality is delegated to the library binary heap
fn test_push() {
    let mut max_heap : ComparatorHeap<UintWrap> = ComparatorHeap::new(uintwrap_max);
    let mut min_heap : ComparatorHeap<UintWrap> = ComparatorHeap::new(uintwrap_min);

    max_heap.push(wrap(8)); min_heap.push(wrap(8));
    max_heap.push(wrap(4)); min_heap.push(wrap(4));
    max_heap.push(wrap(6)); min_heap.push(wrap(6));

    assert_eq!(max_heap.pop().unwrap().val, 8);
    assert_eq!(max_heap.pop().unwrap().val, 6);
    assert_eq!(max_heap.pop().unwrap().val, 4);
    assert!(max_heap.is_empty());

    assert_eq!(min_heap.pop().unwrap().val, 4);
    assert_eq!(min_heap.pop().unwrap().val, 6);
    assert_eq!(min_heap.pop().unwrap().val, 8);
}

fn cmp<T : Ord>(a : &T, b : &T) -> Ordering {
    a.cmp(b)
}
#[test]
fn test_clone() {
    /* uses max ordering, so this is a max heap */
    let mut heap : ComparatorHeap<uint> = ComparatorHeap::new(cmp);

    heap.push(5);
    heap.push(12);
    heap.push(3);
    heap.push(7);

    let mut copy = heap.clone();
    assert_eq!(heap.pop(), Some(12));
    assert_eq!(copy.pop(), Some(12));
    assert_eq!(heap.pop(), Some(7));
    assert_eq!(copy.pop(), Some(7));
}
