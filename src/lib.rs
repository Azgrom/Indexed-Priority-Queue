use std::ops::Range;

mod ipq;

fn max_priority<T>(value_to_insert: &Option<T>, inserted_parent_node_value: &Option<T>) -> bool
where
    T: PartialOrd,
{
    Some(value_to_insert) >= Some(inserted_parent_node_value)
}

fn last_some<T>(vals: &Vec<Option<T>>) -> usize {
    vals.iter().rposition(|x| x.is_some()).unwrap()
}

fn number_of_layers<T>(vals: &Vec<Option<T>>) -> usize {
    let mut number_of_layers = 1;
    let mut n = vals.len().next_power_of_two();

    while n != 2 {
        n /= 2;
        number_of_layers += 1;
    }

    number_of_layers
}

fn parent_node_index(node_index: usize) -> usize {
    return match node_index {
        0 => 0,
        n if n % 2 == 0 => (n / 2) - 1,
        _ => (node_index - 1) / 2,
    };
}

fn left_child<T>(vals: &Vec<Option<T>>, node_index: usize) -> Option<&T> {
    let i = 2 * node_index + 1;
    return if i < vals.len() {
        vals[i].as_ref()
    } else {
        None
    };
}

fn right_child<T>(vals: &Vec<Option<T>>, node_index: usize) -> Option<&T> {
    let i = 2 * node_index + 2;
    return if i < vals.len() {
        vals[i].as_ref()
    } else {
        None
    };
}

fn set_vals<T>(values: &Vec<T>) -> Vec<Option<T>>
where
    T: Clone,
{
    let mut vals = values
        .iter()
        .map(|x| Some(x.clone()))
        .collect::<Vec<Option<T>>>();

    let np_diff = values.len().next_power_of_two() - values.len();
    vals.append(&mut vec![None; np_diff]);

    vals
}

#[cfg(test)]
mod indexed_priority_queue_tests {
    use crate::{parent_node_index, set_vals};

    #[test]
    fn set_vals_should_successfully_return_unordered_binary_heap_main_array() {
        let values = vec![9, 8, 8, 6, 1, 7, 2, 2, 2, 3, 4, 0];
        let vals: Vec<Option<usize>> = set_vals(&values);

        assert_eq!(
            vals,
            [
                Some(9),
                Some(8),
                Some(8),
                Some(6),
                Some(1),
                Some(7),
                Some(2),
                Some(2),
                Some(2),
                Some(3),
                Some(4),
                Some(0),
                None,
                None,
                None,
                None
            ]
        );
    }

    #[test]
    fn parent_node_index_should_return_parent_index_or_panic_if_out_bounds() {
        let values = vec![9, 8, 7, 6, 5, 1, 2, 2, 2, 3, 4, 0];
        let vals: Vec<Option<usize>> = set_vals(&values);

        let pni = |node_index| parent_node_index(node_index);
        assert_eq!(vals[pni(11)], Some(1));
        assert_eq!(vals[pni(10)], Some(5));
        assert_eq!(vals[pni(2)], Some(9));
    }
}

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
            vals: set_vals(values),
            // position_map:
            // inverse_map:
        };
        let next_node_index = last_some(&ipq.vals) + 1;
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
        let mut pni = crate::parent_node_index(edge_node_index);

        // minimum heap invariance breach
        let mib = |a: &Option<T>, b: &Option<T>| max_priority(a, b);

        Range {
            start: 0,
            end: number_of_layers(&self.vals),
        }
        .for_each(|_| {
            while mib(&self.vals[pni], &self.vals[ni]) && ni != pni {
                self.vals.swap(pni, ni);
                ni = pni;
                pni = crate::parent_node_index(ni);
            }

            ni = edge_node_index;
            pni = crate::parent_node_index(edge_node_index);
        });
    }

    fn check_potential_min_heap_invariance_integrity_breach(
        &self,
        value_to_insert: Option<&T>,
    ) -> bool {
        let next_value_index = last_some(&self.vals) + 1;
        let pni = crate::parent_node_index(next_value_index);

        !max_priority(&Some(value_to_insert.unwrap().clone()), &self.vals[pni])
    }

    fn insert(&mut self, value: T) {
        let does_it_break_heap_invariance =
            self.check_potential_min_heap_invariance_integrity_breach(Some(&value));

        let nvi = last_some(&self.vals) + 1;
        self.vals[nvi] = Some(value);

        if does_it_break_heap_invariance {
            self.fix_min_ipq_branch_heap_invariant(nvi);
        }
    }
}

#[cfg(test)]
mod min_indexed_pq_tests {
    use crate::{left_child, right_child, set_vals, MinIndexedPriorityQueue};

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
            vals: set_vals(values),
            // position_map:
            // inverse_map:
        };
        let next_node_index = last_some(&ipq.vals) + 1;
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
        let mut pni = crate::parent_node_index(edge_node_index);

        // maximum heap invariance breach
        let mib = |a: &Option<T>, b: &Option<T>| !max_priority(a, b);

        Range {
            start: 0,
            end: number_of_layers(&self.vals),
        }
        .for_each(|_| {
            while mib(&self.vals[pni], &self.vals[ni]) && ni != pni {
                self.vals.swap(pni, ni);
                ni = pni;
                pni = crate::parent_node_index(ni);
            }

            ni = edge_node_index;
            pni = crate::parent_node_index(edge_node_index);
        });
    }

    fn check_potential_push_max_heap_invariance_integrity_breach(
        &self,
        value_to_insert: &Option<T>,
    ) -> bool {
        let next_value_index = last_some(&self.vals) + 1;
        let pni = crate::parent_node_index(next_value_index);

        max_priority(&value_to_insert, &self.vals[pni])
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
