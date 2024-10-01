// returns n! = n * factorial(n-1)

// n * (n-1) * (n-2) * ... (2) * 1

// for n = 0, n! = 1
// for n = 1, n! = 1
// for n = 2, n! = 2
// for n = 5 n! =5 * 4 * 3 * 2 * 1 = 120
//
// 1. Red - write a failing test
// 2. Green - write the minimum amount of code to make the test pass
// 3. Refactor - refactor the code to make it better

pub fn fact(n: u64) -> u64 {
    if n <= 1 {
        return 1;
    }
    n * fact(n - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fact_0_is_1() {
        assert_eq!(fact(0), 1);
    }

    #[test]
    fn test_fact_1_is_1() {
        assert_eq!(fact(1), 1);
    }

    #[test]
    fn test_fact_2_is_2() {
        // 2! = 1 * 2 = 2
        assert_eq!(fact(2), 2);
    }

    #[test]
    fn test_fact_3_5() {
        assert_eq!(fact(3), 6);
        assert_eq!(fact(4), 24);
        assert_eq!(fact(5), 120);
    }
}
