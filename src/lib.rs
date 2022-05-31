use std::ops::Range;
use ipq::IndexedPriorityQueue;

mod ipq;

type MinIndexedPriorityQueue<T> = IndexedPriorityQueue<T>;

#[cfg(test)]
mod min_indexed_pq_tests {
    use crate::MinIndexedPriorityQueue;

    #[test]
    fn check_min_heap_invariance_integrity_breach() {
        let values = vec![9, 8, 7, 6, 5, 1, 2, 2, 2, 3, 4, 0];
        let min_ipq = MinIndexedPriorityQueue::new_min_ipq(&values);

        let value_to_insert = Some(&8);
        let breached_heap_invariance =
            min_ipq.check_potential_min_heap_invariance_integrity_breach(value_to_insert);

        assert!(!breached_heap_invariance);
    }

    #[test]
    fn min_indexed_pq_should_sort_binary_heap_correctly() {
        let values = vec![9, 8, 7, 6, 5, 1, 2, 2, 2, 3, 4, 0];
        let min_ipq = MinIndexedPriorityQueue::new_min_ipq(&values);

        assert_eq!(
            min_ipq.vals,
            [
                Some(0),
                Some(2),
                Some(1),
                Some(6),
                Some(3),
                Some(2),
                Some(2),
                Some(9),
                Some(8),
                Some(5),
                Some(4),
                Some(7),
                None,
                None,
                None,
                None
            ]
        );
    }

    #[test]
    fn min_ipq_should_reorder_heap_prioritizing_minimum_value() {
        let values = vec![9, 8, 7, 6, 5, 1, 2, 2, 2, 3, 4, 0];
        let mut min_ipq = MinIndexedPriorityQueue::new_min_ipq(&values);
        let to_insert_value = -1;
        min_ipq.insert(to_insert_value);

        assert_eq!(
            min_ipq.vals,
            [
                Some(-1),
                Some(2),
                Some(0),
                Some(6),
                Some(3),
                Some(1),
                Some(2),
                Some(9),
                Some(8),
                Some(5),
                Some(4),
                Some(7),
                Some(2),
                None,
                None,
                None
            ]
        );
    }
}

type MaxIndexedPriorityQueue<T> = IndexedPriorityQueue<T>;

#[cfg(test)]
mod max_indexed_pq_tests {
    use crate::MaxIndexedPriorityQueue;

    #[test]
    fn check_max_heap_invariance_integrity_breach() {
        let values = vec![9, 8, 7, 6, 5, 1, 2, 2, 2, 3, 4, 0];
        let ipq = MaxIndexedPriorityQueue::new_max_ipq(&values);

        let value_to_insert = Some(8);
        let breached_heap_invariance =
            ipq.check_potential_push_max_heap_invariance_integrity_breach(&value_to_insert);

        assert!(breached_heap_invariance);
    }

    #[test]
    fn test() {
        let values = vec![0, 2, 1, 6, 3, 2, 2, 9, 8, 5, 4, 7];
        let ipq = MaxIndexedPriorityQueue::new_max_ipq(&values);

        assert_eq!(
            ipq.vals,
            [
                Some(9),
                Some(8),
                Some(7),
                Some(6),
                Some(5),
                Some(2),
                Some(2),
                Some(0),
                Some(2),
                Some(3),
                Some(4),
                Some(1),
                None,
                None,
                None,
                None
            ]
        );
    }
}
