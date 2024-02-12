fn is_prime(num: u32) -> bool {
    if num < 2 {
        return false;
    }
    for i in 2..=(num as f64).sqrt() as u32 {
        if num % i == 0 {
            return false;
        }
    }
    true
}
