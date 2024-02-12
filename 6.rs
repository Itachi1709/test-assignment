fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let first_str = strings[0].chars().collect::<Vec<char>>();
    let mut prefix = String::new();

    for (i, c) in first_str.iter().enumerate() {
        if strings.iter().all(|s| s.chars().nth(i) == Some(*c)) {
            prefix.push(*c);
        } else {
            break;
        }
    }

    prefix
}
