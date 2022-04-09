// An algorithm of O(N) efficiency which returns an intersection between two arrays.
// i.e [1, 2, 3, 4, 5] and [2, 4, 6] have an intersection of [2, 6] (same elements)

use std::collections::HashMap;

fn main() {
  let inter = intersection(&[1, 2, 3, 4, 5], &[2, 4, 6]);
  println!("{:?}", inter);
}

fn intersection(arr: &[i32], arr2: &[i32]) -> Vec<i32> {
  let mut first_arr_vals: HashMap<i32, bool> = HashMap::new();
  let mut identical_vals: Vec<i32> = Vec::new();

  for i in arr.iter() {
    first_arr_vals.insert(*i, true);
  }

  for i in arr2.iter() {
    if first_arr_vals.contains_key(i) {
      identical_vals.push(*i);
    }
  }

  identical_vals
}
