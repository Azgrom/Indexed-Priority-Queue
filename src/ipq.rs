pub trait IndexedPriorityQueue<T> {
    fn append(&mut self, extra_values: &mut Vec<T>);
    fn contains(&self, key_index: usize) -> bool;
    fn decrease(&mut self, key_index: usize, value: T);
    fn delete(&mut self, key_index: usize) -> Option<T>;
    fn drain(&mut self, start: usize, end: usize) -> Vec<T>;
    fn insert(&mut self, key_index: usize, value: T);
    fn increase(&mut self, key_index: usize, value: T);
    fn peek_min_key_index(&self) -> Option<usize>;
    fn peek_min_value(&self) -> Option<T>;
    fn poll_min_key_index(&mut self) -> Option<usize>;
    fn poll_min_value(&mut self) -> Option<T>;
    fn push(&mut self, value: T);
    fn update(&mut self, key_index: usize, value: T) -> T;
    fn value_of(&self, key_index: usize) -> Option<T>;
}

pub trait IndexedBinaryHeap {
    fn is_empty(&self) -> bool;
    fn less(&self, i: usize, j: usize) -> bool;
    fn min_child(&self, i: usize) -> Option<usize>;
    fn size(&self) -> usize;
    fn swap(&mut self, i: usize, j: usize);
    fn swim(&mut self, i: usize);
    fn sink(&mut self, i: usize);
}
