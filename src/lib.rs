use crate::run::{left_child, parent_node_index, set_inverse_map};
use std::ops::Range;

mod ipq;
mod run;

trait PriorityQueue<T> {}

struct IndexedPriorityQueue<'a, T> {
    values: &'a mut Vec<T>,
    position_map: Vec<Option<usize>>,
    inverse_map: Vec<Option<usize>>,
}

impl<'a, T> IndexedPriorityQueue<'a, T>
where
    T: Clone + PartialOrd,
{
    fn position(&self, i: usize) -> usize {
        self.position_map[i].unwrap()
    }

    #[inline]
    fn inverse(&self, i: usize) -> usize {
        self.inverse_map[i].unwrap()
    }

    #[inline]
    fn value(&self, i: usize) -> &T {
        &self.values[self.inverse(i)]
    }

    #[inline]
    fn less(&self, i: usize, j: usize) -> bool {
        self.value(i) < self.value(j)
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.position_map[self.inverse_map[j].unwrap()] = Some(i);
        self.position_map[self.inverse_map[i].unwrap()] = Some(j);
        self.inverse_map.swap(i, j);
    }

    fn swim(&mut self, mut i: usize) {
        let parent = |x| parent_node_index(x);
        let mut pi = parent(i);
        while self.less(i, pi) {
            self.swap(i, pi);
            i = pi;
            pi = parent(i);
        }
    }

    fn min_child(&self, mut i: usize) -> Option<usize> {
        let from = 2 * i + 1;
        let number_of_direct_childs_per_node = 2;
        let to = from + number_of_direct_childs_per_node;
        let mut index: Option<usize> = None;

        return if let true = self.inverse_map.len() < to {
            index
        } else {
            let mut j = from;

            while j < to {
                if self.less(j, i) {
                    i = j;
                    index = Some(i);
                }
                j += 1;
            }

            index
        };
    }

    fn sink(&mut self, mut i: usize) {
        let mut j = self.min_child(i);

        while j.is_some() {
            self.swap(i, j.unwrap());
            i = j.unwrap();
            j = self.min_child(i);
        }
    }

    fn increase(&mut self, key_index: usize, value: T) {
        //TODO: Add if exists and is_some check
        if self.values[key_index] < value {
            self.values[key_index] = value;
            self.sink(self.position(key_index));
        }
    }

    fn decrease(&mut self, key_index: usize, value: T) {
        //TODO: Add if exists and is_some check
        if value < self.values[key_index] {
            self.values[key_index] = value;
            self.swim(self.position(key_index))
        }
    }

    fn update(&mut self, key_index: usize, value: T) -> T {
        //TODO: Add if exists and is_some check
        let i = self.position(key_index);
        let old_value = self.values[key_index].clone();

        self.values[key_index] = value;
        self.sink(i);
        self.swim(i);

        old_value
    }

    fn delete(&mut self, key_index: usize) -> T {
        let i = self.position(key_index);
        let size = self.values.len() - 1;
        self.swap(i, size);
        self.sink(i);
        self.swim(i);

        let value = self.values[key_index].clone();
        self.values.remove(key_index);
        self.position_map[key_index] = None;
        self.inverse_map[size] = None;

        value
    }
}

type MinIndexedPriorityQueue<'a, T> = IndexedPriorityQueue<'a, T>;

impl<'a, T> PriorityQueue<T> for MinIndexedPriorityQueue<'a, T> where T: Clone + PartialOrd {}

impl<'a, T> MinIndexedPriorityQueue<'a, T>
where
    T: Clone + PartialOrd,
{
    fn new(values: &'a mut Vec<T>) -> Self {
        let inverse_map = set_inverse_map(values);
        let mut mipq = Self {
            values,
            inverse_map,
            position_map: vec![Some(0); 2],
        };

        let last_node_index = run::last_some_index(&mipq.inverse_map);
        let edge_layer_range = Range {
            start: (mipq.inverse_map.len() / 2) - 1,
            end: last_node_index - 1,
        };
        edge_layer_range.for_each(|edge_index| {
            // minimum heap invariance breach
            let mib = |ni: usize, pni: usize| {
                run::max_priority(&(mipq.values[ni]), &mipq.values[pni]) && ni != pni
            };
            let mut node_index = edge_index;
            let mut parent_node_index = run::parent_node_index(edge_index);

            let mut layer = run::number_of_layers(&mipq.inverse_map);
            while layer != 0 {
                layer -= 1;

                while mib(parent_node_index, node_index) {
                    mipq.inverse_map.swap(node_index, parent_node_index);
                }
            }
        });

        mipq
    }

    // fn new_min_ipq(values: &Vec<T>) -> Self {
    //     let mut ipq = Self {
    //         values: run::set_vals(values),
    //         // position_map:
    //         // inverse_map:
    //     };
    //     let next_node_index = run::last_some_index(&ipq.values) + 1;
    //     let edge_layer_index_range = Range {
    //         start: (ipq.values.len() / 2) - 1,
    //         end: next_node_index,
    //     };
    //
    //     edge_layer_index_range
    //         .for_each(|edge_index| ipq.fix_min_ipq_branch_heap_invariant(edge_index));
    //
    //     ipq
    // }

    // fn fix_min_ipq_branch_heap_invariant(&mut self, edge_node_index: usize) {
    //     let mut ni = edge_node_index;
    //     let mut pni = run::parent_node_index(edge_node_index);
    //
    //     // minimum heap invariance breach
    //     let mib = |a: &T, b: &T| run::max_priority(a, b);
    //
    //     Range {
    //         start: 0,
    //         end: run::number_of_layers(&self.inverse_map),
    //     }
    //     .for_each(|_| {
    //         while mib(&self.values[pni], &self.values[ni]) && ni != pni {
    //             self.values.swap(pni, ni);
    //             ni = pni;
    //             pni = run::parent_node_index(ni);
    //         }
    //
    //         ni = edge_node_index;
    //         pni = run::parent_node_index(edge_node_index);
    //     });
    // }

    fn check_potential_min_heap_invariance_integrity_breach(
        &self,
        value_to_insert: Option<&T>,
    ) -> bool {
        let next_value_index = run::last_some_index(&self.inverse_map) + 1;
        let pni = run::parent_node_index(next_value_index);

        !run::max_priority(&value_to_insert.unwrap().clone(), &self.values[pni])
    }

    fn insert(&mut self, value: T) {
        let does_it_break_heap_invariance =
            self.check_potential_min_heap_invariance_integrity_breach(Some(&value));

        let nvi = run::last_some_index(&self.inverse_map) + 1;
        // self.values.push(value);

        // if does_it_break_heap_invariance {
        //     self.fix_min_ipq_branch_heap_invariant(nvi);
        // }
    }
}

#[cfg(test)]
mod min_indexed_pq_tests {
    use crate::run::{left_child, right_child, set_vals};
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
            min_ipq.values,
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
            min_ipq.values,
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

    // #[test]
    // fn left_and_right_childs_should_return_option_even_on_last_layer() {
    //     let values = vec![9, 8, 8, 6, 1, 7, 2, 2, 2, 3, 4, 0];
    //
    //     let ipq = MinIndexedPriorityQueue {
    //         values: set_vals(&values),
    //     };
    //
    //     assert_eq!(left_child(&ipq.values, 4), Some(&3));
    //     assert_eq!(right_child(&ipq.values, 4), Some(&4));
    //
    //     assert_eq!(left_child(&ipq.values, 5), Some(&0));
    //     assert_eq!(right_child(&ipq.values, 5), None);
    //
    //     assert_eq!(left_child(&ipq.values, 7), None);
    //     assert_eq!(right_child(&ipq.values, 7), None);
    //
    //     assert_eq!(left_child(&ipq.values, 12), None);
    //     assert_eq!(right_child(&ipq.values, 12), None);
    // }
}

struct MaxIndexedPriorityQueue<T> {
    vals: Vec<Option<T>>,
    // position_map: Vec<usize>,
    // inverse_map: Vec<usize>,
}

impl<T> PriorityQueue<T> for MaxIndexedPriorityQueue<T> where T: Clone + PartialOrd {}

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
        let next_node_index = run::last_some_index(&ipq.vals) + 1;
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
        let next_value_index = run::last_some_index(&self.vals) + 1;
        let pni = run::parent_node_index(next_value_index);

        run::max_priority(&value_to_insert, &&self.vals[pni])
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
