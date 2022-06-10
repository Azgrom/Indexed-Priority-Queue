use indexed_priority_queue::ipq::{IndexedBinaryHeap, IndexedPriorityQueue};
use indexed_priority_queue::MinIndexedPriorityQueue;

#[test]
fn test_insertion_and_polling_until_emptiness() {
    let mut values = vec![1, 2, 2, 2, 0];
    let mut ipq = MinIndexedPriorityQueue::from(&mut values);

    ipq.insert(5, 4);

    assert_eq!(ipq.poll_min_value(), 0);
    assert_eq!(ipq.poll_min_value(), 1);
    assert_eq!(ipq.poll_min_value(), 2);
    assert_eq!(ipq.poll_min_value(), 2);
    assert_eq!(ipq.poll_min_value(), 2);
    assert_eq!(ipq.poll_min_value(), 4);
    assert!(ipq.is_empty());
}

#[test]
fn test_decrease_and_polling_until_emptiness() {
    let mut values = vec![1, 2, 2, 2, 0];
    let mut ipq = MinIndexedPriorityQueue::from(&mut values);

    ipq.decrease(0, -1);
    ipq.decrease(1, -2);
    ipq.decrease(2, -3);
    ipq.decrease(3, -4);

    assert_eq!(ipq.poll_min_value(), -4);
    assert_eq!(ipq.poll_min_value(), -3);
    assert_eq!(ipq.poll_min_value(), -2);
    assert_eq!(ipq.poll_min_value(), -1);
    assert_eq!(ipq.poll_min_value(), 0);
    assert!(ipq.is_empty());
}
