// Fibonacci series
// For n = 0, return 0
// for n = 1, return 1
// for n > 1, return fibonacci(n-1) + fibonacci(n-2)

pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_0() {
        assert_eq!(fibonacci(0), 0);
    }
    #[test]
    fn test_fibonacci_1() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn test_fibonacci_2() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn test_fibonacci_3() {
        assert_eq!(fibonacci(3), 2);
    }

    #[test]
    fn test_fibonacci_4() {
        assert_eq!(fibonacci(4), 3);
    }

    #[test]
    fn test_fibonacci_5() {
        assert_eq!(fibonacci(5), 5);
    }
}
