fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    for (i, &num) in arr.iter().enumerate() {
        if num == target {
            return Some(i);
        } else if num > target {
            break;
        }
    }
    None
}
