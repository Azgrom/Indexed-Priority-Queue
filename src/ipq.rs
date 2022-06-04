pub trait IndexedPriorityQueue<T> {
    fn contains(&self, key_index: usize) -> bool;
    fn peek_min_key_index(&self) -> usize;
    fn peek_min_value(&self) -> T;
    fn value_of(&self, key_index: usize) -> T;
    fn insert(&mut self, key_index: usize, value: T);
    fn delete(&mut self, key_index: usize) -> T;
    fn increase(&mut self, key_index: usize, value: T);
    fn decrease(&mut self, key_index: usize, value: T);
    fn poll_min_key_index(&mut self) -> usize;
    fn poll_min_value(&mut self) -> T;
    fn update(&mut self, key_index: usize, value: T) -> T;
}

pub trait IndexedBinaryHeap {
    fn is_empty(&self) -> bool;
    fn less(&self, i: usize, j: usize) -> bool;
    fn size(&self) -> usize;
    fn swap(&mut self, i: usize, j: usize);
    fn swim(&mut self, i: usize);
    fn sink(&mut self, i: usize);
    fn min_child(&self, i: usize) -> Option<usize>;
}
