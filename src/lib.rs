use std::ops::Range;

mod ipq;
mod run;

trait IndexedPriorityQueue<T> {}

struct MinIndexedPriorityQueue<T> {
    vals: Vec<Option<T>>,
    // position_map: Vec<usize>,
    // inverse_map: Vec<usize>,
}

impl<T> IndexedPriorityQueue<T> for MinIndexedPriorityQueue<T> where T: Clone + PartialOrd {}

impl<T> MinIndexedPriorityQueue<T>
where
    T: Clone + PartialOrd,
{
    fn new_min_ipq(values: &Vec<T>) -> Self {
        let mut ipq = Self {
            vals: run::set_vals(values),
            // position_map:
            // inverse_map:
        };
        let next_node_index = run::last_some(&ipq.vals) + 1;
        let edge_layer_index_range = Range {
            start: (ipq.vals.len() / 2) - 1,
            end: next_node_index,
        };

        edge_layer_index_range
            .for_each(|edge_index| ipq.fix_min_ipq_branch_heap_invariant(edge_index));

        ipq
    }

    fn fix_min_ipq_branch_heap_invariant(&mut self, edge_node_index: usize) {
        let mut ni = edge_node_index;
        let mut pni = run::parent_node_index(edge_node_index);

        // minimum heap invariance breach
        let mib = |a: &Option<T>, b: &Option<T>| run::max_priority(a, b);

        Range {
            start: 0,
            end: run::number_of_layers(&self.vals),
        }
        .for_each(|_| {
            while mib(&self.vals[pni], &self.vals[ni]) && ni != pni {
                self.vals.swap(pni, ni);
                ni = pni;
                pni = run::parent_node_index(ni);
            }

            ni = edge_node_index;
            pni = run::parent_node_index(edge_node_index);
        });
    }

    fn check_potential_min_heap_invariance_integrity_breach(
        &self,
        value_to_insert: Option<&T>,
    ) -> bool {
        let next_value_index = run::last_some(&self.vals) + 1;
        let pni = run::parent_node_index(next_value_index);

        !run::max_priority(&Some(value_to_insert.unwrap().clone()), &self.vals[pni])
    }

    fn insert(&mut self, value: T) {
        let does_it_break_heap_invariance =
            self.check_potential_min_heap_invariance_integrity_breach(Some(&value));

        let nvi = run::last_some(&self.vals) + 1;
        self.vals[nvi] = Some(value);

        if does_it_break_heap_invariance {
            self.fix_min_ipq_branch_heap_invariant(nvi);
        }
    }
}

#[cfg(test)]
mod min_indexed_pq_tests {
    use crate::MinIndexedPriorityQueue;
    use crate::run::{left_child, right_child, set_vals};

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

    #[test]
    fn left_and_right_childs_should_return_option_even_on_last_layer() {
        let values = vec![9, 8, 8, 6, 1, 7, 2, 2, 2, 3, 4, 0];

        let ipq = MinIndexedPriorityQueue {
            vals: set_vals(&values),
        };

        assert_eq!(left_child(&ipq.vals, 4), Some(&3));
        assert_eq!(right_child(&ipq.vals, 4), Some(&4));

        assert_eq!(left_child(&ipq.vals, 5), Some(&0));
        assert_eq!(right_child(&ipq.vals, 5), None);

        assert_eq!(left_child(&ipq.vals, 7), None);
        assert_eq!(right_child(&ipq.vals, 7), None);

        assert_eq!(left_child(&ipq.vals, 12), None);
        assert_eq!(right_child(&ipq.vals, 12), None);
    }
}

struct MaxIndexedPriorityQueue<T> {
    vals: Vec<Option<T>>,
    // position_map: Vec<usize>,
    // inverse_map: Vec<usize>,
}

impl<T> IndexedPriorityQueue<T> for MaxIndexedPriorityQueue<T> where T: Clone + PartialOrd {}

impl<T> MaxIndexedPriorityQueue<T>
where
    T: Clone + PartialOrd,
{
    fn new_max_ipq(values: &Vec<T>) -> Self {
        let mut ipq = Self {
            vals: run::set_vals(values),
            // position_map:
            // inverse_map:
        };
        let next_node_index = run::last_some(&ipq.vals) + 1;
        let edge_layer_index_range = Range {
            start: (ipq.vals.len() / 2) - 1,
            end: next_node_index,
        };

        edge_layer_index_range
            .for_each(|edge_index| ipq.fix_max_ipq_branch_heap_invariant(edge_index));

        ipq
    }

    fn fix_max_ipq_branch_heap_invariant(&mut self, edge_node_index: usize) {
        let mut ni = edge_node_index;
        let mut pni = run::parent_node_index(edge_node_index);

        // maximum heap invariance breach
        let mib = |a: &Option<T>, b: &Option<T>| !run::max_priority(a, b);

        Range {
            start: 0,
            end: run::number_of_layers(&self.vals),
        }
        .for_each(|_| {
            while mib(&self.vals[pni], &self.vals[ni]) && ni != pni {
                self.vals.swap(pni, ni);
                ni = pni;
                pni = run::parent_node_index(ni);
            }

            ni = edge_node_index;
            pni = run::parent_node_index(edge_node_index);
        });
    }

    fn check_potential_push_max_heap_invariance_integrity_breach(
        &self,
        value_to_insert: &Option<T>,
    ) -> bool {
        let next_value_index = run::last_some(&self.vals) + 1;
        let pni = run::parent_node_index(next_value_index);

        run::max_priority(&value_to_insert, &self.vals[pni])
    }
}

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
