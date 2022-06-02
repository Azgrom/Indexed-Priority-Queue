use std::ops::Range;
use crate::{MaxIndexedPriorityQueue, MinIndexedPriorityQueue};




// impl<T> IndexedPriorityQueue2<T>
// where
//     T: Clone + PartialOrd,
// {
//     fn set_vals(values: &Vec<T>) -> Vec<Option<T>> {
//         let mut vals = values
//             .iter()
//             .map(|x| Some(x.clone()))
//             .collect::<Vec<Option<T>>>();
//
//         let np_diff = values.len().next_power_of_two() - values.len();
//         vals.append(&mut vec![None; np_diff]);
//
//         vals
//     }
//
//     fn max_priority(value_to_insert: &Option<T>, inserted_parent_node_value: &Option<T>) -> bool {
//         Some(value_to_insert) >= Some(inserted_parent_node_value)
//     }
// }

// impl<T> IndexedPriorityQueue2<T>
// where
//     T: Clone + PartialOrd,
// {
//     fn next_value_index(&self) -> usize {
//         let nvi = self.vals.iter().rposition(|x| x.is_some()).unwrap();
//
//         return if nvi < self.vals.len() - 1 {
//             nvi + 1
//         } else {
//             nvi
//         };
//     }
//
//     fn left_child_of(&self, key_index: usize) -> Option<&T> {
//         let i = 2 * key_index + 1;
//         return if i < self.vals.len() {
//             self.vals[i].as_ref()
//         } else {
//             None
//         };
//     }
//
//     fn right_child_of(&self, key_index: usize) -> Option<&T> {
//         let i = 2 * key_index + 2;
//         return if i < self.vals.len() {
//             self.vals[i].as_ref()
//         } else {
//             None
//         };
//     }
//
//     fn number_of_layers(&self) -> usize {
//         let mut number_of_layers = 1;
//         let mut n = self.vals.len().next_power_of_two();
//
//         while n != 2 {
//             n /= 2;
//             number_of_layers += 1;
//         }
//
//         number_of_layers
//     }
// }

// #[cfg(test)]
// mod indexed_priority_queue_tests {
//     #[test]
//     fn set_vals_should_successfully_return_unordered_binary_heap_main_array() {
//         let values = vec![9, 8, 8, 6, 1, 7, 2, 2, 2, 3, 4, 0];
//         let vals: Vec<Option<usize>> = IndexedPriorityQueue2::set_vals(&values);
//
//         assert_eq!(
//             vals,
//             [
//                 Some(9),
//                 Some(8),
//                 Some(8),
//                 Some(6),
//                 Some(1),
//                 Some(7),
//                 Some(2),
//                 Some(2),
//                 Some(2),
//                 Some(3),
//                 Some(4),
//                 Some(0),
//                 None,
//                 None,
//                 None,
//                 None
//             ]
//         );
//     }
//
//     #[test]
//     fn left_and_right_childs_should_return_option_even_on_last_layer() {
//         let values = vec![9, 8, 8, 6, 1, 7, 2, 2, 2, 3, 4, 0];
//
//         let ipq = IndexedPriorityQueue2 {
//             vals: IndexedPriorityQueue2::set_vals(&values),
//         };
//
//         assert_eq!(ipq.left_child_of(4), Some(&3));
//         assert_eq!(ipq.right_child_of(4), Some(&4));
//
//         assert_eq!(ipq.left_child_of(5), Some(&0));
//         assert_eq!(ipq.right_child_of(5), None);
//
//         assert_eq!(ipq.left_child_of(7), None);
//         assert_eq!(ipq.right_child_of(7), None);
//
//         assert_eq!(ipq.left_child_of(12), None);
//         assert_eq!(ipq.right_child_of(12), None);
//     }
//
//     #[test]
//     fn parent_node_index_should_return_parent_index_or_panic_if_out_bounds() {
//         let values = vec![9, 8, 7, 6, 5, 1, 2, 2, 2, 3, 4, 0];
//         let vals: Vec<Option<usize>> = IndexedPriorityQueue2::set_vals(&values);
//
//         let pni = |node_index| parent_node_index(node_index);
//         assert_eq!(vals[pni(11)], Some(1));
//         assert_eq!(vals[pni(10)], Some(5));
//         assert_eq!(vals[pni(2)], Some(9));
//     }
// }

// impl<T> MinIndexedPriorityQueue<T>
// where
//     T: Clone + PartialOrd,
// {
//     fn new_min_ipq(values: &Vec<T>) -> Self {
//         let mut ipq = Self {
//             vals: Self::set_vals(values),
//             // position_map:
//             // inverse_map:
//         };
//         let node_index = ipq.next_value_index();
//         let edge_layer_index_range = Range {
//             start: (ipq.vals.len() / 2) - 1,
//             end: node_index,
//         };
//
//         edge_layer_index_range
//             .for_each(|edge_index| ipq.fix_min_ipq_branch_heap_invariant(edge_index));
//
//         ipq
//     }
//
//     fn fix_min_ipq_branch_heap_invariant(&mut self, edge_node_index: usize) {
//         let mut ni = edge_node_index;
//         let mut pni = crate::parent_node_index(edge_node_index);
//
//         // minimum heap invariance breach
//         let mib = |a: &Option<T>, b: &Option<T>| IndexedPriorityQueue2::max_priority(a, b);
//
//         Range {
//             start: 0,
//             end: self.number_of_layers(),
//         }
//         .for_each(|_| {
//             while mib(&self.vals[pni], &self.vals[ni]) && ni != pni {
//                 self.vals.swap(pni, ni);
//                 ni = pni;
//                 pni = crate::parent_node_index(ni);
//             }
//
//             ni = edge_node_index;
//             pni = crate::parent_node_index(edge_node_index);
//         });
//     }
//
//     fn check_potential_min_heap_invariance_integrity_breach(
//         &self,
//         value_to_insert: Option<&T>,
//     ) -> bool {
//         let next_value_index = self.next_value_index() + 1;
//         let pni = crate::parent_node_index(next_value_index);
//
//         !IndexedPriorityQueue2::max_priority(&Some(value_to_insert.unwrap().clone()), &self.vals[pni])
//     }
//
//     fn insert(&mut self, value: T) {
//         let does_it_break_heap_invariance = self.check_potential_min_heap_invariance_integrity_breach(Some(&value));
//
//         let nvi = self.next_value_index();
//         self.vals[nvi] = Some(value);
//
//         if does_it_break_heap_invariance {
//             self.fix_min_ipq_branch_heap_invariant(nvi);
//         }
//     }
// }

// impl<T> MaxIndexedPriorityQueue<T>
// where
//     T: Clone + PartialOrd,
// {
//     fn new_max_ipq(values: &Vec<T>) -> Self {
//         let mut ipq = Self {
//             vals: Self::set_vals(values),
//             // position_map:
//             // inverse_map:
//         };
//         let node_index = ipq.next_value_index();
//         let edge_layer_index_range = Range {
//             start: (ipq.vals.len() / 2) - 1,
//             end: node_index,
//         };
//
//         edge_layer_index_range
//             .for_each(|edge_index| ipq.fix_max_ipq_branch_heap_invariant(edge_index));
//
//         ipq
//     }
//
//     fn fix_max_ipq_branch_heap_invariant(&mut self, edge_node_index: usize) {
//         let mut ni = edge_node_index;
//         let mut pni = crate::parent_node_index(edge_node_index);
//
//         // maximum heap invariance breach
//         let mib = |a: &Option<T>, b: &Option<T>| !IndexedPriorityQueue2::max_priority(a, b);
//
//         Range {
//             start: 0,
//             end: self.number_of_layers(),
//         }
//         .for_each(|_| {
//             while mib(&self.vals[pni], &self.vals[ni]) && ni != pni {
//                 self.vals.swap(pni, ni);
//                 ni = pni;
//                 pni = crate::parent_node_index(ni);
//             }
//
//             ni = edge_node_index;
//             pni = crate::parent_node_index(edge_node_index);
//         });
//     }
//
//     fn check_potential_push_max_heap_invariance_integrity_breach(
//         &self,
//         value_to_insert: &Option<T>,
//     ) -> bool {
//         let next_value_index = self.next_value_index() + 1;
//         let pni = crate::parent_node_index(next_value_index);
//
//         IndexedPriorityQueue2::max_priority(&value_to_insert, &self.vals[pni])
//     }
// }
