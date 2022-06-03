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
