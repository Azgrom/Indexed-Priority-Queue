use crate::ipq::{IndexedBinaryHeap, IndexedPriorityQueue};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::Range;
use std::slice::{Iter, IterMut};

pub mod ipq;

fn parent_node_index(node_index: usize) -> usize {
    return match node_index {
        0 => 0,
        n if n % 2 == 0 => (n / 2) - 1,
        _ => (node_index - 1) / 2,
    };
}

fn max_value_index<T: Copy + Ord>(array: &Vec<T>) -> usize {
    array
        .iter()
        .enumerate()
        .max_by_key(|(_, &v)| v)
        .map(|(i, _)| i)
        .unwrap()
}

pub struct MinIndexedPriorityQueue<'a, T>
where
    T: Clone,
{
    values: &'a mut Vec<T>,
    position_map: Vec<Option<usize>>,
    inverse_map: Vec<Option<usize>>,
}

impl<'a, T> Display for MinIndexedPriorityQueue<'a, T>
    where
        T: Clone + PartialOrd,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Minimum Priority Queue of {} elements and {} branches",
            self.size(),
            self.branches_count()
        )
    }
}

impl<'a, T> PartialEq<Self> for MinIndexedPriorityQueue<'a, T>
    where
        T: Clone + PartialOrd,
{
    fn eq(&self, other: &Self) -> bool {
        self.peek_min_value().eq(&other.peek_min_value())
    }
}

impl<'a, T> PartialOrd for MinIndexedPriorityQueue<'a, T>
    where
        T: Clone + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.peek_min_value().partial_cmp(&other.peek_min_value())
    }
}

impl<'a, T> From<&'a mut Vec<T>> for MinIndexedPriorityQueue<'a, T>
    where
        T: Clone + PartialOrd,
{
    /// Initializes a minimum indexed priority queue from a mutably borrowed `values` vector.
    ///
    /// # Arguments
    ///
    /// * `values`: `&mut Vec<T>` where `T` implements `Clone` and `PartialOrd`
    ///
    /// returns: `MinIndexedPriorityQueue<T>`
    ///
    /// # Examples
    ///
    /// ```
    /// use std::any::type_name;
    /// use indexed_priority_queue::ipq::IndexedBinaryHeap;
    /// use indexed_priority_queue::MinIndexedPriorityQueue;
    ///
    /// let mut values: Vec<u8> = Vec::new();
    /// let mut min_ipq = MinIndexedPriorityQueue::from(&mut values);
    ///
    /// fn type_of<T>(_: T) -> &'static str {
    ///     type_name::<T>()
    /// }
    ///
    /// assert_eq!(min_ipq.is_empty(), true);
    /// assert_eq!(type_of(min_ipq), "indexed_priority_queue::MinIndexedPriorityQueue<u8>");
    /// ```
    fn from(values: &'a mut Vec<T>) -> Self {
        let npt = values.len().next_power_of_two();
        let mut values_map = vec![None; npt];
        Range {
            start: 0,
            end: values.len(),
        }
            .for_each(|i| values_map[i] = Some(i));

        let position_map = values_map.clone();
        let inverse_map = values_map;

        let mut min_ipq = Self {
            values,
            position_map,
            inverse_map,
        };
        min_ipq.fix_heap_invariant();

        min_ipq
    }
}

impl<'a, T> IndexedBinaryHeap for MinIndexedPriorityQueue<'a, T>
    where
        T: Clone + PartialOrd,
{
    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    #[inline]
    fn less(&self, i: usize, j: usize) -> bool {
        self.priority_sequenced_value(i) < self.priority_sequenced_value(j)
    }

    fn min_child(&self, mut i: usize) -> Option<usize> {
        let number_of_direct_childs_per_node = 2;
        let mut from = number_of_direct_childs_per_node * i + 1;
        let to = if let true = (from + number_of_direct_childs_per_node) > self.size() {
            self.size()
        } else {
            from + number_of_direct_childs_per_node
        };

        let mut index: Option<usize> = None;

        while from < to {
            if self.less(from, i) {
                i = from;
                index = Some(i);
            }
            from += 1;
        }

        index
    }

    fn size(&self) -> usize {
        self.values.len()
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.position_map[self.inverse_map[j].unwrap()] = Some(i);
        self.position_map[self.inverse_map[i].unwrap()] = Some(j);
        self.inverse_map.swap(i, j);
    }

    fn swim(&mut self, mut i: usize) {
        let mut pi = parent_node_index(i);
        while i != pi && self.less(i, pi) {
            self.swap(i, pi);
            i = pi;
            pi = parent_node_index(i);
        }
    }

    fn sink(&mut self, mut i: usize) {
        let mut j = self.min_child(i);

        while j.is_some() && j != Some(self.values.len()) {
            self.swap(i, j.unwrap());
            i = j.unwrap();
            j = self.min_child(i);
        }
    }
}

impl<'a, T> IndexedPriorityQueue<T> for MinIndexedPriorityQueue<'a, T>
where
    T: Clone + PartialOrd,
{
    fn append(&mut self, extra_values: &mut Vec<T>) {
        let size = self.size();
        let next_size = size + extra_values.len();

        if next_size > self.position_map.len() {
            self.expand_mapping();
        }

        self.values.append(extra_values);

        Range {
            start: size,
            end: self.size(),
        }
            .for_each(|i| {
                self.inverse_map[i] = Some(i);
                self.position_map[i] = Some(i);
            });

        self.fix_heap_invariant();
    }

    fn contains(&self, key_index: usize) -> bool {
        return if key_index > (self.size() - 1) {
            false
        } else {
            true
        };
    }

    fn decrease(&mut self, key_index: usize, value: T) {
        self.key_exists_or_panic(key_index);
        if value < self.values[key_index] {
            self.values[key_index] = value;

            self.swim(self.node_index(key_index))
        }
    }

    fn delete(&mut self, key_index: usize) -> Option<T> {
        let size = self.size() - 1;
        if size < key_index {
            return None;
        }

        let im_index_max = max_value_index(&self.inverse_map);
        let pm_index_max = max_value_index(&self.position_map);

        let i = self.node_index(key_index);

        self.inverse_map.swap(size, im_index_max);
        self.position_map.swap(size, pm_index_max);
        self.inverse_map[size] = None;
        self.position_map[size] = None;

        let value = Some(self.values[key_index].clone());
        self.values.remove(key_index);

        self.sink(i);
        self.swim(i);

        value
    }

    fn drain(&mut self, start: usize, end: usize) -> Vec<T> {
        let mapping_len = self.position_map.len();
        let remaining = self.size().wrapping_sub(1 + end - start);
        let drain = self.values.drain(start..=end).collect::<Vec<T>>();

        self.inverse_map.truncate(0);
        self.position_map.truncate(0);

        self.inverse_map.resize_with(mapping_len, || None);
        self.position_map.resize_with(mapping_len, || None);

        if remaining > 0 {
            Range {
                start: 0,
                end: remaining,
            }
                .for_each(|i| {
                    self.inverse_map[i] = Some(i);
                    self.position_map[i] = Some(i);
                });

            self.fix_heap_invariant();
        }

        drain
    }

    fn insert(&mut self, key_index: usize, value: T) {
        let size = self.size();
        if key_index == size {
            self.push(value);
        } else {
            self.position_map[size] = Some(size);
            self.inverse_map[size] = Some(size);
            self.values.insert(key_index, value);
            self.swim(size);
        }
    }

    fn increase(&mut self, key_index: usize, value: T) {
        self.key_exists_or_panic(key_index);
        if self.values[key_index] < value {
            self.values[key_index] = value;
            self.sink(self.node_index(key_index));
        }
    }

    #[inline]
    fn peek_min_key_index(&self) -> Option<usize> {
        self.inverse_map[0]
    }

    fn peek_min_value(&self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        Some(self.values[self.peek_min_key_index().unwrap()].clone())
    }

    fn poll_min_key_index(&mut self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }
        let min_key_index = self.peek_min_key_index();
        self.delete(min_key_index.unwrap());

        min_key_index
    }

    fn poll_min_value(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let min_value = self.peek_min_value();
        let min_key_index = self.peek_min_key_index();
        self.delete(min_key_index.unwrap());

        min_value
    }

    fn push(&mut self, value: T) {
        let size = self.size();

        if size >= self.position_map.len() {
            self.expand_mapping();
        };

        self.position_map[size] = Some(size);
        self.inverse_map[size] = Some(size);
        self.values.push(value);
        self.swim(size);
    }

    fn update(&mut self, key_index: usize, value: T) -> T {
        self.key_exists_or_panic(key_index);

        let i = self.node_index(key_index);
        let old_value = self.values[key_index].clone();

        self.values[key_index] = value;
        self.sink(i);
        self.swim(i);

        old_value
    }

    fn value_of(&self, key_index: usize) -> Option<T> {
        if (self.size() - 1) < key_index {
            return None;
        }
        Some(self.values[key_index].clone())
    }
}

impl<'a, T> MinIndexedPriorityQueue<'a, T>
where
    T: Clone + PartialOrd,
{
    #[inline]
    fn node_index(&self, i: usize) -> usize {
        self.position_map[i].unwrap()
    }

    #[inline]
    fn priority_sequenced_value(&self, i: usize) -> &T {
        &self.values[self.inverse_map[i].unwrap()]
    }

    pub fn left_child(&self, node_index: usize) -> Option<&T> {
        let i = 2 * node_index + 1;
        return if i < self.values.len() {
            Some(&self.values[self.inverse_map[i].unwrap()])
        } else {
            None
        };
    }

    pub fn right_child(&self, node_index: usize) -> Option<&T> {
        let i = 2 * node_index + 2;
        return if i < self.values.len() {
            Some(&self.values[self.inverse_map[i].unwrap()])
        } else {
            None
        };
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.values.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.values.iter_mut()
    }

    fn fix_heap_invariant(&mut self) {
        let mut edge_layer_range = Range {
            start: (self.inverse_map.len() / 2).wrapping_sub(1),
            end: self.size(),
        };

        if edge_layer_range.start > edge_layer_range.end {
            edge_layer_range.start = (self.size().next_power_of_two() / 2).wrapping_sub(1);
        }
        edge_layer_range.for_each(|i| self.swim(i));
    }

    fn expand_mapping(&mut self) {
        let pm_len = self.position_map.len();
        let extra_len = (pm_len + 1).next_power_of_two() - pm_len;

        let mut mapping_expansion = vec![None; extra_len];
        self.position_map.append(&mut mapping_expansion.clone());
        self.inverse_map.append(&mut mapping_expansion);
    }

    fn branches_count(&self) -> usize {
        self.size() - 1
    }

    fn key_exists_or_panic(&self, key_index: usize) {
        if !self.contains(key_index) {
            panic!("Index does not exist; received: {}", key_index);
        }
    }
}

#[cfg(test)]
mod min_indexed_pq_tests {
    use crate::{IndexedBinaryHeap, IndexedPriorityQueue, MinIndexedPriorityQueue};

    #[test]
    fn test_iter_max_and_min_with_integers() {
        let values = vec![
            Some(1),
            Some(0),
            Some(2),
            Some(4),
            Some(3),
            None,
            None,
            None,
        ];

        // Which values are max/min
        assert_eq!(values.iter().max(), Some(&Some(4)));
        assert_eq!(values.iter().min(), Some(&None));

        // Max/Min indexes
        assert_eq!(
            values
                .iter()
                .enumerate()
                .max_by_key(|(_, &v)| v)
                .map(|(i, _)| i),
            Some(3)
        );
        assert_eq!(
            values
                .iter()
                .enumerate()
                .min_by_key(|(_, &v)| v)
                .map(|(i, _)| i),
            Some(5)
        );
    }

    #[test]
    fn branches_count_should_return_correct_number_of_links_between_nodes() {
        let mut values = vec![9, 8, 8, 6, 1, 7, 2, 2, 2, 3, 4, 0];
        let ipq = MinIndexedPriorityQueue::from(&mut values);
        assert_eq!(ipq.branches_count(), 11);
        drop(ipq);
        drop(values);

        let mut values = vec![1, 2, 2, 2, 0];
        let ipq = MinIndexedPriorityQueue::from(&mut values);
        assert_eq!(ipq.branches_count(), 4);
        drop(ipq);
        drop(values);

        let mut values = vec![3, 4, 5, -1];
        let ipq = MinIndexedPriorityQueue::from(&mut values);
        assert_eq!(ipq.branches_count(), 3);
    }

    #[test]
    fn display_implementation_test() {
        let mut values = vec![3, 4, 5, -1];
        let ipq = MinIndexedPriorityQueue::from(&mut values);

        assert_eq!(
            format!("{}", ipq),
            "Minimum Priority Queue of 4 elements and 3 branches"
        )
    }

    #[test]
    fn min_ipq_should_successfully_create_a_binary_heap_from_pre_existent_vec() {
        let mut values: Vec<i32> = vec![9, 8, 7, 6, 5, 1, 2, 2, 2, 3, 4, 0];
        let v_len = values.len();
        let mut min_ipq = MinIndexedPriorityQueue::from(&mut values);

        assert_eq!(
            min_ipq.position_map,
            [
                Some(3),
                Some(8),
                Some(5),
                Some(7),
                Some(9),
                Some(11),
                Some(6),
                Some(2),
                Some(1),
                Some(4),
                Some(10),
                Some(0),
                None,
                None,
                None,
                None
            ]
        );
        assert_eq!(
            min_ipq.inverse_map,
            [
                Some(11),
                Some(8),
                Some(7),
                Some(0),
                Some(9),
                Some(2),
                Some(6),
                Some(3),
                Some(1),
                Some(4),
                Some(10),
                Some(5),
                None,
                None,
                None,
                None
            ]
        );

        min_ipq.insert(v_len, -100);

        assert_eq!(
            min_ipq.position_map,
            [
                Some(3),
                Some(8),
                Some(12),
                Some(7),
                Some(9),
                Some(11),
                Some(6),
                Some(5),
                Some(1),
                Some(4),
                Some(10),
                Some(2),
                Some(0),
                None,
                None,
                None
            ]
        );
        assert_eq!(
            min_ipq.inverse_map,
            [
                Some(12),
                Some(8),
                Some(11),
                Some(0),
                Some(9),
                Some(7),
                Some(6),
                Some(3),
                Some(1),
                Some(4),
                Some(10),
                Some(5),
                Some(2),
                None,
                None,
                None
            ]
        );
    }

    #[test]
    fn left_and_right_childs_should_return_option_even_on_last_layer() {
        let mut values = vec![9, 8, 8, 6, 1, 7, 2, 2, 2, 3, 4, 0];

        let ipq = MinIndexedPriorityQueue::from(&mut values);

        assert_eq!(ipq.left_child(4), Some(&3));
        assert_eq!(ipq.right_child(4), Some(&4));

        assert_eq!(ipq.left_child(5), Some(&7));
        assert_eq!(ipq.right_child(5), None);

        assert_eq!(ipq.left_child(7), None);
        assert_eq!(ipq.right_child(7), None);

        assert_eq!(ipq.left_child(12), None);
        assert_eq!(ipq.right_child(12), None);
    }

    #[test]
    fn poll_insert_peek_methods_should_run_without_breaking_data_structure() {
        let mut values = vec![1, 2, 2, 2, 0];
        let mut ipq = MinIndexedPriorityQueue::from(&mut values);
        assert_eq!(ipq.poll_min_value(), Some(0));
        assert_eq!(ipq.poll_min_value(), Some(1));
        assert_eq!(ipq.poll_min_value(), Some(2));

        ipq.insert(ipq.size(), -100);
        assert_eq!(ipq.peek_min_value(), Some(-100));
    }

    #[test]
    fn insert_should_expand_pq_mapping_if_key_index_is_in_correct_interval() {
        let mut values = vec![1, 2, 2, 2, 0];
        let mut ipq = MinIndexedPriorityQueue::from(&mut values);

        ipq.insert(ipq.size(), 3);
        ipq.insert(ipq.size(), 4);
        ipq.insert(ipq.size(), 5);
        ipq.insert(ipq.size(), 6);

        let values_len = ipq.size();
        let nones_len = ipq.inverse_map.len() - values_len;
        let nones = vec![None; nones_len];

        let mut pm = vec![
            Some(1),
            Some(4),
            Some(2),
            Some(3),
            Some(0),
            Some(5),
            Some(6),
            Some(7),
            Some(8),
        ];
        let mut im = vec![
            Some(4),
            Some(0),
            Some(2),
            Some(3),
            Some(1),
            Some(5),
            Some(6),
            Some(7),
            Some(8),
        ];
        pm.append(&mut nones.clone());
        im.append(&mut nones.clone());

        assert_eq!(values_len, 9);
        assert_eq!(ipq.inverse_map.len(), 16);
        assert_eq!(ipq.position_map.len(), 16);

        assert_eq!(ipq.values, &mut [1, 2, 2, 2, 0, 3, 4, 5, 6]);
        assert_eq!(ipq.inverse_map, im);
        assert_eq!(ipq.position_map, pm);
    }

    #[test]
    fn append_should_successfully_increase_ipq_with_extra_vector_within_mapping_bounds() {
        let mut values = vec![1, 2, 2, 2, 0];
        let mut ipq = MinIndexedPriorityQueue::from(&mut values);
        let mut extra_values = vec![3, 4, 5];

        ipq.append(&mut extra_values);

        assert_eq!(ipq.size(), 8);
        assert_eq!(ipq.inverse_map.len(), 8);
        assert_eq!(ipq.position_map.len(), 8);
    }

    #[test]
    fn append_should_successfully_increase_ipq_with_extra_vector_outside_mapping_bounds() {
        let mut values = vec![1, 2, 2, 2, 0];
        let mut ipq = MinIndexedPriorityQueue::from(&mut values);
        let mut extra_values = vec![3, 4, 5, -1];

        ipq.append(&mut extra_values);

        assert_eq!(ipq.size(), 9);
        assert_eq!(ipq.inverse_map.len(), 16);
        assert_eq!(ipq.position_map.len(), 16);
    }

    #[test]
    fn is_empty_should_correctly_function_with_a_empty_vector_generated_ipq() {
        let mut values: Vec<i32> = vec![];
        let ipq = MinIndexedPriorityQueue::from(&mut values);

        assert!(ipq.is_empty());
    }

    #[test]
    fn drain_should_successfully_remove_values_instances_from_within_a_interval() {
        let mut values: Vec<i32> = vec![9, 8, 7, 6, 5, 1, 2, 2, 2, 3, 4, 0];
        let mut ipq = MinIndexedPriorityQueue::from(&mut values);

        ipq.drain(5, 11);

        assert_eq!(ipq.size(), 5);
        assert_eq!(ipq.peek_min_value(), Some(5));
        assert_eq!(ipq.inverse_map.len(), 16);
        assert_eq!(ipq.position_map.len(), 16);
    }

    #[test]
    fn drain_should_successfully_empty_a_ipq() {
        let mut values = vec![1, 2, 2, 2, 0];
        let mut ipq = MinIndexedPriorityQueue::from(&mut values);

        ipq.drain(0, 4);

        assert!(ipq.is_empty());
    }

    #[test]
    fn decrease_should_successfully_manipulate_and_correct_heap() {
        let mut values: Vec<i32> = vec![9, 8, 0];
        let mut ipq = MinIndexedPriorityQueue::from(&mut values);

        ipq.decrease(0, -100);
        ipq.decrease(1, -2);

        assert_eq!(ipq.size(), 3);
        assert_eq!(ipq.peek_min_key_index(), Some(0));
        assert_eq!(ipq.poll_min_value(), Some(-100));
        assert_eq!(ipq.peek_min_key_index(), Some(0));
        assert_eq!(ipq.poll_min_value(), Some(-2));
        assert_eq!(ipq.poll_min_value(), Some(0));
    }

    #[test]
    fn increase_should_successfully_manipulate_and_correct_heap() {
        let mut values: Vec<i32> = vec![9, 8, 0];
        let mut ipq = MinIndexedPriorityQueue::from(&mut values);

        ipq.increase(0, 100);
        ipq.increase(1, 10);

        assert_eq!(ipq.size(), 3);
        assert_eq!(ipq.peek_min_key_index(), Some(2));
        assert_eq!(ipq.poll_min_value(), Some(0));
        assert_eq!(ipq.peek_min_key_index(), Some(1));
        assert_eq!(ipq.poll_min_value(), Some(10));
        assert_eq!(ipq.poll_min_value(), Some(100));
    }

    #[test]
    fn poll_should_be_able_to_empty_heap_with_no_problems() {
        let mut values: Vec<i32> = vec![9, 8, 0];
        let mut ipq = MinIndexedPriorityQueue::from(&mut values);

        assert_eq!(ipq.poll_min_value(), Some(0));
        assert_eq!(ipq.poll_min_value(), Some(8));
        assert_eq!(ipq.poll_min_value(), Some(9));
        assert!(ipq.is_empty());
    }

    #[test]
    fn poll_min_key_index_should_successfully_return_min_value_index_and_remove_it() {
        let mut values: Vec<i32> = vec![9, 8, 0];
        let mut ipq = MinIndexedPriorityQueue::from(&mut values);

        assert_eq!(ipq.poll_min_key_index(), Some(2));
        assert_eq!(ipq.poll_min_key_index(), Some(1));
        assert_eq!(ipq.poll_min_key_index(), Some(0));
    }

    #[test]
    fn value_of_should_return_values_by_index() {
        let mut values: Vec<i32> = vec![9, 8, 0];
        let ipq = MinIndexedPriorityQueue::from(&mut values);

        assert_eq!(ipq.value_of(1), Some(8));
        assert_eq!(ipq.value_of(0), Some(9));
        assert_eq!(ipq.value_of(2), Some(0));
    }

    #[test]
    fn sequential_update_and_polling_operations_should_be_executed_without_breaching_heap_invariance() {
        let mut values = vec![1, 2, 2, 2, 0];
        let mut ipq = MinIndexedPriorityQueue::from(&mut values);

        assert_eq!(ipq.peek_min_value(), Some(0));
        assert_eq!(ipq.update(1, -1), 2);
        assert_eq!(ipq.peek_min_value(), Some(-1));
        assert_eq!(ipq.update(3, -5), 2);
        assert_eq!(ipq.poll_min_value(), Some(-5));
        assert_eq!(ipq.update(1, 4), -1);
        assert_eq!(ipq.poll_min_value(), Some(0));
    }

    #[test]
    #[should_panic]
    fn drain_should_fail_with_invalid_start_index_delimiter() {
        let mut values = vec![1, 2, 2, 2, 0];
        let mut ipq = MinIndexedPriorityQueue::from(&mut values);

        ipq.drain(11, 4);
    }

    #[test]
    #[should_panic]
    fn drain_should_fail_with_invalid_end_index_delimiter() {
        let mut values = vec![1, 2, 2, 2, 0];
        let mut ipq = MinIndexedPriorityQueue::from(&mut values);

        ipq.drain(0, 20);
    }

    #[test]
    fn polling_empty_pq_should_return_none() {
        let mut values: Vec<u8> = Vec::new();
        let mut ipq = MinIndexedPriorityQueue::from(&mut values);

        assert_eq!(ipq.poll_min_value(), None);
    }

    #[test]
    fn peek_on_empty_pq_should_return_none() {
        let mut values: Vec<u8> = Vec::new();
        let ipq = MinIndexedPriorityQueue::from(&mut values);

        assert!(ipq.is_empty());
        assert_eq!(ipq.peek_min_value(), None);
    }

    #[test]
    fn invalid_key_index_should_return_none() {
        let mut values = vec![1, 2, 2, 2, 0];
        let ipq = MinIndexedPriorityQueue::from(&mut values);

        assert_eq!(ipq.value_of(5), None);
    }

    #[test]
    #[should_panic]
    fn invalid_key_index_should_panic_insert_at_value_method() {
        let mut values = vec![1, 2, 2, 2, 0];
        let mut ipq = MinIndexedPriorityQueue::from(&mut values);

        ipq.insert(ipq.size() + 1, -1);
    }

    #[test]
    #[should_panic]
    fn invalid_key_index_should_provide_invalid_inverse_map_as_key() {
        let mut values = vec![1, 2, 2, 2, 0];
        let ipq = MinIndexedPriorityQueue::from(&mut values);

        ipq.priority_sequenced_value(5);
    }
}
