
fn helper(a: i64, b: i64, n: i64) -> i64 {
    if n > 0 {
        helper(b, a + b, n - 1)
    } else {
        a
    }
}

/// Generate the nth fibonacci number with tail recursion.
pub fn fib(n: i64) -> i64 {
    helper(0, 1, n)
}

/// Generate the nthe fibonacci number without any tail recursion.
pub fn fib_no_tr(n: i64) -> i64 {
    if n < 2 {
        return n;
    }
    fib_no_tr(n - 1) + fib_no_tr(n - 2)
}

#[cfg(test)]
mod tests {

    use crate::fib::{fib, fib_no_tr};

    #[test]
    fn fib0() {
        assert_eq!(0, fib(0));
        assert_eq!(0, fib_no_tr(0));
    }

    #[test]
    fn fib1() {
        assert_eq!(1, fib(1));
        assert_eq!(1, fib_no_tr(1));
    }

    #[test]
    fn fib10() {
        assert_eq!(55, fib(10));
        assert_eq!(55, fib_no_tr(10));
    }

    #[test]
    fn fib250() {
        assert_eq!(12586269025, fib(50));
    }
}
