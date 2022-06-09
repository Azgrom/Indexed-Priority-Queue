# A Indexed Priority Queue Library

Based on [William Fiset's](https://github.com/williamfiset/algorithms) algorithm, this priority queue accepts any type
that supports the [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html)
and [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html) traits. In practical words, for who might
use this library as [FFI](https://en.wikipedia.org/wiki/Foreign_function_interface), it will work with any type of data
that accepts being compared by the `<`, `<=`, `>` and `>=` operators.

## Methods

| Name Signature       | Parameter                      | Utility                                                      |
| -------------------- | ------------------------------ | ------------------------------------------------------------ |
| `append`             | extra_values: `&mut Vec<T>`    | Adds a vector o `values` to an already existing `IPQ`        |
| `contains`           | key_index: `usize`             | Returns a boolean stating if there is a mapping on a given index |
| `decrease`           | key_index: `usize`             | Updates a known `value` to a 'lesser' version of itself and fixes heap invariance, if necessary |
| `delete`             | key_index: `usize`             | Deletes an index specified value and updates heap accordingly |
| `drain`              | start: `usize`, end: `usize`   | [Removes](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.drain) from `values`, returns a vector contained in a specified index range, fixes heap invariance and maintains mapping allocation |
| `insert`             | key_index: `usize`, value: `T` | Adds a `value` of type `T` and updates heap accordingly      |
| `increase`           | key_index: `usize`, value: `T` | Updates a known `value` to a 'greater' version of itself and fixes heap invariance, if necessary |
| `peek_min_key_index` |                                | Returns current minimum `value` index                        |
| `peek_min_value`     |                                | Returns current minimum `value`                              |
| `poll_min_key_index` |                                | Remove and returns current minimum `value` index and updates heap accordingly |
| `poll_min_value`     |                                | Remove, returns current minimum `value` and updates heap accordingly |
| `update`             | key_index: `usize`, value: `T` | Updates any given `value` to a `new provided value` and fixes heap invariance, if necessary |
| `value_of`           | key_index: `usize`             | Returns `value` without consuming it                         |
