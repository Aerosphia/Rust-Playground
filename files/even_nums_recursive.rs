fn main() {
    let evens: Vec<i32> = even_nums(vec![2, 1, 4, 3, 7, 6], 0);
    println!("{:?}", evens);
}

fn even_nums(mut vector: Vec<i32>, index: usize) -> Vec<i32> {
    if index >= vector.len() {
        return vector;
    }

    let is_even = vector[index] % 2 == 0;

    if !is_even {
        vector.remove(index);
    }

    return even_nums(vector, if is_even { index + 1 } else { index });
}
