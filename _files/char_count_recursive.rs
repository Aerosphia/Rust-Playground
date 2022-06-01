fn main() {
    let chars: usize = total_chars(&["hello", "world"], 0);
    println!("{}", chars);
}

fn total_chars(arr: &[&'static str], index: usize) -> usize {
    if index >= arr.len() {
        return 0;
    }

    let char_count: usize = arr[index].chars().count();
    
    return char_count + total_chars(arr, index + 1);
}
