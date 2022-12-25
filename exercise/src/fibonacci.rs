pub fn fib_match(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        2 => 2,
        _ => fib_match(n -2) + fib_match(n-1),
    }
}

#[cfg(test)]
mod tests_fibonacci {
    use super::*;

    #[test]
    fn test_fib_match() {
        assert_eq!(89, fib_match(10));
    }
}