
fn helper(a: i64, b: i64, n: i64) -> i64 {
    if n > 0 {
        helper(b, a + b, n - 1)
    } else {
        a
    }
}

pub fn fib(n: i64) -> i64 {
    helper(0, 1, n)
}

#[cfg(test)]
mod tests {

    use crate::fib;

    #[test]
    fn fib0() {
        assert_eq!(0, fib(0));
    }

    #[test]
    fn fib1() {
        assert_eq!(1, fib(1));
    }

    #[test]
    fn fib10() {
        assert_eq!(55, fib(10));
    }

    #[test]
    fn fib250() {
        assert_eq!(12586269025, fib(50));
    }
}
