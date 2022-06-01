pub fn max_priority<T>(value_to_insert: &Option<T>, inserted_parent_node_value: &Option<T>) -> bool
where
    T: PartialOrd,
{
    Some(value_to_insert) >= Some(inserted_parent_node_value)
}

pub fn last_some<T>(vals: &Vec<Option<T>>) -> usize {
    vals.iter().rposition(|x| x.is_some()).unwrap()
}

pub fn number_of_layers<T>(vals: &Vec<Option<T>>) -> usize {
    let mut number_of_layers = 1;
    let mut n = vals.len().next_power_of_two();

    while n != 2 {
        n /= 2;
        number_of_layers += 1;
    }

    number_of_layers
}

pub fn parent_node_index(node_index: usize) -> usize {
    return match node_index {
        0 => 0,
        n if n % 2 == 0 => (n / 2) - 1,
        _ => (node_index - 1) / 2,
    };
}

pub fn left_child<T>(vals: &Vec<Option<T>>, node_index: usize) -> Option<&T> {
    let i = 2 * node_index + 1;
    return if i < vals.len() {
        vals[i].as_ref()
    } else {
        None
    };
}

pub fn right_child<T>(vals: &Vec<Option<T>>, node_index: usize) -> Option<&T> {
    let i = 2 * node_index + 2;
    return if i < vals.len() {
        vals[i].as_ref()
    } else {
        None
    };
}

pub fn set_vals<T>(values: &Vec<T>) -> Vec<Option<T>>
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
    use super::{parent_node_index, set_vals};

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
