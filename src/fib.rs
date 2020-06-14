use std::collections::HashMap;


fn fib_m(m: &mut HashMap<i64, i64>, n: i64) -> i64 {
    if m.contains_key(&n) {
        *m.get(&n).unwrap()
    } else {
        let res = fib_m(m, n - 1) + fib_m(m, n - 2);
        m.insert(n, res);
        res
    }
}

/// Generate the nth fibonacci number with memoization.
pub fn fib_mem(n: i64) -> i64 {
    let mut m = HashMap::new();
    m.insert(0, 0);
    m.insert(1, 1);
    fib_m(&mut m, n)
}

/// Generate the nth fibonacci number with iteration.
pub fn fib_iter(n: i64) -> i64 {
    if n == 0 {
        return n;
    }
    let mut last = 0;
    let mut next = 1;
    for _ in 1..n {
        let mut tmp = last;
        last = next;
        next = tmp + next
    }
    next
}

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

    use crate::fib::*;

    #[test]
    fn fib0() {
        assert_eq!(0, fib(0));
        assert_eq!(0, fib_no_tr(0));
        assert_eq!(0, fib_mem(0));
        assert_eq!(0, fib_iter(0));
    }

    #[test]
    fn fib1() {
        assert_eq!(1, fib(1));
        assert_eq!(1, fib_no_tr(1));
        assert_eq!(1, fib_mem(1));
        assert_eq!(1, fib_iter(1));
    }

    #[test]
    fn fib10() {
        assert_eq!(55, fib(10));
        assert_eq!(55, fib_no_tr(10));
        assert_eq!(55, fib_mem(10));
        assert_eq!(55, fib_iter(10));
    }

    #[test]
    fn fib250() {
        assert_eq!(12586269025, fib(50));
        assert_eq!(12586269025, fib_mem(50));
        assert_eq!(12586269025, fib_iter(50));
    }
}
