pub fn recursive_factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * recursive_factorial(n - 1)
    }
}

pub fn iterative_factorial(n: u64) -> u64 {
    let mut result = 1;
    for i in 2..=n {
        result *= i;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterative_factorial_0() {
        assert_eq!(iterative_factorial(0), 1);
    }

    #[test]
    fn test_iterative_factorial_1() {
        assert_eq!(iterative_factorial(1), 1);
    }

    #[test]
    fn test_factorial_0() {
        assert_eq!(recursive_factorial(0), 1);
    }

    #[test]
    fn test_factorial_1() {
        assert_eq!(recursive_factorial(1), 1);
    }

    #[test]
    fn test_factorial_2() {
        assert_eq!(recursive_factorial(2), 2);
        assert_eq!(recursive_factorial(3), 6);
        assert_eq!(recursive_factorial(4), 24);
        assert_eq!(recursive_factorial(5), 120);
    }
}
