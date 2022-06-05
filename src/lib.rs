use crate::ipq::{IndexedBinaryHeap, IndexedPriorityQueue};
use std::ops::Range;

pub mod ipq;

fn parent_node_index(node_index: usize) -> usize {
    return match node_index {
        0 => 0,
        n if n % 2 == 0 => (n / 2) - 1,
        _ => (node_index - 1) / 2,
    };
}

pub struct MinIndexedPriorityQueue<'a, T> {
    values: &'a mut Vec<T>,
    position_map: Vec<Option<usize>>,
    inverse_map: Vec<Option<usize>>,
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
        self.value(i) < self.value(j)
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
        let parent = |x| parent_node_index(x);
        let mut pi = parent(i);
        while self.less(i, pi) {
            self.swap(i, pi);
            i = pi;
            pi = parent(i);
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

    fn min_child(&self, mut i: usize) -> Option<usize> {
        let number_of_direct_childs_per_node = 2;
        let from = number_of_direct_childs_per_node * i + 1;
        let to = from + number_of_direct_childs_per_node;

        return if let true = self.size() <= to {
            None
        } else {
            let mut j = from;
            let mut index: Option<usize> = None;

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
}

impl<'a, T> IndexedPriorityQueue<T> for MinIndexedPriorityQueue<'a, T>
where
    T: Clone + PartialOrd,
{
    fn contains(&self, key_index: usize) -> bool {
        self.key_in_bounds_or_panic(key_index);

        self.position_map[key_index].is_some()
    }

    #[inline]
    fn peek_min_key_index(&self) -> usize {
        self.is_not_empty_or_panic();

        self.inverse(0)
    }

    fn peek_min_value(&self) -> T {
        self.values[self.peek_min_key_index()].clone()
    }

    fn value_of(&self, key_index: usize) -> T {
        self.key_exists_or_panic(key_index);
        self.values[key_index].clone()
    }

    fn insert(&mut self, key_index: usize, value: T) {
        self.key_implies_expanding_need(key_index);
        self.key_already_exists_panic(key_index);

        let size = self.size();
        self.position_map[key_index] = Some(size);
        self.inverse_map[size] = Some(key_index);
        if key_index < self.values.len() {
            self.values.insert(key_index, value);
        } else {
            self.values.push(value);
        }
        self.swim(size);
    }

    fn delete(&mut self, key_index: usize) -> T {
        self.key_exists_or_panic(key_index);

        let i = self.position(key_index);
        let size = self.size() - 1;

        if self.inverse(size) == self.position(size) {
            let value = self.values[key_index].clone();
            self.values.remove(key_index);
            self.position_map[size] = None;
            self.inverse_map[size] = None;

            self.sink(i);
            self.swim(i);

            return value;
        }

        self.swap(i, size);
        self.sink(i);
        self.swim(i);

        let value = self.values[key_index].clone();
        self.values.remove(key_index);
        self.position_map[key_index] = None;
        self.inverse_map[size] = None;

        value
    }

    fn increase(&mut self, key_index: usize, value: T) {
        self.key_exists_or_panic(key_index);
        if self.values[key_index] < value {
            self.values[key_index] = value;
            self.sink(self.position(key_index));
        }
    }

    fn decrease(&mut self, key_index: usize, value: T) {
        self.key_exists_or_panic(key_index);
        if value < self.values[key_index] {
            self.values[key_index] = value;
            self.swim(self.position(key_index))
        }
    }

    fn poll_min_key_index(&mut self) -> usize {
        let min_key_index = self.peek_min_key_index();
        self.delete(min_key_index);

        min_key_index
    }

    fn poll_min_value(&mut self) -> T {
        let min_value = self.peek_min_value();
        let min_key_index = self.peek_min_key_index();
        self.delete(min_key_index);

        min_value
    }

    fn update(&mut self, key_index: usize, value: T) -> T {
        self.key_exists_or_panic(key_index);

        let i = self.position(key_index);
        let old_value = self.values[key_index].clone();

        self.values[key_index] = value;
        self.sink(i);
        self.swim(i);

        old_value
    }
}

impl<'a, T> MinIndexedPriorityQueue<'a, T>
where
    T: Clone + PartialOrd,
{
    pub fn from_existent_vec(values: &'a mut Vec<T>) -> Self {
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

    #[inline]
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

    fn fix_heap_invariant(&mut self) {
        let edge_layer_range = Range {
            start: (self.inverse_map.len() / 2) - 1,
            end: self.values.len(),
        };
        edge_layer_range.for_each(|i| self.swim(i));
    }

    pub fn left_child(&self, node_index: usize) -> Option<&T> {
        let i = 2 * node_index + 1;
        return if i < self.values.len() {
            Some(&self.values[self.inverse(i)])
        } else {
            None
        };
    }

    pub fn right_child(&self, node_index: usize) -> Option<&T> {
        let i = 2 * node_index + 2;
        return if i < self.values.len() {
            Some(&self.values[self.inverse(i)])
        } else {
            None
        };
    }

    fn key_implies_expanding_need(&mut self, key_index: usize) {
        if key_index >= self.position_map.len() && key_index == self.values.len() {
            let pm_len = self.position_map.len();
            let extra_len = (pm_len + 1).next_power_of_two() - pm_len;

            let mut mapping_expansion = vec![None; extra_len];
            self.position_map.append(&mut mapping_expansion.clone());
            self.inverse_map.append(&mut mapping_expansion);
        }
    }

    fn is_not_empty_or_panic(&self) {
        if self.is_empty() {
            panic!("Priority queue underflow");
        }
    }

    fn key_already_exists_panic(&self, key_index: usize) {
        if self.contains(key_index) {
            panic!("Index already exists: received: {}", key_index);
        }
    }

    fn key_exists_or_panic(&self, key_index: usize) {
        if !self.contains(key_index) {
            panic!("Index does not exist; received: {}", key_index);
        }
    }

    fn key_in_bounds_or_panic(&self, key_index: usize) {
        if key_index >= self.position_map.len() {
            panic!(
                "Key index way off expanding capacity/necessity; received: {}",
                key_index
            );
        }
    }
}

#[cfg(test)]
mod min_indexed_pq_tests {
    use crate::{IndexedBinaryHeap, IndexedPriorityQueue, MinIndexedPriorityQueue};

    #[test]
    fn min_ipq_should_successfully_create_a_binary_heap_from_pre_existent_vec() {
        let mut values: Vec<i32> = vec![9, 8, 7, 6, 5, 1, 2, 2, 2, 3, 4, 0];
        let v_len = values.len();
        let mut min_ipq = MinIndexedPriorityQueue::from_existent_vec(&mut values);

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

        let ipq = MinIndexedPriorityQueue::from_existent_vec(&mut values);

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
        let mut ipq = MinIndexedPriorityQueue::from_existent_vec(&mut values);
        assert_eq!(ipq.poll_min_value(), 0);
        assert_eq!(ipq.poll_min_value(), 1);
        assert_eq!(ipq.poll_min_value(), 2);

        ipq.insert(ipq.size(), -100);
        assert_eq!(ipq.peek_min_value(), -100);
    }

    #[test]
    fn insert_should_expand_pq_mapping_if_key_index_is_in_correct_interval() {
        let mut values = vec![1, 2, 2, 2, 0];
        let mut ipq = MinIndexedPriorityQueue::from_existent_vec(&mut values);

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
    #[should_panic]
    fn invalid_key_index_should_panic_insert() {
        let mut values = vec![1, 2, 2, 2, 0];
        let mut ipq = MinIndexedPriorityQueue::from_existent_vec(&mut values);

        ipq.insert(ipq.size() + 1, -1);
    }

    #[test]
    #[should_panic]
    fn invalid_key_index_should_provide_invalid_inverse_map_as_key() {
        let mut values = vec![1, 2, 2, 2, 0];
        let mut ipq = MinIndexedPriorityQueue::from_existent_vec(&mut values);

        ipq.value(5);
    }

    #[test]
    #[should_panic]
    fn invalid_key_index_should_trigger_exist_key_panic() {
        let mut values = vec![1, 2, 2, 2, 0];
        let mut ipq = MinIndexedPriorityQueue::from_existent_vec(&mut values);

        ipq.value_of(5);
    }
}
