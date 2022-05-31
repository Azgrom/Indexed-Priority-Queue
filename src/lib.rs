use std::ops::Range;

struct IndexedPriorityQueue<T> {
    vals: Vec<Option<T>>,
    // position_map: Vec<usize>,
    // inverse_map: Vec<usize>,
}

fn parent_node_index(node_index: usize) -> usize {
    return match node_index {
        0 => 0,
        n if n % 2 == 0 => (n / 2) - 1,
        _ => (node_index - 1) / 2,
    };
}

impl<T> IndexedPriorityQueue<T>
where
    T: Clone + PartialOrd,
{
    fn set_vals(values: &Vec<T>) -> Vec<Option<T>> {
        let mut vals = values
            .iter()
            .map(|x| Some(x.clone()))
            .collect::<Vec<Option<T>>>();

        let np_diff = values.len().next_power_of_two() - values.len();
        vals.append(&mut vec![None; np_diff]);

        vals
    }

    fn max_priority(value_to_insert: &Option<T>, inserted_parent_node_value: &Option<T>) -> bool {
        Some(value_to_insert) >= Some(inserted_parent_node_value)
    }
}

impl<T> IndexedPriorityQueue<T>
where
    T: Clone + PartialOrd,
{
    fn next_value_index(&self) -> usize {
        let nvi = self.vals.iter().rposition(|x| x.is_some()).unwrap();

        return if nvi < self.vals.len() - 1 {
            nvi + 1
        } else {
            nvi
        };
    }

    fn left_child_of(&self, key_index: usize) -> Option<&T> {
        let i = 2 * key_index + 1;
        return if i < self.vals.len() {
            self.vals[i].as_ref()
        } else {
            None
        };
    }

    fn right_child_of(&self, key_index: usize) -> Option<&T> {
        let i = 2 * key_index + 2;
        return if i < self.vals.len() {
            self.vals[i].as_ref()
        } else {
            None
        };
    }

    fn number_of_layers(&self) -> usize {
        let mut number_of_layers = 1;
        let mut n = self.vals.len().next_power_of_two();

        while n != 2 {
            n /= 2;
            number_of_layers += 1;
        }

        number_of_layers
    }
}

#[cfg(test)]
mod indexed_priority_queue_tests {
    use crate::{parent_node_index, IndexedPriorityQueue};

    #[test]
    fn set_vals_should_successfully_return_unordered_binary_heap_main_array() {
        let values = vec![9, 8, 8, 6, 1, 7, 2, 2, 2, 3, 4, 0];
        let vals: Vec<Option<usize>> = IndexedPriorityQueue::set_vals(&values);

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
    fn left_and_right_childs_should_return_option_even_on_last_layer() {
        let values = vec![9, 8, 8, 6, 1, 7, 2, 2, 2, 3, 4, 0];

        let ipq = IndexedPriorityQueue {
            vals: IndexedPriorityQueue::set_vals(&values),
        };

        assert_eq!(ipq.left_child_of(4), Some(&3));
        assert_eq!(ipq.right_child_of(4), Some(&4));

        assert_eq!(ipq.left_child_of(5), Some(&0));
        assert_eq!(ipq.right_child_of(5), None);

        assert_eq!(ipq.left_child_of(7), None);
        assert_eq!(ipq.right_child_of(7), None);

        assert_eq!(ipq.left_child_of(12), None);
        assert_eq!(ipq.right_child_of(12), None);
    }

    #[test]
    fn parent_node_index_should_return_parent_index_or_panic_if_out_bounds() {
        let values = vec![9, 8, 7, 6, 5, 1, 2, 2, 2, 3, 4, 0];
        let vals: Vec<Option<usize>> = IndexedPriorityQueue::set_vals(&values);

        let pni = |node_index| parent_node_index(node_index);
        assert_eq!(vals[pni(11)], Some(1));
        assert_eq!(vals[pni(10)], Some(5));
        assert_eq!(vals[pni(2)], Some(9));
    }
}

type MinIndexedPriorityQueue<T> = IndexedPriorityQueue<T>;

impl<T> MinIndexedPriorityQueue<T>
where
    T: Clone + PartialOrd,
{
    fn new(values: &Vec<T>) -> Self {
        let mut min_priority_queue = Self {
            vals: Self::set_vals(values),
            // position_map:
            // inverse_map:
        };
        let node_index = min_priority_queue.next_value_index();
        let edge_layer_index_range = Range {
            start: (min_priority_queue.vals.len() / 2) - 1,
            end: node_index,
        };

        edge_layer_index_range.for_each(|i| min_priority_queue.fix_branch_heap_invariant(i));
        // min_priority_queue.fix_branch_heap_invariant(node_index);

        min_priority_queue
    }

    fn fix_branch_heap_invariant(&mut self, edge_node_index: usize) {
        let mut ni = edge_node_index;
        let mut pni = parent_node_index(edge_node_index);

        // minimum heap invariance breach
        let mut mib = |a: &Option<T>, b: &Option<T>| IndexedPriorityQueue::max_priority(a, b);

        Range {
            start: 0,
            end: self.number_of_layers(),
        }
        .for_each(|_| {
            while mib(&self.vals[pni], &self.vals[ni]) && ni != pni {
                self.vals.swap(pni, ni);
                ni = pni;
                pni = parent_node_index(ni);
            }

            ni = edge_node_index;
            pni = parent_node_index(edge_node_index);
        });
    }

    fn check_potential_min_heap_invariance_integrity_breach(
        &self,
        value_to_insert: Option<T>,
    ) -> bool {
        let next_value_index = self.next_value_index() + 1;
        let pni = parent_node_index(next_value_index);

        !IndexedPriorityQueue::max_priority(&value_to_insert, &self.vals[pni])
    }

    fn check_branch_min_heap_invariance_breach(&mut self, mut node_index: usize) {
        let mut pni = parent_node_index(node_index);
        let mut invariance_breach = false;
        while pni != 0 && !invariance_breach {
            // self.fix_heap_invariant(node_index, pni);
            invariance_breach =
                IndexedPriorityQueue::max_priority(&self.vals[node_index], &self.vals[pni]);
            node_index = pni;
            pni = parent_node_index(node_index);
        }
    }
}

#[cfg(test)]
mod min_indexed_pq_tests {
    use crate::{parent_node_index, MinIndexedPriorityQueue};

    #[test]
    fn check_min_heap_invariance_integrity_breach() {
        let values = vec![9, 8, 7, 6, 5, 1, 2, 2, 2, 3, 4, 0];
        let mut min_ipq = MinIndexedPriorityQueue::new(&values);

        let value_to_insert = Some(8);
        let breached_heap_invariance =
            min_ipq.check_potential_min_heap_invariance_integrity_breach(value_to_insert);

        assert!(!breached_heap_invariance);
    }

    #[test]
    fn test() {
        let values = vec![9, 8, 7, 6, 5, 1, 2, 2, 2, 3, 4, 0];
        let mut min_ipq = MinIndexedPriorityQueue::new(&values);

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
}

type MaxIndexedPriorityQueue<T> = IndexedPriorityQueue<T>;

impl<T> MaxIndexedPriorityQueue<T>
where
    T: Clone + PartialOrd,
{
    fn check_potential_push_max_heap_invariance_integrity_breach(
        &self,
        value_to_insert: &Option<T>,
    ) -> bool {
        let next_value_index = self.next_value_index() + 1;
        let pni = parent_node_index(next_value_index);

        IndexedPriorityQueue::max_priority(&value_to_insert, &self.vals[pni])
    }
}

#[cfg(test)]
mod max_indexed_pq_tests {
    use crate::MaxIndexedPriorityQueue;

    #[test]
    fn check_max_heap_invariance_integrity_breach() {
        let values = vec![9, 8, 7, 6, 5, 1, 2, 2, 2, 3, 4, 0];
        let mut ipq = MaxIndexedPriorityQueue::new(&values);

        let value_to_insert = Some(8);
        let breached_heap_invariance =
            ipq.check_potential_push_max_heap_invariance_integrity_breach(&value_to_insert);

        assert!(breached_heap_invariance);
    }
}
